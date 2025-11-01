# V4 REPL

Interactive Forth REPL for the V4 virtual machine.

## Features

### Core Features
- Interactive command-line interface with linenoise
- **Persistent word definitions across lines** - define words on one line, use them on subsequent lines
- **Stack preservation** - stack contents are maintained across all operations
- **Detailed error messages** - error position information with visual indicators
- Stack display after each command
- Command history (when filesystem support is enabled)
- Compile-time configurable for embedded systems

### Advanced Features (v0.2.0+)
- **🔧 Meta-commands** - Built-in commands for REPL control and inspection (`.words`, `.stack`, `.reset`, `.memory`, `.help`, `.version`)
- **📝 PASTE mode** - Multi-line input mode for complex word definitions (`<<<` to enter, `>>>` to execute)
- **⚡ Ctrl+C interrupt handling** - Safely interrupt long-running operations without crashing the REPL
- **📚 Comprehensive documentation** - User guide and meta-commands reference

### Supported Forth Language Features (V4-front v0.3.x)
- **🔢 Extended arithmetic** - `1+`, `1-`, `U/`, `UMOD` (unsigned operations)
- **🔀 Bitwise operations** - `LSHIFT`, `RSHIFT`, `ARSHIFT` (shift operations)
- **📊 Comparison operations** - `U<`, `U<=` (unsigned), `0=`, `0<`, `0>` (zero comparison)
- **📚 Stack manipulation** - `ROT`, `NIP`, `TUCK`, `2DUP`, `2DROP`, `2SWAP`, `2OVER`
- **🧮 Arithmetic utilities** - `NEGATE`, `ABS`, `MIN`, `MAX`, `?DUP`
- **🎯 Boolean constants** - `TRUE` (-1), `FALSE` (0)
- **🔁 Recursion** - `RECURSE` keyword for recursive word definitions
- **💾 Memory access** - `C@`, `C!` (byte), `W@`, `W!` (halfword)

## Requirements

- C++17 compiler
- CMake 3.15 or later
- V4 VM library
- V4-front compiler library
- linenoise (fetched automatically)

## Building

### Quick start with Makefile

```bash
make          # Build with local V4/V4-front
make run      # Build and run REPL
make test     # Run smoke tests
make help     # Show all available targets
```

### Standard build with CMake

```bash
mkdir build
cd build
cmake ..
make
```

### Embedded build without filesystem

For systems without filesystem support (no history feature):

```bash
# Using Makefile
make build-no-fs

# Or using CMake
mkdir build
cd build
cmake -DWITH_FILESYSTEM=OFF ..
make
```

### Building with V4-hal (C++17 CRTP HAL)

V4-repl can optionally use [V4-hal](https://github.com/kirisaki/V4-hal) for zero-cost hardware abstraction:

```bash
# Using Makefile
make build V4_USE_V4HAL=ON

# Or using CMake
mkdir build
cd build
cmake -DV4_USE_V4HAL=ON ..
make
```

Benefits:
- C++17 CRTP for zero-cost abstraction
- Platform support: POSIX, ESP32, CH32V203
- Minimal runtime footprint (~5.7KB for GPIO+Timer)
- Backward compatible with mock HAL

### Custom V4/V4-front paths

By default, the build system looks for V4 and V4-front in `../V4` and `../V4-front`.

```bash
# Using Makefile
make build V4_PATH=/path/to/V4 V4FRONT_PATH=/path/to/V4-front

# Or using CMake
cmake -DV4_LOCAL_PATH=/path/to/V4 \
      -DV4FRONT_LOCAL_PATH=/path/to/V4-front \
      ..
```

### Available Makefile targets

```bash
make              # Build (default)
make build        # Build with local dependencies
make build-no-fs  # Build without filesystem support
make release      # Release build
make run          # Build and run REPL
make test         # Run smoke tests
make size         # Quick binary size check
make size-report  # Detailed size analysis with recommendations
make clean        # Clean build artifacts
make format       # Format code
make asan         # Build with AddressSanitizer
make ubsan        # Build with UndefinedBehaviorSanitizer
make help         # Show all targets
```

## Quick Start

```bash
# Build and run
make run

# Or run directly
./build/v4-repl
```

## Usage Examples

### Basic Arithmetic

```bash
$ ./build/v4-repl
V4 REPL v0.2.0
Type 'bye' or press Ctrl+D to exit
Type '.help' for help
Type '<<<' to enter PASTE mode

v4> 1 2 +
 ok [1]: 3

v4> 10 5 - 2 *
 ok [1]: 10
```

### Word Definitions

```forth
v4> : SQUARE DUP * ;
 ok

v4> 5 SQUARE
 ok [1]: 25

v4> : DOUBLE 2 * ;
 ok

v4> : QUADRUPLE DOUBLE DOUBLE ;
 ok

v4> 3 QUADRUPLE
 ok [1]: 12
```

### Meta-Commands

```forth
v4> : TEST 42 ;
 ok

v4> .words
Defined words (1):
  TEST
 ok

v4> 10 20 30
 ok [3]: 10 20 30

v4> .stack
Data Stack (depth: 3):
  [0]: 10 (0x0000000A)
  [1]: 20 (0x00000014)
  [2]: 30 (0x0000001E)

Return Stack: <not yet implemented>
 ok [3]: 10 20 30

v4> .help
( Shows comprehensive help )
```

### PASTE Mode (Multi-line Input)

```forth
v4> <<<
Entering PASTE mode. Type '>>>' to compile and execute.
... : FACTORIAL
...   DUP 1 >
...   IF DUP 1 - FACTORIAL *
...   ELSE DROP 1
...   THEN
... ;
... >>>
 ok

v4> 5 FACTORIAL
 ok [1]: 120
```

### Interrupt Handling

Press `Ctrl+C` during execution to safely interrupt:

```forth
v4> : FOREVER BEGIN 1 UNTIL ;
 ok

v4> FOREVER
^C
Execution interrupted
 ok

v4> ( REPL continues normally )
```

### New Language Features (V4-front v0.3.x)

#### Recursion with RECURSE

```forth
v4> : FACTORIAL DUP 1 > IF DUP 1 - RECURSE * THEN ;
 ok

v4> 5 FACTORIAL
 ok [1]: 120

v4> : FIB DUP 2 < IF DROP 1 ELSE DUP 1 - RECURSE SWAP 2 - RECURSE + THEN ;
 ok

v4> 7 FIB
 ok [1]: 21
```

#### Extended Arithmetic and Bitwise Operations

```forth
v4> 10 1+
 ok [1]: 11

v4> 10 1-
 ok [1]: 9

v4> 17 5 U/
 ok [1]: 3

v4> 1 3 LSHIFT
 ok [1]: 8

v4> 8 2 RSHIFT
 ok [1]: 2
```

#### Stack Manipulation

```forth
v4> 1 2 3 ROT
 ok [3]: 2 3 1

v4> 1 2 NIP
 ok [1]: 2

v4> 1 2 TUCK
 ok [3]: 2 1 2

v4> 1 2 2DUP
 ok [4]: 1 2 1 2
```

#### Arithmetic Utilities

```forth
v4> -5 ABS
 ok [1]: 5

v4> 3 7 MIN
 ok [1]: 3

v4> 3 7 MAX
 ok [1]: 7

v4> 5 ?DUP
 ok [2]: 5 5

v4> 0 ?DUP
 ok [1]: 0
```

#### Comparison and Boolean Operations

```forth
v4> 0 0=
 ok [1]: -1

v4> -5 0<
 ok [1]: -1

v4> TRUE
 ok [1]: -1

v4> FALSE
 ok [1]: 0
```

## Stack Display Format

After each successful command, the stack is displayed as:

```
 ok [depth]: val1 val2 ... valN
```

Where:
- `depth` is the number of elements on the stack
- Values are shown from bottom to top

If the stack is empty:

```
 ok
```

## Commands

### Exit Commands
- `bye` or `quit` - Exit the REPL
- `Ctrl+D` - Exit the REPL

### Control Keys
- `Ctrl+C` - Interrupt current execution
- `Ctrl+A` / `Home` - Move to start of line
- `Ctrl+E` / `End` - Move to end of line
- `Ctrl+K` - Delete to end of line
- `Ctrl+U` - Delete entire line
- `↑` / `↓` - Navigate command history

### Meta-Commands
- `.help` - Show comprehensive help
- `.words` - List all defined words
- `.stack` - Show detailed stack contents
- `.reset` - Reset VM and compiler context
- `.memory` - Show memory usage statistics
- `.version` - Show version information

### PASTE Mode
- `<<<` - Enter multi-line input mode
- `>>>` - Exit PASTE mode and compile/execute

### Forth Language
- Any valid V4 Forth code

## Documentation

📖 **[User Guide](docs/user-guide.md)** - Complete guide to using the REPL
📖 **[Meta-Commands Reference](docs/meta-commands.md)** - Detailed documentation for all meta-commands

## Features in Detail

### Persistent Word Definitions

Words can now be defined on one line and used on subsequent lines:

```forth
v4> : SQUARE DUP * ;
 ok

v4> 5 SQUARE
 ok [1]: 25

v4> 7 SQUARE
 ok [1]: 49
```

### Stack Preservation

The data stack is preserved across all operations, including word definitions:

```forth
v4> 10 20
 ok [2]: 10 20

v4> : DOUBLE DUP + ;
 ok [2]: 10 20        # Stack preserved!

v4> 30 DOUBLE
 ok [3]: 10 20 60
```

### Detailed Error Messages

Errors show the exact position with visual indicators:

```forth
v4> 1 2 UNKNOWN +
Error: unknown token at line 1, column 5
  1 2 UNKNOWN +
      ^~~~~~~
```

### Meta-Commands

Built-in commands for REPL control and inspection:

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

v4> 42 100
 ok [2]: 42 100

v4> .stack
Data Stack (depth: 2):
  [0]: 42 (0x0000002A)
  [1]: 100 (0x00000064)
 ok [2]: 42 100

v4> .reset
VM and compiler context reset.
 ok

v4> .words
No words defined.
 ok
```

### PASTE Mode

Multi-line input mode for complex definitions:

```forth
v4> <<<
Entering PASTE mode. Type '>>>' to compile and execute.
... : ABS
...   DUP 0 <
...   IF NEGATE
...   THEN
... ;
... >>>
 ok

v4> -42 ABS
 ok [1]: 42
```

Benefits:
- Easier entry of multi-line word definitions
- Better for control structures (IF/THEN/ELSE, BEGIN/UNTIL, etc.)
- Visual feedback with changed prompt (`...`)
- Can be interrupted with `Ctrl+C`

### Interrupt Handling

Safe interruption of long-running operations:

- Press `Ctrl+C` during execution to interrupt
- Clears the data stack
- Returns to REPL prompt
- Works in both normal and PASTE mode
- REPL remains stable after interrupt

## History

When compiled with filesystem support (default), command history is saved to `~/.v4_history`.

Features:
- Up/Down arrows to navigate history
- Maximum 1000 entries
- Persists across sessions

## Compiler Flags

The project is compiled with:
- `-fno-exceptions` - No exception handling
- `-fno-rtti` - No runtime type information

This makes it suitable for embedded systems and resource-constrained environments.

## Binary Size

v4-repl is designed to be compact and suitable for embedded systems:

| Build Type | Size | Use Case |
|------------|------|----------|
| Debug | 224KB | Development (includes debug symbols) |
| Debug (stripped) | 91KB | Testing without debugger |
| Release (stripped) | **91KB** | **Production (recommended)** |
| MinSizeRel (stripped) | ~85KB | Size-critical embedded systems |

**Note**: Size increased from previous 67KB due to V4/V4-front feature additions (31 new opcodes, RECURSE, composite words, etc.). The binary remains compact and suitable for embedded systems.

Check binary size:
```bash
make size         # Quick check
make size-report  # Detailed analysis with recommendations
```

The compact size makes v4-repl ideal for:
- Embedded devices (ESP32, STM32, etc.)
- IoT applications
- Resource-constrained environments
- Quick deployment without large dependencies

## Platform-Independent Library (libv4repl)

V4-REPL provides a **platform-independent C API** that can be integrated into any platform, including embedded systems.

### Library API

The core REPL functionality is available as a C library (`libv4repl`) with a simple API:

```c
#include "v4repl/repl.h"

// Create REPL context
V4ReplConfig config = {
    .vm = vm,
    .front_ctx = compiler_ctx,
    .line_buffer_size = 512,
};
V4ReplContext *repl = v4_repl_create(&config);

// Process a line of input
v4_err err = v4_repl_process_line(repl, "2 3 +");

// Display stack
v4_repl_print_stack(repl);  // Prints: " ok [1]: 5\n"

// Cleanup
v4_repl_destroy(repl);
```

### Embedded Systems Integration

For embedded platform implementations, see [V4-ports](https://github.com/kirisaki/V4-ports):

- **ESP32-C6 REPL Demo** - Complete working example with USB Serial/JTAG and GPIO LED control
- **Platform-specific HAL** - Hardware abstraction layer implementations
- **Build instructions** - Integration guides for various MCU platforms

The ESP32-C6 example demonstrates:
- UART-based interactive Forth REPL
- ~23KB RAM usage (configurable)
- Line editing with backspace support
- Ctrl+C interrupt handling
- GPIO control integration

## Project Structure

```
V4-repl/
├── CMakeLists.txt          # Build configuration
├── Makefile                # Convenient build targets
├── README.md               # This file
├── .clang-format           # Code formatting rules
├── .gitignore
├── include/
│   └── v4repl/
│       └── repl.h          # Platform-independent REPL API
├── src/
│   ├── repl.c              # REPL library implementation
│   ├── main.cpp            # Linux REPL entry point
│   ├── repl.hpp            # Linux REPL class interface
│   ├── repl.cpp            # Linux REPL implementation
│   ├── meta_commands.hpp   # Meta-commands interface
│   └── meta_commands.cpp   # Meta-commands implementation
├── examples/
│   └── esp32c6/
│       ├── main.c          # ESP32-C6 REPL example
│       └── README.md       # ESP32-C6 integration guide
├── docs/
│   ├── user-guide.md       # Complete user guide
│   ├── meta-commands.md    # Meta-commands reference
│   └── recurse-implementation.md  # RECURSE implementation
├── test_smoke.sh           # Smoke test script
├── size_report.sh          # Binary size analysis script
└── build/                  # Build directory (generated)
```

## License

Same as V4 project.
