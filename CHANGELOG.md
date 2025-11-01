# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Build configuration for V4-front library integration via CMake
- Support for compiling Forth source code to bytecode
- Vendored V4, V4-front, and V4-repl libraries for standalone builds
- Static linking of libv4, libv4front, and libv4repl
- Rust FFI bindings for libv4repl C API with safe wrapper

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

[Unreleased]: https://github.com/kirisaki/v4-cli/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/kirisaki/v4-cli/releases/tag/v0.1.0
