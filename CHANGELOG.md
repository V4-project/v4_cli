# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2025-11-05

### Added
- **New debugging meta-commands for REPL**
  - `.stack` - Display data stack and return stack contents
  - `.rstack` - Display return stack with call trace
  - `.dump [addr] [len]` - Hexdump memory at specified address (default: 256 bytes)
  - `.see <word_idx>` - Show word bytecode disassembly
- **V4-link protocol query commands**
  - `QueryStack (0x30)` - Query stack state
  - `QueryMemory (0x40)` - Query memory dump
  - `QueryWord (0x50)` - Query word information
- Serial protocol methods: `query_stack()`, `query_memory()`, `query_word()`
- Response data field for query command payloads
- CLI error variant for user input validation errors

### Changed
- Updated `.help` command to include new debugging meta-commands
- Enhanced `Response` struct with `data` field for query responses

## [0.3.0] - 2025-11-04

### Added

- `v4 exec <file> --port <PORT>` command for executing Forth source files on device
  - Compiles Forth source code (.fs files) using V4-front
  - Sends word definitions to device and registers them in VM
  - Executes main bytecode if present
  - Supports `--timeout` flag for configuring execution timeout
- `v4 exec --repl` flag to enter REPL after file execution
  - Preserves compiler context from file execution
  - Enables immediate testing of defined words
  - Seamless transition from batch execution to interactive mode
- `v4 repl --no-reset` flag to skip VM reset on REPL startup
  - Preserves existing word definitions on device
  - Useful for connecting to already-running VMs
  - Displays warning about empty compiler context
- Example Forth library: `examples/nanoc6/led.fs`
  - LED control words for NanoC6 (ESP32-C6) board
  - Provides `led-on`, `led-off`, `led-blink`, `led-blink-fast`, `led-blink-slow`
  - Demonstrates GPIO syscall usage (SYS 0, SYS 1, SYS 34)

### Changed

- REPL startup message now shows version from `CARGO_PKG_VERSION`
- Improved REPL help text with `.reset` command documentation

## [0.2.0] - 2025-11-02

### Added

- Build configuration for V4-front library integration via CMake
- Support for compiling Forth source code to bytecode
- Vendored V4, V4-front, and V4-repl libraries for standalone builds
- Static linking of libv4, libv4front, and libv4repl
- Rust FFI bindings for V4-front compiler API with safe wrapper
- `v4 repl --port <PORT>` command for interactive Forth REPL over serial connection
- Meta-commands in REPL: `.help`, `.ping`, `.reset`, `.exit`
- Persistent word definitions across REPL lines
- Command history with rustyline (arrow keys navigation)

### Changed

- **BREAKING**: Extended V4-link protocol EXEC response to variable-length format
  - EXEC response now includes word indices registered by device VM
  - Response format: `[STX][LEN_L][LEN_H][ERR_CODE][WORD_COUNT][WORD_IDX...]...[CRC8]`
  - Required for proper word definition synchronization between host compiler and device VM
- Updated REPL to use device-returned word indices instead of auto-incrementing
- Fixed word definition functionality in REPL (previously would timeout on word usage)
- Updated CLI help text to include REPL command

### Fixed

- Fixed serial response reading to support variable-length frames
  - Previously hardcoded to read exactly 5 bytes (old protocol format)
  - Now reads LENGTH field from frame header to determine total frame size
  - Enables proper handling of EXEC responses with word indices
  - Critical fix for REPL word definition support

### Infrastructure

- Add comprehensive CI/CD workflows (test, lint, security, release)
- Add cargo-deny configuration for dependency management
- Add multi-platform release builds (Linux, macOS, Windows, ARM64)
- Add security audit workflow with cargo-audit

## [0.1.0] - 2025-01-XX

### Added

- Initial release of v4 CLI tool
- V4-link protocol implementation (CRC-8, frame encoding/decoding)
- `v4 push` command for deploying bytecode to devices
- `v4 ping` command for checking device connection
- `v4 reset` command for resetting VM state
- Progress bar for bytecode deployment
- Configurable timeout for all commands
- `--detach` flag for fire-and-forget deployment
- Support for ESP32-C6, CH32V203, and other V4-enabled devices

### Technical Details

- CRC-8 checksum with polynomial 0x07
- Little-endian frame encoding
- Default baud rate: 115200
- Maximum payload size: 512 bytes
- Default timeout: 5 seconds

[Unreleased]: https://github.com/kirisaki/v4-cli/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/kirisaki/v4-cli/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/kirisaki/v4-cli/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/kirisaki/v4-cli/releases/tag/v0.1.0
