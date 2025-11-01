# V4 VM への機能追加リクエスト: REPL サポート強化

## 背景

V4-repl の実装を通じて、REPL（対話型シェル）に必要な機能が不足していることが判明しました。
現在の回避策として VM 全体をリセットしていますが、これによりスタックもクリアされてしまい、ユーザー体験が低下しています。

### 現在の問題

**問題1: スタックの保存・復元ができない**
```c
// 現状: 読み取り専用の API しかない
int depth = vm_ds_depth_public(vm);
v4_i32 val = vm_ds_peek_public(vm, 0);

// できないこと:
// - スタックに値を直接 push
// - スタックから値を直接 pop
// - スタックの内容を保存・復元
```

**問題2: ワード辞書だけをリセットできない**
```c
// 現状: vm_reset() はスタックと辞書をすべてクリア
vm_reset(vm);  // スタックも消えてしまう
```

### REPL での具体的な使用シーン

```forth
v4> 1 2 +
 ok [1]: 3

v4> 10 20 +
 ok [2]: 3 30        # スタックが保持される（良い）

v4> : SQUARE DUP * ; 5 SQUARE
 ok [1]: 25          # スタックがクリアされた（悪い）
                     # 理由: ワードインデックスをリセットするため VM 全体をリセット
```

**理想の動作**:
```forth
v4> 1 2 +
 ok [1]: 3

v4> 10 20 +
 ok [2]: 3 30

v4> : SQUARE DUP * ; 5 SQUARE
 ok [3]: 3 30 25     # スタックが保持されたまま！
```

---

## 機能リクエスト1: スタック操作 API

### 提案する API

```c
// ========================================
// スタック直接操作（基本）
// ========================================

/**
 * @brief データスタックに値をプッシュする
 * @param vm VM インスタンス
 * @param value プッシュする値
 * @return 0 on success, negative on error (スタックオーバーフロー等)
 */
v4_err vm_ds_push(struct Vm* vm, v4_i32 value);

/**
 * @brief データスタックから値をポップする
 * @param vm VM インスタンス
 * @param out_value ポップした値の格納先（NULL 可）
 * @return 0 on success, negative on error (スタックアンダーフロー等)
 */
v4_err vm_ds_pop(struct Vm* vm, v4_i32* out_value);

/**
 * @brief データスタックをクリアする
 * @param vm VM インスタンス
 */
void vm_ds_clear(struct Vm* vm);

// ========================================
// スタックスナップショット（高レベル）
// ========================================

/**
 * @brief スタック内容のスナップショットを作成
 *
 * スタックの現在の内容をコピーして保存します。
 * 使用後は vm_ds_snapshot_free() で解放する必要があります。
 *
 * @param vm VM インスタンス
 * @return スナップショット（失敗時は NULL）
 */
struct VmStackSnapshot {
  v4_i32* data;  // スタックデータ（動的確保）
  int depth;     // スタック深度
};

struct VmStackSnapshot* vm_ds_snapshot(struct Vm* vm);

/**
 * @brief スナップショットからスタックを復元
 *
 * 現在のスタックをクリアし、スナップショットの内容で置き換えます。
 *
 * @param vm VM インスタンス
 * @param snapshot 復元元のスナップショット
 * @return 0 on success, negative on error
 */
v4_err vm_ds_restore(struct Vm* vm, const struct VmStackSnapshot* snapshot);

/**
 * @brief スナップショットを解放
 * @param snapshot 解放するスナップショット（NULL 可）
 */
void vm_ds_snapshot_free(struct VmStackSnapshot* snapshot);
```

### 使用例: REPL での活用

```c
// REPL の eval_line() での使用例
int Repl::eval_line(const char* line) {
  // コンパイル
  V4FrontBuf buf;
  v4front_compile(line, &buf, errmsg, sizeof(errmsg));

  // ワード定義がある場合、スタックを保存して VM をリセット
  struct VmStackSnapshot* snapshot = nullptr;
  if (buf.word_count > 0) {
    snapshot = vm_ds_snapshot(vm_);  // スタック保存
    vm_reset(vm_);                   // VM 全体リセット
    vm_ds_restore(vm_, snapshot);    // スタック復元
    vm_ds_snapshot_free(snapshot);
  }

  // ワード登録と実行...

  return 0;
}
```

---

## 機能リクエスト2: 選択的リセット API

### 提案する API

```c
/**
 * @brief VM の全状態をリセット（既存）
 *
 * スタック、ワード辞書、メモリをすべて初期状態に戻します。
 *
 * @param vm VM インスタンス
 */
void vm_reset(struct Vm* vm);

/**
 * @brief ワード辞書のみをリセット（新規）
 *
 * 登録されたすべてのワードをクリアします。
 * データスタック、リターンスタック、メモリは保持されます。
 *
 * @param vm VM インスタンス
 */
void vm_reset_dictionary(struct Vm* vm);

/**
 * @brief スタックのみをリセット（新規）
 *
 * データスタックとリターンスタックをクリアします。
 * ワード辞書とメモリは保持されます。
 *
 * @param vm VM インスタンス
 */
void vm_reset_stacks(struct Vm* vm);

/**
 * @brief メモリのみをリセット（新規、オプション）
 *
 * VM の RAM を 0 クリアします。
 * スタックとワード辞書は保持されます。
 *
 * @param vm VM インスタンス
 */
void vm_reset_memory(struct Vm* vm);
```

### 使用例: REPL での活用

```c
// より簡潔な実装
int Repl::eval_line(const char* line) {
  V4FrontBuf buf;
  v4front_compile(line, &buf, errmsg, sizeof(errmsg));

  // ワード定義がある場合、辞書のみリセット（スタックは保持！）
  if (buf.word_count > 0) {
    vm_reset_dictionary(vm_);  // これだけ！
  }

  // ワード登録と実行...

  return 0;
}
```

---

## 実装上の考慮点

### 1. エラーハンドリング

```c
// スタックオーバーフロー
v4_err err = vm_ds_push(vm, 42);
if (err == -14) {  // V4_ERR_StackOverflow
  fprintf(stderr, "Stack overflow\n");
}

// スタックアンダーフロー
v4_err err = vm_ds_pop(vm, &value);
if (err == -15) {  // V4_ERR_StackUnderflow
  fprintf(stderr, "Stack underflow\n");
}
```

### 2. スレッドセーフティ

現在の V4 API と同様、これらの API もスレッドセーフではないことを想定しています。
複数スレッドから同時にアクセスする場合は、呼び出し側で排他制御が必要です。

### 3. パフォーマンス

- `vm_ds_push/pop`: 内部スタックポインタの操作のみ（高速）
- `vm_ds_snapshot`: メモリコピーが発生（深度に比例）
- `vm_reset_dictionary`: ワード辞書のクリアのみ（スタック/メモリ触らない）

---

## 代替案

もし上記の API が大きすぎる場合、最小限の実装として以下を提案します：

### 最小案: 選択的リセットのみ

```c
// 最優先
void vm_reset_dictionary(struct Vm* vm);

// あれば便利（優先度低）
void vm_reset_stacks(struct Vm* vm);
```

この場合、REPL 側で以下のような回避策を継続します：

```c
// スタック保存（手動実装）
int depth = vm_ds_depth_public(vm_);
v4_i32 stack[256];
for (int i = 0; i < depth; i++) {
  stack[i] = vm_ds_peek_public(vm_, depth - 1 - i);
}

// 辞書リセット
vm_reset_dictionary(vm_);

// スタック復元（バイトコード生成経由）
for (int i = 0; i < depth; i++) {
  // LIT 命令を実行してスタックに値を戻す
  uint8_t code[] = {0x00, /*value bytes*/, 0x51};
  // ... vm_register_word & vm_exec ...
}
```

ただし、この方法は：
- 煩雑でエラーが起きやすい
- 一時的にワードスロットを消費する
- パフォーマンスが悪い

ため、専用 API があることが望ましいです。

---

## テストケース案

```c
TEST_CASE("Stack manipulation API") {
  Vm* vm = create_test_vm();

  // Push and peek
  REQUIRE(vm_ds_push(vm, 10) == 0);
  REQUIRE(vm_ds_push(vm, 20) == 0);
  REQUIRE(vm_ds_depth_public(vm) == 2);
  REQUIRE(vm_ds_peek_public(vm, 0) == 20);

  // Pop
  v4_i32 val;
  REQUIRE(vm_ds_pop(vm, &val) == 0);
  REQUIRE(val == 20);
  REQUIRE(vm_ds_depth_public(vm) == 1);

  // Clear
  vm_ds_clear(vm);
  REQUIRE(vm_ds_depth_public(vm) == 0);

  vm_destroy(vm);
}

TEST_CASE("Stack snapshot and restore") {
  Vm* vm = create_test_vm();

  vm_ds_push(vm, 10);
  vm_ds_push(vm, 20);
  vm_ds_push(vm, 30);

  // Snapshot
  VmStackSnapshot* snap = vm_ds_snapshot(vm);
  REQUIRE(snap != nullptr);
  REQUIRE(snap->depth == 3);

  // Modify stack
  vm_ds_clear(vm);
  vm_ds_push(vm, 999);
  REQUIRE(vm_ds_depth_public(vm) == 1);

  // Restore
  REQUIRE(vm_ds_restore(vm, snap) == 0);
  REQUIRE(vm_ds_depth_public(vm) == 3);
  REQUIRE(vm_ds_peek_public(vm, 0) == 30);
  REQUIRE(vm_ds_peek_public(vm, 1) == 20);
  REQUIRE(vm_ds_peek_public(vm, 2) == 10);

  vm_ds_snapshot_free(snap);
  vm_destroy(vm);
}

TEST_CASE("Selective reset") {
  Vm* vm = create_test_vm();

  // Setup: register word and push values
  vm_register_word(vm, "test", bytecode, len);
  vm_ds_push(vm, 42);
  vm_ds_push(vm, 100);

  // Reset dictionary only
  vm_reset_dictionary(vm);

  // Stack should be preserved
  REQUIRE(vm_ds_depth_public(vm) == 2);
  REQUIRE(vm_ds_peek_public(vm, 0) == 100);

  // Words should be cleared
  REQUIRE(vm_find_word(vm, "test") < 0);  // not found

  vm_destroy(vm);
}
```

---

## まとめ

### 優先度1: 選択的リセット（必須）
- `vm_reset_dictionary()` - 辞書のみリセット

### 優先度2: スタック操作（強く推奨）
- `vm_ds_push()` / `vm_ds_pop()` / `vm_ds_clear()` - 基本操作
- `vm_ds_snapshot()` / `vm_ds_restore()` / `vm_ds_snapshot_free()` - 保存・復元

### 優先度3: その他（あると便利）
- `vm_reset_stacks()` - スタックのみリセット
- `vm_reset_memory()` - メモリのみリセット

これらの API により、REPL のユーザー体験が大幅に向上し、V4 VM の汎用性も高まります。

---

## 参考情報

- **V4-repl リポジトリ**: `/home/kirisaki/devel/V4-repl`
- **既存の実装**: `src/repl.cpp` の `eval_line()` メソッド
- **現在の回避策**: ワード定義時に `vm_reset()` を呼び、スタックがクリアされる
- **V4 API ヘッダ**: `include/v4/vm_api.h`
