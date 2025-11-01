# V4-front への機能追加リクエスト: REPL サポート強化

## 背景

V4-repl の実装を通じて、V4-front のステートレスな設計が REPL（対話型シェル）にとって大きな制約となることが判明しました。
現在は各行ごとに完全に独立してコンパイルされるため、前の行で定義したワードを次の行で使うことができません。

### 現在の問題

**問題1: ワード定義が次の行で使えない**
```forth
v4> : SQUARE DUP * ;
 ok

v4> 5 SQUARE
Error [-1]: unknown token  # SQUARE を知らない！
```

**問題2: エラーメッセージが不親切**
```
Error [-1]: unknown token

どのトークンが問題なのか？
どの位置でエラーが起きたのか？
→ 分からない
```

### 根本原因

V4-front は完全にステートレスです：
```c
// 毎回まっさらな状態からコンパイル
v4front_compile(": SQUARE DUP * ;", &buf, err, sizeof(err));  // OK

// 次の呼び出しは前の定義を覚えていない
v4front_compile("5 SQUARE", &buf, err, sizeof(err));  // Error: SQUARE が未定義
```

ワードインデックスも常に 0 から始まります：
```c
// Line 1: : SQUARE DUP * ;
// → SQUARE に CALL 0 を生成

// Line 2: : DOUBLE DUP + ;
// → DOUBLE にも CALL 0 を生成（衝突！）
```

### REPL での回避策（現状）

```c
// ワード定義がある行では VM をリセット
if (buf.word_count > 0) {
  vm_reset(vm_);  // 辞書をクリア
}
// → これによりスタックもクリアされてしまう（UX 低下）
```

**問題点**:
- ワード定義を含む行でスタックがクリアされる
- 定義したワードを次の行で使えない
- インクリメンタルな開発ができない

---

## 機能リクエスト1: ステートフルコンパイラ

### 提案する API

```c
// ========================================
// コンパイラコンテキスト
// ========================================

/**
 * @brief コンパイラコンテキスト（不透明型）
 *
 * 過去にコンパイルされたワード定義を記憶し、
 * インクリメンタルなコンパイルを可能にします。
 */
typedef struct V4FrontContext V4FrontContext;

/**
 * @brief コンパイラコンテキストを作成
 *
 * @return 新しいコンテキスト（失敗時は NULL）
 */
V4FrontContext* v4front_context_create(void);

/**
 * @brief コンパイラコンテキストを破棄
 *
 * @param ctx 破棄するコンテキスト（NULL 可）
 */
void v4front_context_destroy(V4FrontContext* ctx);

// ========================================
// ワード登録（VM と同期）
// ========================================

/**
 * @brief ワード定義をコンテキストに登録
 *
 * VM に登録されたワードをコンパイラに教えます。
 * 以降のコンパイルで、このワードを参照できるようになります。
 *
 * @param ctx コンパイラコンテキスト
 * @param name ワード名（NULL 不可）
 * @param vm_word_idx VM 内でのワードインデックス
 * @return 0 on success, negative on error
 */
v4front_err v4front_context_register_word(
  V4FrontContext* ctx,
  const char* name,
  int vm_word_idx
);

/**
 * @brief 登録されたワードをすべて削除
 *
 * VM がリセットされた際に呼び出します。
 *
 * @param ctx コンパイラコンテキスト
 */
void v4front_context_reset(V4FrontContext* ctx);

// ========================================
// コンテキスト付きコンパイル
// ========================================

/**
 * @brief コンテキストを使ってコンパイル
 *
 * 過去に登録されたワードを参照できます。
 * 従来の v4front_compile() と同じインターフェース。
 *
 * @param ctx コンパイラコンテキスト
 * @param source ソースコード
 * @param out_buf 出力バッファ
 * @param err エラーメッセージバッファ（NULL 可）
 * @param err_cap エラーメッセージバッファサイズ
 * @return 0 on success, negative on error
 */
v4front_err v4front_compile_with_context(
  V4FrontContext* ctx,
  const char* source,
  V4FrontBuf* out_buf,
  char* err,
  size_t err_cap
);

// ========================================
// 便利機能（オプション）
// ========================================

/**
 * @brief 登録されているワード数を取得
 *
 * @param ctx コンパイラコンテキスト
 * @return ワード数
 */
int v4front_context_get_word_count(const V4FrontContext* ctx);

/**
 * @brief 登録されているワード名を取得
 *
 * @param ctx コンパイラコンテキスト
 * @param idx インデックス（0-based）
 * @return ワード名（見つからない場合は NULL）
 */
const char* v4front_context_get_word_name(const V4FrontContext* ctx, int idx);

/**
 * @brief ワード名から VM インデックスを検索
 *
 * @param ctx コンパイラコンテキスト
 * @param name ワード名
 * @return VM ワードインデックス（見つからない場合は負の値）
 */
int v4front_context_find_word(const V4FrontContext* ctx, const char* name);
```

### 使用例: REPL での活用

```c
class Repl {
private:
  Vm* vm_;
  V4FrontContext* compiler_ctx_;  // 追加

public:
  Repl() {
    vm_ = vm_create(&cfg);
    compiler_ctx_ = v4front_context_create();  // 初期化
  }

  ~Repl() {
    v4front_context_destroy(compiler_ctx_);
    vm_destroy(vm_);
  }

  int eval_line(const char* line) {
    V4FrontBuf buf;
    char errmsg[256];

    // コンテキスト付きコンパイル
    v4front_err err = v4front_compile_with_context(
      compiler_ctx_,
      line,
      &buf,
      errmsg,
      sizeof(errmsg)
    );

    if (err != 0) {
      print_error(errmsg, err);
      return -1;
    }

    // ワード定義を VM とコンパイラに登録
    for (int i = 0; i < buf.word_count; ++i) {
      V4FrontWord* word = &buf.words[i];

      // VM に登録
      int wid = vm_register_word(vm_, word->name, word->code, word->code_len);
      if (wid < 0) {
        return -1;
      }

      // コンパイラコンテキストにも登録
      v4front_context_register_word(compiler_ctx_, word->name, wid);
    }

    // main code の実行...

    v4front_free(&buf);
    return 0;
  }
};
```

### 動作イメージ

```forth
v4> : SQUARE DUP * ;
 ok

# 内部動作:
# 1. v4front_compile_with_context(ctx, ": SQUARE DUP * ;", ...)
# 2. VM に SQUARE を登録 → wid = 0
# 3. v4front_context_register_word(ctx, "SQUARE", 0)

v4> 5 SQUARE
 ok [1]: 25

# 内部動作:
# 1. v4front_compile_with_context(ctx, "5 SQUARE", ...)
#    → ctx を見て SQUARE が wid=0 と知っている
#    → CALL 0 を生成 ✅
# 2. 実行成功！

v4> : DOUBLE DUP + ;
 ok

# 内部動作:
# 1. v4front_compile_with_context(ctx, ": DOUBLE DUP + ;", ...)
# 2. VM に DOUBLE を登録 → wid = 1
# 3. v4front_context_register_word(ctx, "DOUBLE", 1)

v4> 3 SQUARE DOUBLE
 ok [1]: 18

# 内部動作:
# 1. ctx が SQUARE=0, DOUBLE=1 を記憶
# 2. CALL 0, CALL 1 を生成 ✅
```

---

## 機能リクエスト2: エラー位置情報

### 提案する API

```c
// ========================================
// 拡張エラー情報
// ========================================

/**
 * @brief 詳細なエラー情報
 *
 * エラーの発生位置とコンテキストを含みます。
 */
typedef struct V4FrontError {
  v4front_err_t code;        // エラーコード（従来通り）
  char message[256];         // エラーメッセージ
  int position;              // エラー発生位置（バイトオフセット）
  int line;                  // 行番号（複数行入力対応、1-based）
  int column;                // 列番号（1-based）
  char token[64];            // エラーの原因となったトークン
  char context[128];         // エラー周辺のコンテキスト（前後数トークン）
} V4FrontError;

/**
 * @brief 詳細エラー情報付きコンパイル
 *
 * 従来の v4front_compile() と同じだが、
 * エラー時に詳細情報を返します。
 *
 * @param source ソースコード
 * @param out_buf 出力バッファ
 * @param error_out エラー情報の格納先（NULL 可）
 * @return 0 on success, negative on error
 */
v4front_err v4front_compile_ex(
  const char* source,
  V4FrontBuf* out_buf,
  V4FrontError* error_out
);

/**
 * @brief コンテキスト付き + 詳細エラー情報でコンパイル
 *
 * @param ctx コンパイラコンテキスト
 * @param source ソースコード
 * @param out_buf 出力バッファ
 * @param error_out エラー情報の格納先（NULL 可）
 * @return 0 on success, negative on error
 */
v4front_err v4front_compile_with_context_ex(
  V4FrontContext* ctx,
  const char* source,
  V4FrontBuf* out_buf,
  V4FrontError* error_out
);

// ========================================
// エラーフォーマット（便利機能）
// ========================================

/**
 * @brief エラーを人間可読形式でフォーマット
 *
 * @param error エラー情報
 * @param source 元のソースコード（位置表示用）
 * @param out_buf 出力バッファ
 * @param out_cap 出力バッファサイズ
 */
void v4front_format_error(
  const V4FrontError* error,
  const char* source,
  char* out_buf,
  size_t out_cap
);
```

### 使用例: REPL での活用

```c
int Repl::eval_line(const char* line) {
  V4FrontBuf buf;
  V4FrontError error;

  // 詳細エラー情報付きでコンパイル
  v4front_err err = v4front_compile_with_context_ex(
    compiler_ctx_,
    line,
    &buf,
    &error
  );

  if (err != 0) {
    // 詳細なエラー表示
    char formatted[512];
    v4front_format_error(&error, line, formatted, sizeof(formatted));
    fprintf(stderr, "%s\n", formatted);
    return -1;
  }

  // 成功時の処理...
}
```

### 出力例

**ケース1: 未定義ワード**
```
v4> 1 2 UNKNOWN +

Error: Unknown word 'UNKNOWN' at line 1, column 5
  1 2 UNKNOWN +
      ^~~~~~~
Did you mean: DUP, DROP, SWAP?
```

**ケース2: 文法エラー**
```
v4> : SQUARE DUP *

Error: Unexpected end of input at line 1, column 16
  : SQUARE DUP *
                ^
Expected ';' to close word definition
```

**ケース3: 数値パースエラー**
```
v4> 123456789012345678901234567890

Error: Numeric literal out of range at line 1, column 1
  123456789012345678901234567890
  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
Maximum value: 2147483647 (32-bit signed integer)
```

---

## 実装上の考慮点

### 1. コンテキストの内部構造（推奨）

```c
// 内部実装の一例
struct V4FrontContext {
  // ワード名 → VM インデックスのマップ
  struct WordEntry {
    char name[32];
    int vm_idx;
  } *words;
  int word_count;
  int word_capacity;
};
```

### 2. 後方互換性

既存の API は維持：
```c
// 従来通り動作（コンテキストなし）
v4front_compile(source, &buf, err, sizeof(err));

// 新しい API（オプトイン）
v4front_compile_with_context(ctx, source, &buf, err, sizeof(err));
```

### 3. メモリ管理

```c
// コンテキストは明示的に解放
V4FrontContext* ctx = v4front_context_create();
// ... use ctx ...
v4front_context_destroy(ctx);

// エラー情報はスタック確保（動的確保不要）
V4FrontError error;
v4front_compile_ex(source, &buf, &error);
```

### 4. パフォーマンス

- ワード検索: O(n) の線形探索で十分（ワード数は少ない）
- 将来的にハッシュマップ化も可能

---

## 代替案

### 最小案: ステートフルコンパイラのみ

もしエラー位置情報が大きすぎる場合、最低限以下を実装：

```c
// 必須
V4FrontContext* v4front_context_create(void);
void v4front_context_destroy(V4FrontContext* ctx);
v4front_err v4front_context_register_word(V4FrontContext* ctx, const char* name, int vm_word_idx);
v4front_err v4front_compile_with_context(V4FrontContext* ctx, const char* source, V4FrontBuf* out_buf, char* err, size_t err_cap);

// オプション
void v4front_context_reset(V4FrontContext* ctx);
```

エラー情報は Phase 2 で実装でも OK です。

---

## テストケース案

```cpp
TEST_CASE("Stateful compiler: incremental word definitions") {
  V4FrontContext* ctx = v4front_context_create();
  V4FrontBuf buf;
  char errmsg[256];

  // Define SQUARE
  SUBCASE("Define and use word") {
    v4front_err err = v4front_compile_with_context(
      ctx, ": SQUARE DUP * ;", &buf, errmsg, sizeof(errmsg)
    );
    REQUIRE(err == 0);
    REQUIRE(buf.word_count == 1);

    // Register to context
    v4front_context_register_word(ctx, "SQUARE", 0);

    // Use SQUARE in next line
    err = v4front_compile_with_context(
      ctx, "5 SQUARE", &buf, errmsg, sizeof(errmsg)
    );
    REQUIRE(err == 0);  // Should succeed!
    REQUIRE(buf.word_count == 0);
    REQUIRE(buf.size > 0);

    v4front_free(&buf);
  }

  // Unknown word should fail
  SUBCASE("Unknown word error") {
    v4front_err err = v4front_compile_with_context(
      ctx, "5 UNKNOWN", &buf, errmsg, sizeof(errmsg)
    );
    REQUIRE(err < 0);  // Should fail
    CHECK(strstr(errmsg, "UNKNOWN") != nullptr);
  }

  v4front_context_destroy(ctx);
}

TEST_CASE("Error position information") {
  V4FrontError error;
  V4FrontBuf buf;

  SUBCASE("Unknown word position") {
    const char* source = "1 2 UNKNOWN +";
    v4front_err err = v4front_compile_ex(source, &buf, &error);

    REQUIRE(err < 0);
    REQUIRE(error.line == 1);
    REQUIRE(error.column == 5);  // "UNKNOWN" starts at column 5
    CHECK(strcmp(error.token, "UNKNOWN") == 0);
  }

  SUBCASE("Missing semicolon") {
    const char* source = ": SQUARE DUP *";
    v4front_err err = v4front_compile_ex(source, &buf, &error);

    REQUIRE(err < 0);
    REQUIRE(error.position > 0);
    // Error message should mention missing ';'
    CHECK(strstr(error.message, ";") != nullptr);
  }
}

TEST_CASE("Context reset") {
  V4FrontContext* ctx = v4front_context_create();
  V4FrontBuf buf;
  char errmsg[256];

  // Register word
  v4front_context_register_word(ctx, "TEST", 0);
  REQUIRE(v4front_context_get_word_count(ctx) == 1);

  // Reset
  v4front_context_reset(ctx);
  REQUIRE(v4front_context_get_word_count(ctx) == 0);

  // Word should be unknown now
  v4front_err err = v4front_compile_with_context(
    ctx, "TEST", &buf, errmsg, sizeof(errmsg)
  );
  REQUIRE(err < 0);

  v4front_context_destroy(ctx);
}

TEST_CASE("Error formatting") {
  V4FrontError error = {
    .code = -1,
    .message = "Unknown word",
    .position = 4,
    .line = 1,
    .column = 5,
    .token = "UNKNOWN",
    .context = "1 2 UNKNOWN +"
  };

  char formatted[512];
  v4front_format_error(&error, "1 2 UNKNOWN +", formatted, sizeof(formatted));

  // Should contain line number, column, and pointer
  CHECK(strstr(formatted, "line 1") != nullptr);
  CHECK(strstr(formatted, "column 5") != nullptr);
  CHECK(strstr(formatted, "^") != nullptr);
}
```

---

## 統合テスト（REPL との組み合わせ）

```cpp
TEST_CASE("REPL integration: persistent word definitions") {
  // Setup VM and compiler context
  uint8_t ram[1024] = {0};
  VmConfig cfg = {ram, sizeof(ram), nullptr, 0};
  Vm* vm = vm_create(&cfg);
  V4FrontContext* ctx = v4front_context_create();

  // Line 1: Define SQUARE
  {
    V4FrontBuf buf;
    char errmsg[256];
    v4front_err err = v4front_compile_with_context(
      ctx, ": SQUARE DUP * ;", &buf, errmsg, sizeof(errmsg)
    );
    REQUIRE(err == 0);

    int wid = vm_register_word(vm, buf.words[0].name, buf.words[0].code, buf.words[0].code_len);
    REQUIRE(wid >= 0);

    v4front_context_register_word(ctx, buf.words[0].name, wid);
    v4front_free(&buf);
  }

  // Line 2: Use SQUARE
  {
    V4FrontBuf buf;
    char errmsg[256];
    v4front_err err = v4front_compile_with_context(
      ctx, "5 SQUARE", &buf, errmsg, sizeof(errmsg)
    );
    REQUIRE(err == 0);  // Should compile successfully!

    int wid = vm_register_word(vm, nullptr, buf.data, buf.size);
    Word* entry = vm_get_word(vm, wid);
    v4_err exec_err = vm_exec(vm, entry);
    REQUIRE(exec_err == 0);

    // Check result
    REQUIRE(vm_ds_depth_public(vm) == 1);
    REQUIRE(vm_ds_peek_public(vm, 0) == 25);

    v4front_free(&buf);
  }

  v4front_context_destroy(ctx);
  vm_destroy(vm);
}
```

---

## まとめ

### 優先度1: ステートフルコンパイラ（必須）

**必須 API**:
- `v4front_context_create()` / `v4front_context_destroy()`
- `v4front_context_register_word()` - ワード登録
- `v4front_compile_with_context()` - コンテキスト付きコンパイル

**推奨 API**:
- `v4front_context_reset()` - 辞書クリア
- `v4front_context_get_word_count()` / `v4front_context_get_word_name()` - 一覧取得

### 優先度2: エラー位置情報（強く推奨）

**推奨 API**:
- `V4FrontError` 構造体
- `v4front_compile_ex()` - 詳細エラー付きコンパイル
- `v4front_compile_with_context_ex()` - 両方対応
- `v4front_format_error()` - フォーマット関数

### 期待される効果

これらの機能により：
- ✅ REPL で自然なワード定義と使用が可能
- ✅ スタックを保持したままワード定義可能
- ✅ 分かりやすいエラーメッセージ
- ✅ 開発体験が大幅に向上
- ✅ V4-front の汎用性が向上

---

## 参考情報

- **V4-repl リポジトリ**: `/home/kirisaki/devel/V4-repl`
- **既存の実装**: `src/repl.cpp` の `eval_line()` メソッド
- **現在の問題**: ワード定義時に VM をリセット、スタックがクリア
- **V4-front API ヘッダ**: `include/v4front/compile.h`
- **V4-front エラー定義**: `include/v4front/errors.h`
