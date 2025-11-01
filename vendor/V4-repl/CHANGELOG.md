# Changelog

All notable changes to V4-repl will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- V4-hal C++17 CRTP HAL implementation support
  - Optional V4-hal integration via `V4_USE_V4HAL` option (default: OFF)
  - Automatic HAL library selection (v4_hal_wrapper or mock_hal)
  - Zero-cost abstraction benefits from C++17 CRTP architecture
  - Platform support: POSIX, ESP32, CH32V203
  - All 11 tests pass with V4-hal enabled

## [0.4.0] - 2025-10-31

### Added
- **Comprehensive unit tests for libv4repl API** using doctest framework
  - 11 test cases with 148 assertions covering all API functionality
  - Tests for basic operations, word definitions, stack preservation, RECURSE, and v0.3.x features
  - C++17-compatible test code (no designated initializers)
- **Full Windows support** for v4-repl executable
  - Simple line input with `std::getline()` on Windows (linenoise on Unix/macOS)
  - Custom history implementation using `std::vector`
  - Platform-specific history file paths (`%USERPROFILE%\.v4_history` on Windows)
  - Cross-platform build configuration with conditional compilation
- **Multi-platform CI testing** on Linux, macOS, and Windows
  - Matrix builds for all three platforms
  - Smoke tests and unit tests running on all platforms
  - Platform-specific dependency installation
- **Cross-platform test infrastructure**
  - Auto-detection of executable paths (handles `build/Debug/` on Windows)
  - Clear error messages when executable not found

### Changed
- **Updated for V4 v0.5.0 and V4-front v0.3.x compatibility**
  - Support for automatic frame pointer management in CALL instruction
  - Documentation for all 31 new opcodes and RECURSE keyword
  - Binary size increased from 67KB to 91KB (stripped release) due to new features
- **Enhanced documentation**
  - Comprehensive user guide with all v0.3.x features
  - RECURSE implementation examples (factorial, Fibonacci, GCD, etc.)
  - Platform integration guide pointing to V4-ports repository
  - Test results and feature documentation

### Fixed
- **VmConfig arena field initialization** to prevent undefined behavior
  - Zero-initialize VmConfig structure with `{0}`
  - Explicitly set `cfg.arena = nullptr`
  - Fixes macOS smoke test failures (InvalidArg error -16)
- **Windows compilation issues**
  - Added dummy `g_interrupted` flag for Windows (Ctrl+C not implemented)
  - Platform-specific includes (`signal.h`/`unistd.h` only on Unix)
  - MSVC-compatible code without POSIX-specific features
- **Cross-platform executable paths** in test scripts
  - Handles `build/`, `build/Debug/`, `build/Release/` locations
  - Fixes "command not found" errors (exit code 127) on Windows CI

### Notes
- Windows version has simplified input (no line editing features like Ctrl+A/E/K/U)
- Ctrl+C interrupt handling only available on Unix/macOS
- All core REPL functionality works identically across platforms

## [0.3.0] - Previous release
- Meta-commands (.words, .stack, .reset, .memory, .help, .version)
- PASTE mode for multi-line input
- Ctrl+C interrupt handling (Unix/macOS)
- Comprehensive documentation

## [0.2.0] - Previous release
- Persistent word definitions across lines
- Stack preservation
- Detailed error messages with position information

## [0.1.0] - Initial release
- Basic REPL functionality
- V4 VM integration
- V4-front compiler integration

[Unreleased]: https://github.com/kirisaki/V4-repl/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/kirisaki/V4-repl/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/kirisaki/V4-repl/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/kirisaki/V4-repl/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/kirisaki/V4-repl/releases/tag/v0.1.0
