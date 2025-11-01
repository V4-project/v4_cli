# v4 CLI

CLI tool for deploying bytecode to V4 VM devices via the V4-link protocol over USB Serial.

## Features

- **Deploy bytecode** to V4 VM devices (`v4 push`)
- **Check connection** to devices (`v4 ping`)
- **Reset VM** state (`v4 reset`)
- Progress bar for bytecode deployment
- Configurable timeout
- Works with ESP32-C6, CH32V203, and other V4-enabled devices

## Installation

### From source

```bash
cargo install --path .
```

### Build release binary

```bash
cargo build --release
# Binary will be in target/release/v4
```

## Usage

### Push bytecode to device

```bash
v4 push app.v4b --port /dev/ttyACM0
v4 push app.v4b --port /dev/ttyACM0 --timeout 10
v4 push app.v4b --port /dev/ttyACM0 --detach  # Don't wait for response
```

### Check device connection

```bash
v4 ping --port /dev/ttyACM0
```

### Reset VM

```bash
v4 reset --port /dev/ttyACM0
```

### Get help

```bash
v4 --help
v4 push --help
```

## V4-link Protocol

The V4-link protocol is a simple frame-based protocol for transferring bytecode to V4 VM devices over serial.

### Frame Format

```
[STX][LEN_L][LEN_H][CMD][DATA...][CRC8]

- STX:   0xA5 (start marker)
- LEN_L: Payload length low byte (little-endian u16)
- LEN_H: Payload length high byte
- CMD:   Command code
- DATA:  Payload (0-512 bytes)
- CRC8:  Checksum (polynomial 0x07, init 0x00)
```

### Commands

- `0x10` - EXEC: Execute bytecode
- `0x20` - PING: Connection check
- `0xFF` - RESET: VM reset

### Response Format

```
[STX][0x01][0x00][ERR_CODE][CRC8]

Error codes:
- 0x00 OK
- 0x01 ERROR
- 0x02 INVALID_FRAME
- 0x03 BUFFER_FULL
- 0x04 VM_ERROR
```

## Development

### Run tests

```bash
cargo test
```

### Build documentation

```bash
cargo doc --open
```

## Examples

The `examples/` directory contains sample V4 bytecode files for testing with ESP32-C6 devices:

```bash
# Turn LED on
v4 push examples/led_on.v4b --port /dev/ttyACM0

# Start blinking pattern
v4 push examples/led_blink.v4b --port /dev/ttyACM0

# SOS morse code
v4 push examples/led_sos.v4b --port /dev/ttyACM0

# Turn LED off
v4 push examples/led_off.v4b --port /dev/ttyACM0
```

See `examples/README.md` for full list of available examples.

For the Python reference implementation, see `../V4-ports/esp32c6/examples/v4-link-demo/host/v4_link_send.py`.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
