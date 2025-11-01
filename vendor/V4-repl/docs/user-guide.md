# V4 REPL User Guide

Welcome to the V4 REPL (Read-Eval-Print Loop) - an interactive environment for the V4 Forth virtual machine.

## Table of Contents

- [Getting Started](#getting-started)
- [Basic Usage](#basic-usage)
- [Word Definitions](#word-definitions)
- [PASTE Mode](#paste-mode)
- [Meta-Commands](#meta-commands)
- [Control Flow](#control-flow)
- [Stack Operations](#stack-operations)
- [Tips & Tricks](#tips--tricks)

## Getting Started

### Building the REPL

```bash
# Build from source
make build

# Run the REPL
make run

# Or run directly
./build/v4-repl
```

### Starting the REPL

```bash
$ ./build/v4-repl
V4 REPL v0.2.0
Type 'bye' or press Ctrl+D to exit
Type '.help' for help
Type '<<<' to enter PASTE mode

v4>
```

## Basic Usage

### Arithmetic Operations

V4 uses Reverse Polish Notation (RPN):

```forth
v4> 1 2 +
 ok [1]: 3

v4> 10 5 -
 ok [1]: 5

v4> 6 7 *
 ok [1]: 42

v4> 100 10 /
 ok [1]: 10
```

### Stack Operations

The REPL shows the data stack after each successful operation:

```forth
v4> 42
 ok [1]: 42

v4> DUP
 ok [2]: 42 42

v4> *
 ok [1]: 1764
```

Common stack operations:
- `DUP` - Duplicate top of stack
- `DROP` - Remove top of stack
- `SWAP` - Swap top two values
- `OVER` - Copy second value to top
- `ROT` - Rotate top three values

## Word Definitions

### Simple Word Definitions

Define reusable words with `: NAME ... ;` syntax:

```forth
v4> : DOUBLE 2 * ;
 ok

v4> 21 DOUBLE
 ok [1]: 42
```

### Multi-Word Definitions

Words can call other words:

```forth
v4> : SQUARE DUP * ;
 ok

v4> : QUAD SQUARE SQUARE ;
 ok

v4> 2 QUAD
 ok [1]: 16
```

### Checking Defined Words

Use the `.words` meta-command to list all defined words:

```forth
v4> : DOUBLE 2 * ;
 ok

v4> : TRIPLE 3 * ;
 ok

v4> .words
Defined words (2):
  DOUBLE
  TRIPLE
 ok
```

## PASTE Mode

PASTE mode allows you to enter multi-line definitions easily, which is especially useful for complex word definitions and control structures.

### Entering PASTE Mode

Type `<<<` to enter PASTE mode:

```forth
v4> <<<
Entering PASTE mode. Type '>>>' to compile and execute.
...
```

Notice the prompt changes from `v4>` to `...` to indicate PASTE mode.

### Multi-Line Word Definition

```forth
v4> <<<
Entering PASTE mode. Type '>>>' to compile and execute.
... : SQUARE
...   DUP *
... ;
... >>>
 ok

v4> 5 SQUARE
 ok [1]: 25
```

### Complex Control Structures

PASTE mode is ideal for nested control structures:

```forth
v4> <<<
Entering PASTE mode. Type '>>>' to compile and execute.
... : ABS
...   DUP 0 <
...   IF
...     NEGATE
...   THEN
... ;
... >>>
 ok

v4> -42 ABS
 ok [1]: 42
```

### Exiting PASTE Mode

- Type `>>>` to compile and execute the buffered code
- Press `Ctrl+C` to cancel and exit PASTE mode
- Empty buffer will show `(empty PASTE buffer)`

## Meta-Commands

Meta-commands start with `.` and provide REPL control and inspection features.

### `.help` - Show Help

Display comprehensive help:

```forth
v4> .help
V4 REPL Help
════════════════════════════════════════════════════════════════

Meta-commands:
  .words     - List all defined words
  .stack     - Show detailed stack contents (hex and decimal)
  .reset     - Reset VM and compiler context
  .memory    - Show memory usage statistics
  .help      - Show this help message
  .version   - Show REPL and component versions
...
```

### `.words` - List Defined Words

Show all user-defined words:

```forth
v4> : DOUBLE 2 * ;
 ok

v4> : SQUARE DUP * ;
 ok

v4> .words
Defined words (2):
  DOUBLE
  SQUARE
 ok
```

### `.stack` - Detailed Stack View

Display stack contents in both decimal and hexadecimal:

```forth
v4> 10 20 30
 ok [3]: 10 20 30

v4> .stack
Data Stack (depth: 3):
  [0]: 10 (0x0000000A)
  [1]: 20 (0x00000014)
  [2]: 30 (0x0000001E)

Return Stack: <not yet implemented>
 ok [3]: 10 20 30
```

### `.reset` - Reset Everything

Clear the VM, stack, and all word definitions:

```forth
v4> : TEST 42 ;
 ok

v4> .words
Defined words (1):
  TEST
 ok

v4> .reset
VM and compiler context reset.
 ok

v4> .words
No words defined.
 ok
```

### `.memory` - Memory Usage

Show memory usage statistics (note: detailed stats require V4-core API enhancements):

```forth
v4> .memory
Memory usage information:
  VM memory size: (not yet available from V4-core)
  Data stack depth: 3
  Return stack depth: (API not yet available)
  Registered words: 5
 ok
```

### `.version` - Version Information

Display REPL and component versions:

```forth
v4> .version
V4 REPL v0.2.0
════════════════════════════════════════════════════════════════
Components:
  V4-core:   (version API not yet available)
  V4-front:  (version API not yet available)
  linenoise: integrated

Build configuration:
  Filesystem: enabled (history support)
  C++ standard: C++17
  Exceptions: disabled (-fno-exceptions)
  RTTI: disabled (-fno-rtti)
════════════════════════════════════════════════════════════════
 ok
```

## Control Flow

### IF-THEN-ELSE

Conditional execution:

```forth
: ABS
  DUP 0 <
  IF NEGATE
  ELSE
  THEN
;

-42 ABS  ( Result: 42 )
42 ABS   ( Result: 42 )
```

### BEGIN-UNTIL

Loop until condition is true:

```forth
: COUNTDOWN
  BEGIN
    DUP .
    1 -
    DUP 0 =
  UNTIL
  DROP
;

5 COUNTDOWN  ( Prints: 5 4 3 2 1 )
```

### DO-LOOP

Counted loop:

```forth
: PRINT-NUMBERS
  10 0 DO
    I .
  LOOP
;

PRINT-NUMBERS  ( Prints: 0 1 2 3 4 5 6 7 8 9 )
```

## Stack Operations

### Viewing the Stack

The stack is shown after each successful command:

```forth
v4> 1 2 3
 ok [3]: 1 2 3
```

The format is: `ok [depth]: bottom ... top`

### Stack Manipulation

```forth
v4> 1 2 3
 ok [3]: 1 2 3

v4> DROP     ( Remove top )
 ok [2]: 1 2

v4> DUP      ( Duplicate top )
 ok [3]: 1 2 2

v4> SWAP     ( Swap top two )
 ok [3]: 1 2 2
```

### Stack Underflow

Attempting operations on an empty stack will result in an error:

```forth
v4> +
Error: Stack underflow
```

## Tips & Tricks

### 1. Use PASTE Mode for Complex Definitions

When defining words with multiple lines or nested control structures, always use PASTE mode (`<<<` ... `>>>`). It's much easier than trying to fit everything on one line.

### 2. Check Stack with `.stack` Before Debugging

If a calculation isn't working as expected, use `.stack` to see the exact stack contents in both decimal and hexadecimal.

### 3. Reset When Things Go Wrong

If you get into a confusing state, use `.reset` to start fresh. This clears everything: the stack, all word definitions, and the compiler context.

### 4. Use Ctrl+C to Interrupt

If you accidentally create an infinite loop or long-running operation, press `Ctrl+C` to interrupt it. The REPL will remain stable and you can continue working.

### 5. History Support

If compiled with filesystem support (default), the REPL saves your command history to `~/.v4_history`. Use the up/down arrow keys to navigate your history.

### 6. Comments

Use parentheses for comments:

```forth
v4> ( This is a comment )
 ok

v4> 42 ( The answer ) DUP ( Duplicate it ) *
 ok [1]: 1764
```

### 7. Line Editing

The REPL uses linenoise for line editing, providing:
- Arrow keys for cursor movement
- Home/End for line navigation
- Ctrl+A / Ctrl+E for start/end
- Ctrl+K to delete to end of line
- Ctrl+U to delete entire line

### 8. Exit Options

Multiple ways to exit the REPL:
- Type `bye`
- Type `quit`
- Press `Ctrl+D`

## Examples

### Example 1: Temperature Conversion

```forth
v4> <<<
... ( Convert Fahrenheit to Celsius: C = (F - 32) * 5 / 9 )
... : F>C
...   32 -
...   5 *
...   9 /
... ;
... >>>
 ok

v4> 212 F>C
 ok [1]: 100
```

### Example 2: Greatest Common Divisor

```forth
v4> <<<
... : GCD
...   BEGIN
...     DUP 0 >
...   WHILE
...     SWAP OVER MOD
...   REPEAT
...   DROP
... ;
... >>>
 ok

v4> 48 18 GCD
 ok [1]: 6
```

### Example 3: Sum of Squares

```forth
v4> : SQUARE DUP * ;
 ok

v4> : SUM-OF-SQUARES SQUARE SWAP SQUARE + ;
 ok

v4> 3 4 SUM-OF-SQUARES
 ok [1]: 25
```

### Example 4: Recursive Factorial (V4-front v0.3.x)

```forth
v4> : FACTORIAL DUP 1 > IF DUP 1 - RECURSE * THEN ;
 ok

v4> 5 FACTORIAL
 ok [1]: 120

v4> 10 FACTORIAL
 ok [1]: 3628800
```

### Example 5: Fibonacci with RECURSE (V4-front v0.3.x)

```forth
v4> : FIB DUP 2 < IF DROP 1 ELSE DUP 1 - RECURSE SWAP 2 - RECURSE + THEN ;
 ok

v4> 7 FIB
 ok [1]: 21

v4> 10 FIB
 ok [1]: 89
```

## Advanced Language Features (V4-front v0.3.x)

V4-front v0.3.0 and v0.3.1 introduce powerful new language features.

### Recursion with RECURSE

The `RECURSE` keyword allows a word to call itself recursively:

```forth
v4> : COUNTDOWN DUP 0 > IF DUP . 1 - RECURSE THEN ;
 ok

v4> 5 COUNTDOWN
```

**Benefits:**
- Natural expression of recursive algorithms
- Automatically references the current word being defined
- Works with multiple RECURSE calls in the same word

### Extended Arithmetic

#### Increment and Decrement
```forth
v4> 5 1+    ( Increment )
 ok [1]: 6

v4> 5 1-    ( Decrement )
 ok [1]: 4
```

#### Unsigned Arithmetic
```forth
v4> 17 5 U/     ( Unsigned division )
 ok [1]: 3

v4> 17 5 UMOD   ( Unsigned modulo )
 ok [1]: 2
```

### Bitwise Operations

```forth
v4> 1 3 LSHIFT   ( Logical left shift: 1 << 3 )
 ok [1]: 8

v4> 8 2 RSHIFT   ( Logical right shift: 8 >> 2 )
 ok [1]: 2

v4> -8 2 ARSHIFT ( Arithmetic right shift: preserves sign )
 ok [1]: -2
```

### Enhanced Stack Manipulation

#### Single-Cell Operations
```forth
v4> 1 2 3 ROT    ( Rotate: a b c -- b c a )
 ok [3]: 2 3 1

v4> 1 2 NIP      ( Remove second: a b -- b )
 ok [1]: 2

v4> 1 2 TUCK     ( Insert copy: a b -- b a b )
 ok [3]: 2 1 2
```

#### Double-Cell Operations
```forth
v4> 1 2 2DUP     ( Duplicate top two )
 ok [4]: 1 2 1 2

v4> 1 2 3 4 2DROP ( Drop top two )
 ok [2]: 1 2

v4> 1 2 3 4 2SWAP ( Swap top two pairs )
 ok [4]: 3 4 1 2

v4> 1 2 3 4 2OVER ( Copy second pair over )
 ok [6]: 1 2 3 4 1 2
```

### Arithmetic Utilities

```forth
v4> 5 NEGATE     ( Sign negation )
 ok [1]: -5

v4> -7 ABS       ( Absolute value )
 ok [1]: 7

v4> 3 7 MIN      ( Minimum )
 ok [1]: 3

v4> 3 7 MAX      ( Maximum )
 ok [1]: 7

v4> 5 ?DUP       ( Duplicate if non-zero )
 ok [2]: 5 5

v4> 0 ?DUP       ( Leave zero alone )
 ok [1]: 0
```

### Comparison Operations

#### Zero Comparison
```forth
v4> 0 0=         ( Test if zero )
 ok [1]: -1

v4> -5 0<        ( Test if less than zero )
 ok [1]: -1

v4> 5 0>         ( Test if greater than zero )
 ok [1]: -1
```

#### Unsigned Comparison
```forth
v4> 5 10 U<      ( Unsigned less than )
 ok [1]: -1

v4> 10 10 U<=    ( Unsigned less than or equal )
 ok [1]: -1
```

### Boolean Constants

```forth
v4> TRUE         ( Boolean true: -1 )
 ok [1]: -1

v4> FALSE        ( Boolean false: 0 )
 ok [1]: 0

v4> 5 5 = TRUE = ( Compare result with TRUE )
 ok [1]: -1
```

### Memory Access

#### Byte Operations
```forth
v4> ( Store and fetch bytes )
v4> variable BYTE-VAR
v4> 255 BYTE-VAR C!  ( Store byte )
v4> BYTE-VAR C@      ( Fetch byte )
 ok [1]: 255
```

#### Halfword Operations
```forth
v4> ( Store and fetch 16-bit values )
v4> variable HALF-VAR
v4> 65535 HALF-VAR W!  ( Store halfword )
v4> HALF-VAR W@        ( Fetch halfword )
 ok [1]: 65535
```

## Troubleshooting

### "unknown token" Error

This means the word is not defined. Check:
1. Spelling and case (V4 is case-insensitive by default)
2. Whether you defined the word earlier
3. Use `.words` to see all defined words

### Stack Depth Mismatch

If operations fail unexpectedly, check the stack depth with `.stack`. Each word should have a consistent stack effect.

### PASTE Mode Confusion

If you're in PASTE mode by accident:
- Type `>>>` to exit with empty buffer
- Press `Ctrl+C` to cancel

### Cannot Exit

If `bye` or `Ctrl+D` don't work, check if you're in PASTE mode (prompt shows `...`). Exit PASTE mode first with `>>>` or `Ctrl+C`.

## Next Steps

- Read the [Meta-Commands Reference](meta-commands.md) for detailed command documentation
- Explore the V4 Forth instruction set
- Try implementing your own word definitions
- Experiment with control structures in PASTE mode

Happy Forth programming! 🚀
