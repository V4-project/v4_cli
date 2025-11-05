# V4 Device Test Scripts

Test scripts for V4 hardware integration.

## test_device.sh

Integration test suite for V4 devices (ESP32-C6, etc).

### Prerequisites

- V4 firmware flashed on target device
- v4_cli built (`cargo build --release`)
- Device connected via USB serial

### Usage

```bash
# Run tests with default port (/dev/ttyACM0)
./scripts/test_device.sh

# Specify custom port
./scripts/test_device.sh /dev/ttyUSB0
```

### Test Coverage

#### Phase 1: Basic Arithmetic
- Addition, subtraction, multiplication, division

#### Phase 2: Stack Operations
- DUP, SWAP, DROP, OVER, ROT

#### Phase 3: Word Definitions
- Simple word definitions
- Words with arithmetic operations
- Nested word calls

#### Phase 4: Meta Commands
- `.words` - List registered words
- `.reset` - Reset VM state

#### Phase 5: Stack Inspection
- `.stack` - Display data and return stack contents
- Test with empty stack, values, and after operations

#### Phase 6: Word Inspection
- `.see <word_id>` - Display word bytecode

#### Phase 7: Memory Inspection
- `.dump <addr> <len>` - Dump memory contents

#### Phase 8: Combined Workflow
- Word definition + execution + inspection

#### Phase 9: Error Handling
- Stack underflow
- Division by zero

#### Phase 10: Protocol Commands
- PING (connection check)
- RESET (VM reset)

### Notes

- Cannot run in CI (requires physical device)
- Tests run serially (~1-2 minutes total)
- Each test has 10-second timeout
