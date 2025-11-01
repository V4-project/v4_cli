#!/usr/bin/env python3
"""
Automated test script for V4 REPL.

Usage:
    python test_repl.py /dev/ttyACM0
"""

import sys
import pexpect
import time

def test_repl(port):
    """Test V4 REPL interactively."""
    print(f"Testing V4 REPL on {port}...")

    # Start REPL
    cmd = f"./target/release/v4 repl --port {port}"
    child = pexpect.spawn(cmd, timeout=10, encoding='utf-8')
    child.logfile = sys.stdout

    try:
        # Wait for welcome message
        child.expect("V4 REPL")
        child.expect("Device ready")
        print("✓ REPL started successfully")

        # Test 1: Simple arithmetic
        print("\n[TEST 1] Simple arithmetic: 1 2 +")
        child.sendline("1 2 +")
        child.expect("ok")
        print("✓ Simple arithmetic works")

        # Test 2: Word definition
        print("\n[TEST 2] Define word: : sq dup * ;")
        child.sendline(": sq dup * ;")
        child.expect("ok")
        print("✓ Word definition works")

        # Test 3: Use defined word
        print("\n[TEST 3] Use defined word: 5 sq")
        child.sendline("5 sq")
        child.expect("ok")
        print("✓ Defined word execution works")

        # Test 4: Multiple words
        print("\n[TEST 4] Define another word: : double 2 * ;")
        child.sendline(": double 2 * ;")
        child.expect("ok")
        print("✓ Multiple word definitions work")

        # Test 5: Use both words
        print("\n[TEST 5] Use both words: 3 double sq")
        child.sendline("3 double sq")
        child.expect("ok")
        print("✓ Multiple words work together")

        # Test 6: .ping command
        print("\n[TEST 6] Meta command: .ping")
        child.sendline(".ping")
        child.expect("Device is responsive")
        print("✓ .ping command works")

        # Test 7: Exit
        print("\n[TEST 7] Exit REPL")
        child.sendline("bye")
        child.expect("Goodbye!")
        print("✓ Exit works")

        print("\n✅ All tests passed!")
        return True

    except pexpect.TIMEOUT as e:
        print(f"\n❌ Test failed: Timeout waiting for response")
        print(f"Buffer: {child.buffer}")
        print(f"Before: {child.before}")
        return False
    except pexpect.EOF as e:
        print(f"\n❌ Test failed: Unexpected EOF")
        return False
    except Exception as e:
        print(f"\n❌ Test failed: {e}")
        return False
    finally:
        child.close()

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python test_repl.py <port>")
        print("Example: python test_repl.py /dev/ttyACM0")
        sys.exit(1)

    port = sys.argv[1]
    success = test_repl(port)
    sys.exit(0 if success else 1)
