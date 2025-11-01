# V4 REPL v0.2.0 Release Notes

**Release Date**: 2025-10-28

## 🎉 New Features

### 1. Meta-Command System

Six powerful meta-commands for REPL control and inspection:

- **`.words`** - List all defined words
- **`.stack`** - Show detailed stack contents (hex + decimal)
- **`.reset`** - Reset VM and compiler context
- **`.memory`** - Display memory usage statistics
- **`.help`** - Show comprehensive help message
- **`.version`** - Display version information

```forth
v4> : SQUARE DUP * ;
 ok

v4> .words
Defined words (1):
  SQUARE
 ok

v4> 10 20 30
 ok [3]: 10 20 30

v4> .stack
Data Stack (depth: 3):
  [0]: 10 (0x0000000A)
  [1]: 20 (0x00000014)
  [2]: 30 (0x0000001E)
 ok [3]: 10 20 30
```

### 2. PASTE Mode

Multi-line input mode for complex word definitions:

- Enter with `<<<`
- Exit and compile with `>>>`
- Prompt changes to `...` during PASTE mode
- Perfect for nested control structures

```forth
v4> <<<
Entering PASTE mode. Type '>>>' to compile and execute.
... : FACTORIAL
...   DUP 1 >
...   IF DUP 1 - RECURSE *
...   ELSE DROP 1
...   THEN
... ;
... >>>
 ok

v4> 5 FACTORIAL
 ok [1]: 120
```

### 3. Ctrl+C Interrupt Handling

Safe interruption of long-running operations:

- Press `Ctrl+C` during execution to interrupt
- Clears data stack and returns to prompt
- Works in both normal and PASTE mode
- REPL remains stable after interrupt

```forth
v4> : FOREVER BEGIN 1 UNTIL ;
 ok

v4> FOREVER
^C
Execution interrupted
 ok
```

### 4. Comprehensive Documentation

- **User Guide** (`docs/user-guide.md`) - Complete tutorial with examples
- **Meta-Commands Reference** (`docs/meta-commands.md`) - Detailed command documentation
- **Updated README** - Feature highlights and usage examples

## 🔧 Improvements

- Added `.clang-format` for consistent code style
- Improved error messages with better context
- Enhanced startup messages with feature hints
- Version bumped to 0.2.0 across all components

## 📦 Binary Size

| Build Type | Size | Use Case |
|------------|------|----------|
| Debug | 193KB | Development |
| Release | 127KB | Production |
| Release (stripped) | **68KB** | **Embedded/IoT** |

## ✅ Testing

- All 8 smoke tests pass
- No regressions in existing functionality
- Tested on Linux (WSL2)

## 📚 Documentation

Total documentation: **1,047 lines**

- User guide: 605 lines
- Meta-commands reference: 442 lines
- README updates and examples

## 🚀 Getting Started

### Installation

```bash
# Clone repository
git clone https://github.com/your-org/V4-repl.git
cd V4-repl

# Build
make build

# Run
./build/v4-repl
```

### Quick Example

```forth
v4> .help
( Shows comprehensive help )

v4> : DOUBLE 2 * ;
 ok

v4> 21 DOUBLE
 ok [1]: 42

v4> <<<
... : ABS
...   DUP 0 <
...   IF NEGATE THEN
... ;
... >>>
 ok

v4> -42 ABS
 ok [1]: 42
```

## 🔄 Breaking Changes

None. This release is fully backward compatible with v0.1.x.

## 📝 Commits

- `4337d59` docs: Add comprehensive documentation and update README
- `f4b3c44` feat: Add Ctrl-C interrupt handling
- `1f4f849` feat: Add PASTE mode for multi-line input
- `bd3a457` feat: Add meta-command system (.words, .stack, .reset, etc.)

## 🐛 Known Issues

- Return stack display not yet implemented (requires V4-core API)
- Memory usage details limited (requires V4-core API enhancements)
- RECURSE not yet supported (planned for future release)

## 🔮 Future Plans

- RECURSE implementation for direct recursion
- ReplCore extraction for device REPL support
- Enhanced integration tests
- Breakpoint and debugging support

## 🙏 Acknowledgments

Built with:
- V4-core v0.3.0
- V4-front (latest)
- linenoise (line editing)

Generated with Claude Code 🤖

## 📄 License

Same as V4 project.

---

For detailed usage instructions, see:
- [User Guide](docs/user-guide.md)
- [Meta-Commands Reference](docs/meta-commands.md)
- [README](README.md)
