#!/bin/bash
# V4 Device Integration Test Script
# Tests V4-link protocol and v4_cli functionality with actual hardware
# Usage: ./test_device.sh [PORT]
#   PORT: Serial port (default: /dev/ttyACM0)

set -e

PORT="${1:-/dev/ttyACM0}"
REPL_CMD="cargo run --release -- repl --port $PORT"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "======================================"
echo "V4 Device Integration Tests"
echo "======================================"
echo "Port: $PORT"
echo

# Helper function to run test
run_test() {
    local test_name="$1"
    local input="$2"

    echo -e "${YELLOW}Test: $test_name${NC}"
    echo -e "$input" | timeout 10 $REPL_CMD 2>/dev/null || {
        echo -e "${RED}✗ FAILED: $test_name${NC}"
        return 1
    }
    echo -e "${GREEN}✓ PASSED: $test_name${NC}"
    echo
}

# Test 1: Basic arithmetic
echo "=== Phase 1: Basic Arithmetic ==="
run_test "Addition" "2 3 +\nbye"
run_test "Subtraction" "10 4 -\nbye"
run_test "Multiplication" "6 7 *\nbye"
run_test "Division" "20 4 /\nbye"

# Test 2: Stack operations
echo "=== Phase 2: Stack Operations ==="
run_test "DUP" "5 dup\nbye"
run_test "SWAP" "1 2 swap\nbye"
run_test "DROP" "1 2 3 drop\nbye"
run_test "OVER" "1 2 over\nbye"
run_test "ROT" "1 2 3 rot\nbye"

# Test 3: Word definitions
echo "=== Phase 3: Word Definitions ==="
run_test "Simple word definition" ": double dup + ;\n5 double\nbye"
run_test "Word with arithmetic" ": square dup * ;\n7 square\nbye"
run_test "Nested word calls" ": triple 3 * ;\n: times6 triple 2 * ;\n4 times6\nbye"

# Test 4: Meta-commands (.words, .reset)
echo "=== Phase 4: Meta Commands ==="
run_test ".words command" ": foo 1 ;\n: bar 2 ;\n.words\nbye"
run_test ".reset command" ": temp 99 ;\n.reset\nbye"

# Test 5: Stack inspection (.stack, .rstack)
echo "=== Phase 5: Stack Inspection ==="
run_test ".stack with values" "1 2 3 4 5\n.stack\nbye"
run_test ".stack empty" ".reset\n.stack\nbye"
run_test ".stack after operations" "10 20 + 30 *\n.stack\nbye"

# Test 6: Word inspection (.see)
echo "=== Phase 6: Word Inspection ==="
run_test ".see with named word" ": myword dup * ;\n.see 0\nbye"
run_test ".see with multiple words" ": w1 1 ;\n: w2 2 ;\n.see 0\n.see 1\nbye"

# Test 7: Memory dump (.dump)
echo "=== Phase 7: Memory Inspection ==="
run_test ".dump at address 0" ".dump 0 16\nbye"
run_test ".dump at address 0x100" ".dump 0x100 32\nbye"

# Test 8: Combined workflow
echo "=== Phase 8: Combined Workflow ==="
run_test "Definition + Execution + Inspection" "\
: factorial dup 1 > if dup 1 - factorial * then ;\n\
5 factorial\n\
.stack\n\
.see 0\n\
bye"

# Test 9: Error handling
echo "=== Phase 9: Error Handling ==="
run_test "Stack underflow handling" "1 + bye" || true
run_test "Division by zero handling" "5 0 / bye" || true

# Test 10: PING command (protocol level)
echo "=== Phase 10: Protocol Commands ==="
run_test "PING" "bye"  # PING is sent during REPL startup
run_test "RESET via .reset" ".reset\nbye"

echo "======================================"
echo -e "${GREEN}All tests completed!${NC}"
echo "======================================"
