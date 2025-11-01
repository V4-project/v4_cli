#!/bin/bash
# V4-repl バイナリサイズ分析スクリプト

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# 色定義
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# ヘッダー
echo -e "${BOLD}${CYAN}========================================${NC}"
echo -e "${BOLD}${CYAN}   V4-repl Binary Size Report${NC}"
echo -e "${BOLD}${CYAN}========================================${NC}\n"

# 関数: バイナリのサイズを取得して表示
show_binary_size() {
  local path=$1
  local label=$2
  local color=$3

  if [ ! -f "$path" ]; then
    echo -e "${color}${label}:${NC} ${RED}Not found${NC}"
    return
  fi

  local size=$(ls -lh "$path" | awk '{print $5}')
  local size_bytes=$(stat -c%s "$path" 2>/dev/null || stat -f%z "$path" 2>/dev/null)

  echo -e "${color}${label}:${NC} ${BOLD}${size}${NC} (${size_bytes} bytes)"
}

# 関数: セクション情報を表示
show_sections() {
  local path=$1

  if [ ! -f "$path" ]; then
    return
  fi

  echo -e "\n${YELLOW}Section breakdown:${NC}"
  size "$path" | awk -v bold="${BOLD}" -v nc="${NC}" 'NR==1 {printf "  %-10s %10s\n", "Section", "Size"} NR==2 {printf "  %-10s %10d bytes\n", ".text", $1; printf "  %-10s %10d bytes\n", ".data", $2; printf "  %-10s %10d bytes\n", ".bss", $3; printf "  " bold "%-10s %10d bytes" nc "\n", "Total", $4}'
}

# 関数: strip してサイズを表示
show_stripped_size() {
  local path=$1
  local label=$2

  if [ ! -f "$path" ]; then
    return
  fi

  local temp_stripped="${path}.tmp_stripped"
  cp "$path" "$temp_stripped" 2>/dev/null || return
  strip "$temp_stripped" 2>/dev/null || { rm -f "$temp_stripped"; return; }

  local size=$(ls -lh "$temp_stripped" | awk '{print $5}')
  local size_bytes=$(stat -c%s "$temp_stripped" 2>/dev/null || stat -f%z "$temp_stripped" 2>/dev/null)
  local orig_bytes=$(stat -c%s "$path" 2>/dev/null || stat -f%z "$path" 2>/dev/null)
  local saved=$((orig_bytes - size_bytes))
  local saved_kb=$((saved / 1024))

  echo -e "  ${GREEN}→ Stripped:${NC} ${BOLD}${size}${NC} (saved ${saved_kb}KB debug symbols)"

  rm -f "$temp_stripped"
}

# 現在のビルド
echo -e "${BOLD}Current Builds:${NC}\n"

if [ -f "build/v4-repl" ]; then
  show_binary_size "build/v4-repl" "Debug build" "$BLUE"
  show_sections "build/v4-repl"
  show_stripped_size "build/v4-repl" "Debug"
  echo ""
fi

if [ -f "build-release/v4-repl" ]; then
  show_binary_size "build-release/v4-repl" "Release build" "$GREEN"
  show_sections "build-release/v4-repl"
  show_stripped_size "build-release/v4-repl" "Release"
  echo ""
fi

if [ -f "build-opt/v4-repl" ]; then
  show_binary_size "build-opt/v4-repl" "Optimized build (-O3)" "$GREEN"
  show_sections "build-opt/v4-repl"
  show_stripped_size "build-opt/v4-repl" "Optimized"
  echo ""
fi

# サイズ最適化ビルド推奨
echo -e "${BOLD}${YELLOW}Recommended Builds:${NC}\n"

echo -e "${CYAN}For production (balanced):${NC}"
echo -e "  cmake -B build-release -DCMAKE_BUILD_TYPE=Release"
echo -e "  cmake --build build-release"
echo -e "  strip build-release/v4-repl"
echo -e "  ${GREEN}→ Expected size: ~67KB${NC}\n"

echo -e "${CYAN}For minimum size:${NC}"
echo -e "  cmake -B build-size -DCMAKE_BUILD_TYPE=MinSizeRel"
echo -e "  cmake --build build-size"
echo -e "  strip build-size/v4-repl"
echo -e "  ${GREEN}→ Expected size: ~60KB${NC}\n"

# 比較表
if [ -f "build/v4-repl" ] || [ -f "build-release/v4-repl" ]; then
  echo -e "${BOLD}${CYAN}Size Comparison:${NC}\n"

  echo -e "  ${BOLD}Build Type              Size (with debug)  Size (stripped)${NC}"
  echo -e "  ─────────────────────────────────────────────────────────"

  if [ -f "build/v4-repl" ]; then
    debug_size=$(ls -lh build/v4-repl | awk '{print $5}')
    debug_stripped=$(cp build/v4-repl /tmp/v4-repl-test && strip /tmp/v4-repl-test && ls -lh /tmp/v4-repl-test | awk '{print $5}'; rm -f /tmp/v4-repl-test)
    printf "  Debug                   %-18s %-15s\n" "$debug_size" "$debug_stripped"
  fi

  if [ -f "build-release/v4-repl" ]; then
    rel_size=$(ls -lh build-release/v4-repl | awk '{print $5}')
    rel_stripped=$(cp build-release/v4-repl /tmp/v4-repl-test && strip /tmp/v4-repl-test && ls -lh /tmp/v4-repl-test | awk '{print $5}'; rm -f /tmp/v4-repl-test)
    printf "  Release                 %-18s ${GREEN}%-15s${NC}\n" "$rel_size" "$rel_stripped"
  fi

  echo ""
fi

# フッター
echo -e "${BOLD}${CYAN}========================================${NC}"
echo -e "${CYAN}Tip: Run 'make size' for quick check${NC}"
echo -e "${BOLD}${CYAN}========================================${NC}"
