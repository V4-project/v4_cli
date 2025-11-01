#!/bin/bash
# Smoke tests for V4-repl

set -e

# Auto-detect REPL executable path (Unix vs Windows)
if [ -f "./build/v4-repl" ]; then
    REPL="./build/v4-repl"
elif [ -f "./build/Debug/v4-repl.exe" ]; then
    REPL="./build/Debug/v4-repl.exe"
elif [ -f "./build/Release/v4-repl.exe" ]; then
    REPL="./build/Release/v4-repl.exe"
elif [ -f "./build/v4-repl.exe" ]; then
    REPL="./build/v4-repl.exe"
else
    echo "❌ Error: v4-repl executable not found"
    echo "   Searched in:"
    echo "     ./build/v4-repl"
    echo "     ./build/Debug/v4-repl.exe"
    echo "     ./build/Release/v4-repl.exe"
    echo "     ./build/v4-repl.exe"
    exit 1
fi

echo "🧪 Running smoke tests..."
echo "   Using: $REPL"

# Test 1: Basic arithmetic
echo "  Test 1: Basic arithmetic (1 + 2)..."
OUTPUT=$(echo -e "1 2 +\nbye" | $REPL 2>&1)
if echo "$OUTPUT" | grep -qF "ok [1]: 3"; then
    echo "  ✅ Test 1 passed"
else
    echo "  ❌ Test 1 failed"
    echo "$OUTPUT"
    exit 1
fi

# Test 2: Stack operations
echo "  Test 2: Stack operations (10 20 30 + *)..."
OUTPUT=$(echo -e "10 20 30\n+\n*\nbye" | $REPL 2>&1)
if echo "$OUTPUT" | grep -qF "ok [1]: 500"; then
    echo "  ✅ Test 2 passed"
else
    echo "  ❌ Test 2 failed"
    echo "$OUTPUT"
    exit 1
fi

# Test 3: Word definition
echo "  Test 3: Word definition..."
OUTPUT=$(echo -e ": SQUARE DUP * ; 5 SQUARE\nbye" | $REPL 2>&1)
if echo "$OUTPUT" | grep -qF "ok [1]: 25"; then
    echo "  ✅ Test 3 passed"
else
    echo "  ❌ Test 3 failed"
    echo "$OUTPUT"
    exit 1
fi

# Test 4: DUP operation
echo "  Test 4: DUP operation..."
OUTPUT=$(echo -e "5 DUP\n*\nbye" | $REPL 2>&1)
if echo "$OUTPUT" | grep -qF "ok [1]: 25"; then
    echo "  ✅ Test 4 passed"
else
    echo "  ❌ Test 4 failed"
    echo "$OUTPUT"
    exit 1
fi

# Test 5: Cross-line word definition (Phase 2 feature!)
echo "  Test 5: Cross-line word definition..."
OUTPUT=$(echo -e ": SQUARE DUP * ;\n5 SQUARE\nbye" | $REPL 2>&1)
if echo "$OUTPUT" | grep -qF "ok [1]: 25"; then
    echo "  ✅ Test 5 passed"
else
    echo "  ❌ Test 5 failed"
    echo "$OUTPUT"
    exit 1
fi

# Test 6: Chained word definitions
echo "  Test 6: Chained word definitions..."
OUTPUT=$(echo -e ": DOUBLE DUP + ;\n: QUADRUPLE DOUBLE DOUBLE ;\n3 QUADRUPLE\nbye" | $REPL 2>&1)
if echo "$OUTPUT" | grep -qF "ok [1]: 12"; then
    echo "  ✅ Test 6 passed"
else
    echo "  ❌ Test 6 failed"
    echo "$OUTPUT"
    exit 1
fi

# Test 7: Stack preservation across word definitions
echo "  Test 7: Stack preservation..."
OUTPUT=$(echo -e "10 20\n: DOUBLE DUP + ;\n30 DOUBLE\nbye" | $REPL 2>&1)
if echo "$OUTPUT" | grep -qF "ok [3]: 10 20 60"; then
    echo "  ✅ Test 7 passed"
else
    echo "  ❌ Test 7 failed"
    echo "$OUTPUT"
    exit 1
fi

# Test 8: Error message with position
echo "  Test 8: Error message with position..."
OUTPUT=$(echo -e "NONEXISTENT\nbye" | $REPL 2>&1)
if echo "$OUTPUT" | grep -q "unknown token at line 1, column 1"; then
    echo "  ✅ Test 8 passed"
else
    echo "  ❌ Test 8 failed"
    echo "$OUTPUT"
    exit 1
fi

echo "✅ All smoke tests passed!"
