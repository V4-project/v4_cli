#!/bin/bash
# Test script for V4-front extended Forth features
# Tests composite words, primitives, and language extensions

REPL="./build/v4-repl"
FAILED=0

echo "🧪 Testing V4-front extended features..."

# Helper function to test REPL commands
test_feature() {
    local name="$1"
    local input="$2"
    local expected="$3"

    echo -n "  Testing $name... "

    result=$(echo -e "$input\nbye" | $REPL 2>&1 | grep -A 1 "v4>" | tail -1 | tr -d '\n' | sed 's/^ *//')

    if echo "$result" | grep -q "$expected"; then
        echo "✅"
    else
        echo "❌ (expected: $expected, got: $result)"
        FAILED=$((FAILED + 1))
    fi
}

# Test zero comparison words
echo ""
echo "📦 Testing zero comparison words:"
test_feature "0= (zero)" "0 0=" "ok \[1\]: -1"
test_feature "0= (non-zero)" "5 0=" "ok \[1\]: 0"
test_feature "0< (negative)" "-5 0<" "ok \[1\]: -1"
test_feature "0< (positive)" "5 0<" "ok \[1\]: 0"
test_feature "0> (positive)" "5 0>" "ok \[1\]: -1"
test_feature "0> (negative)" "-5 0>" "ok \[1\]: 0"

# Test double-cell stack operations
echo ""
echo "📦 Testing double-cell stack operations:"
test_feature "2DUP" "1 2 2DUP" "ok \[4\]: 1 2 1 2"
test_feature "2DROP" "1 2 3 4 2DROP" "ok \[2\]: 1 2"
test_feature "2SWAP" "1 2 3 4 2SWAP" "ok \[4\]: 3 4 1 2"
test_feature "2OVER" "1 2 3 4 2OVER" "ok \[6\]: 1 2 3 4 1 2"

# Test boolean constants
echo ""
echo "📦 Testing boolean constants:"
test_feature "TRUE" "TRUE" "ok \[1\]: -1"
test_feature "FALSE" "FALSE" "ok \[1\]: 0"

# Test extended arithmetic primitives
echo ""
echo "📦 Testing arithmetic primitives:"
test_feature "1+" "5 1+" "ok \[1\]: 6"
test_feature "1-" "5 1-" "ok \[1\]: 4"
test_feature "U/" "10 3 U/" "ok \[1\]: 3"
test_feature "UMOD" "10 3 UMOD" "ok \[1\]: 1"

# Test bitwise operations
echo ""
echo "📦 Testing bitwise operations:"
test_feature "LSHIFT" "1 3 LSHIFT" "ok \[1\]: 8"
test_feature "RSHIFT" "8 2 RSHIFT" "ok \[1\]: 2"
test_feature "ARSHIFT" "-8 2 ARSHIFT" "ok \[1\]: -2"

# Test unsigned comparison
echo ""
echo "📦 Testing unsigned comparison:"
test_feature "U<" "5 10 U<" "ok \[1\]: -1"
test_feature "U<=" "5 5 U<=" "ok \[1\]: -1"

# Test composite words
echo ""
echo "📦 Testing composite words:"
test_feature "ROT" "1 2 3 ROT" "ok \[3\]: 2 3 1"
test_feature "NIP" "1 2 NIP" "ok \[1\]: 2"
test_feature "TUCK" "1 2 TUCK" "ok \[3\]: 2 1 2"
test_feature "NEGATE" "5 NEGATE" "ok \[1\]: -5"
test_feature "ABS (positive)" "5 ABS" "ok \[1\]: 5"
test_feature "ABS (negative)" "-5 ABS" "ok \[1\]: 5"
test_feature "MIN" "3 7 MIN" "ok \[1\]: 3"
test_feature "MAX" "3 7 MAX" "ok \[1\]: 7"

# Test recursion with RECURSE
echo ""
echo "📦 Testing recursion (RECURSE):"
echo -e ": FACTORIAL DUP 1 > IF DUP 1 - RECURSE * THEN ;\n5 FACTORIAL\nbye" | $REPL > /tmp/repl_test.out 2>&1
if grep -q "ok \[1\]: 120" /tmp/repl_test.out; then
    echo "  Testing RECURSE (factorial)... ✅"
else
    echo "  Testing RECURSE (factorial)... ❌"
    FAILED=$((FAILED + 1))
fi

echo ""
if [ $FAILED -eq 0 ]; then
    echo "✅ All extended feature tests passed!"
    exit 0
else
    echo "❌ $FAILED test(s) failed"
    exit 1
fi
