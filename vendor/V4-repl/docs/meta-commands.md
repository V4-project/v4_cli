# Meta-Commands Reference

Meta-commands are special REPL commands that start with `.` (dot) and provide inspection, control, and debugging capabilities.

## Overview

Meta-commands are not part of the V4 Forth language itself - they are REPL-specific features that help you interact with and inspect the VM state.

### Quick Reference

| Command | Purpose | Example |
|---------|---------|---------|
| `.help` | Show comprehensive help | `.help` |
| `.words` | List all defined words | `.words` |
| `.stack` | Show detailed stack view | `.stack` |
| `.reset` | Reset VM and context | `.reset` |
| `.memory` | Show memory usage | `.memory` |
| `.version` | Show version info | `.version` |

## Command Details

### `.help`

**Purpose**: Display comprehensive help message covering all REPL features.

**Syntax**:
```forth
.help
```

**Description**:
Shows a help screen with:
- List of all meta-commands
- PASTE mode usage
- Control key bindings
- Basic Forth commands
- Word definition syntax

**Example**:
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

PASTE mode (multi-line input):
  <<<        - Enter PASTE mode for multi-line definitions
  >>>        - Exit PASTE mode and compile buffered input

Control:
  Ctrl+D     - Exit REPL
  Ctrl+C     - Interrupt execution (if supported)
  bye / quit - Exit REPL

...
```

**Notes**:
- Does not affect stack or VM state
- Can be called at any time
- Provides quick reference without leaving the REPL

---

### `.words`

**Purpose**: List all user-defined words currently registered in the compiler context.

**Syntax**:
```forth
.words
```

**Description**:
Displays all words you've defined with `: NAME ... ;` syntax. Built-in V4 words are not shown (only user definitions).

**Example 1**: With definitions
```forth
v4> : DOUBLE 2 * ;
 ok

v4> : SQUARE DUP * ;
 ok

v4> : CUBE DUP DUP * * ;
 ok

v4> .words
Defined words (3):
  DOUBLE
  SQUARE
  CUBE
 ok
```

**Example 2**: Without definitions
```forth
v4> .words
No words defined.
 ok
```

**Notes**:
- Words are listed in the order they were defined
- After `.reset`, `.words` will show "No words defined"
- Built-in words (like `+`, `-`, `DUP`, etc.) are not shown

**Use Cases**:
- Check what words you've defined
- Verify a word was successfully defined
- Debug name conflicts
- Document your session

---

### `.stack`

**Purpose**: Display detailed stack contents with both decimal and hexadecimal representations.

**Syntax**:
```forth
.stack
```

**Description**:
Shows each stack value with:
- Position index (0 = bottom, increasing to top)
- Decimal value
- Hexadecimal value (useful for bitwise operations)

**Example 1**: Multiple values
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

**Example 2**: Empty stack
```forth
v4> .reset
VM and compiler context reset.
 ok

v4> .stack
Data Stack (depth: 0):
  <empty>

Return Stack: <not yet implemented>
 ok
```

**Example 3**: Hexadecimal values
```forth
v4> 255 256
 ok [2]: 255 256

v4> .stack
Data Stack (depth: 2):
  [0]: 255 (0x000000FF)
  [1]: 256 (0x00000100)

Return Stack: <not yet implemented>
 ok [2]: 255 256
```

**Notes**:
- Does not modify the stack (non-destructive)
- Return stack display requires V4-core API enhancement
- Useful for debugging stack manipulation
- Hexadecimal format helpful for bitwise operations

**Use Cases**:
- Debug stack manipulation errors
- Verify stack state before complex operations
- Check hexadecimal values for bit operations
- Understand stack depth issues

---

### `.reset`

**Purpose**: Reset the VM and compiler context to initial state.

**Syntax**:
```forth
.reset
```

**Description**:
Performs a complete reset:
- Clears data stack
- Clears return stack
- Removes all user-defined words
- Resets compiler context
- VM state returns to initial configuration

**Example**:
```forth
v4> 10 20 30
 ok [3]: 10 20 30

v4> : TEST 42 ;
 ok

v4> .words
Defined words (1):
  TEST
 ok

v4> .reset
VM and compiler context reset.
 ok

v4> .stack
Data Stack (depth: 0):
  <empty>

Return Stack: <not yet implemented>
 ok

v4> .words
No words defined.
 ok
```

**Notes**:
- **Destructive operation** - cannot be undone
- History is preserved (command history remains)
- Built-in words remain available
- Does not exit the REPL

**Use Cases**:
- Start fresh after errors
- Clear confusing state
- Begin a new session without restarting
- Recover from stack overflow/underflow
- Remove conflicting word definitions

**Warning**:
⚠️ This command permanently deletes all your word definitions. There is no undo. Use with caution in important sessions.

---

### `.memory`

**Purpose**: Display memory usage statistics for the VM and REPL.

**Syntax**:
```forth
.memory
```

**Description**:
Shows memory usage information including:
- VM memory size (requires V4-core API)
- Data stack depth
- Return stack depth (requires V4-core API)
- Number of registered words

**Example**:
```forth
v4> 10 20 30
 ok [3]: 10 20 30

v4> : TEST 42 ;
 ok

v4> : DOUBLE 2 * ;
 ok

v4> .memory
Memory usage information:
  VM memory size: (not yet available from V4-core)
  Data stack depth: 3
  Return stack depth: (API not yet available)
  Registered words: 2
 ok [3]: 10 20 30
```

**Notes**:
- Current implementation has limited info (V4-core API limitations)
- Does not modify any state
- Useful for monitoring resource usage
- Future versions will show more detailed memory breakdown

**Use Cases**:
- Monitor stack usage
- Check number of defined words
- Debug memory-related issues
- Profile REPL usage

**Future Enhancements**:
The following information will be added when V4-core provides the necessary APIs:
- Total VM memory size
- Used vs. free memory
- Return stack depth
- Memory fragmentation info
- Per-word memory usage

---

### `.version`

**Purpose**: Display version information for the REPL and its components.

**Syntax**:
```forth
.version
```

**Description**:
Shows:
- REPL version
- Component versions (V4-core, V4-front, linenoise)
- Build configuration (C++ standard, compiler flags, features)

**Example**:
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

**Notes**:
- Component versions require API additions to V4-core and V4-front
- Build configuration reflects compile-time settings
- Useful for bug reports and compatibility checking

**Use Cases**:
- Check REPL version
- Verify build configuration
- Include in bug reports
- Confirm feature availability

---

## Meta-Command Behavior

### Non-Destructive

Most meta-commands are non-destructive (don't modify state):
- `.help` - Read-only
- `.words` - Read-only
- `.stack` - Read-only
- `.memory` - Read-only
- `.version` - Read-only

### Destructive

Only one meta-command modifies state:
- `.reset` - **Destructive** (clears everything)

### Error Handling

Unknown meta-commands show an error:

```forth
v4> .unknown
Unknown meta-command: .unknown
Type .help for available commands
 ok
```

### Case Sensitivity

Meta-commands are case-sensitive and must be lowercase:

```forth
v4> .HELP
Unknown meta-command: .HELP
Type .help for available commands
 ok

v4> .help
( works correctly )
```

### Whitespace Handling

Leading/trailing whitespace is ignored:

```forth
v4>    .help
( works correctly )
```

---

## PASTE Mode (Special Syntax)

While not meta-commands in the strict sense, the PASTE mode markers use similar syntax:

### `<<<` - Enter PASTE Mode

**Syntax**:
```forth
<<<
```

**Effect**: Changes prompt from `v4>` to `...` and begins accumulating input lines.

**Example**:
```forth
v4> <<<
Entering PASTE mode. Type '>>>' to compile and execute.
... : SQUARE
...   DUP *
... ;
... >>>
 ok
```

### `>>>` - Exit PASTE Mode

**Syntax**:
```forth
>>>
```

**Effect**: Compiles and executes all buffered lines, then returns to normal mode.

**Notes**:
- Can be interrupted with `Ctrl+C`
- Empty buffer shows warning
- Errors in buffered code are reported normally

---

## Integration with Forth Code

Meta-commands are processed before Forth compilation, so they:
- Cannot be used inside word definitions
- Don't affect the compiled bytecode
- Are purely REPL-level features

**Invalid** (won't work):
```forth
v4> : TEST .stack 42 ;
Error: unknown token at line 1, column 8
```

**Valid** (correct usage):
```forth
v4> : TEST 42 ;
 ok

v4> .stack
Data Stack (depth: 0):
  <empty>
 ok

v4> TEST
 ok [1]: 42

v4> .stack
Data Stack (depth: 1):
  [0]: 42 (0x0000002A)
 ok [1]: 42
```

---

## Future Meta-Commands

Planned additions for future versions:

### `.debug` - Debug Mode
Toggle verbose debugging output

### `.trace` - Execution Tracing
Show each instruction as it executes

### `.breakpoint` - Set Breakpoints
Set breakpoints for word execution

### `.save` / `.load` - Session Management
Save and load REPL sessions

### `.export` - Export Definitions
Export word definitions to file

---

## See Also

- [User Guide](user-guide.md) - Complete REPL usage guide
- [README](../README.md) - Project overview and setup
- V4 Forth Language Reference (coming soon)

## Getting Help

If you encounter issues with meta-commands:
1. Check syntax (lowercase, starts with `.`)
2. Use `.help` for quick reference
3. Check this documentation for detailed behavior
4. Report bugs at: https://github.com/your-org/V4-repl/issues
