# ESP32 Integration Guide

For ESP32/ESP32-C6 REPL implementations, please refer to **[V4-ports](https://github.com/kirisaki/V4-ports)**.

## Why V4-ports?

Platform-specific implementations are maintained in the V4-ports repository to:
- Centralize platform-specific code (HAL, drivers, build systems)
- Avoid duplication between V4-repl and platform ports
- Provide working examples with actual hardware integration
- Maintain consistency across different MCU platforms

## Available Examples

### ESP32-C6 REPL Demo
**Location**: `V4-ports/esp32c6/examples/v4-repl-demo/`

**Features**:
- Interactive Forth REPL over USB Serial/JTAG
- GPIO LED control (GPIO7)
- ~16KB arena memory (configurable)
- Line editing with backspace
- Ctrl+C interrupt handling
- Compile-time word definitions

**Quick Start**:
```bash
cd V4-ports/esp32c6/examples/v4-repl-demo
idf.py build flash monitor
```

See the [V4-ports README](https://github.com/kirisaki/V4-ports#esp32-c6-port) for detailed build instructions.

## Integration Approaches

V4-ports demonstrates two integration approaches:

### 1. Direct Integration (v4-repl-demo)
Directly use V4 VM and V4-front APIs for maximum control:

```c
#include "v4/vm_api.h"
#include "v4front/compile.h"

// Initialize VM
VmConfig config = { .mem = arena_buf, .mem_size = ARENA_SIZE };
struct Vm *vm = vm_create(&config);

// Initialize compiler
V4FrontContext *ctx = v4front_context_create();

// Compile and execute Forth code
V4FrontBuf buf = {0};
V4FrontError error = {0};
v4front_compile_with_context_ex(ctx, "2 3 +", &buf, &error);

// Register and execute compiled words...
```

**Use when**: You need full control over compilation and execution flow.

### 2. Library Integration (libv4repl)
Use the platform-independent libv4repl library (future enhancement):

```c
#include "v4repl/repl.h"

// Create REPL with existing VM and compiler
V4ReplConfig config = { .vm = vm, .front_ctx = ctx, .line_buffer_size = 256 };
V4ReplContext *repl = v4_repl_create(&config);

// Process input line
v4_repl_process_line(repl, input_line);
v4_repl_print_stack(repl);
```

**Use when**: You want standard REPL behavior with minimal code.

## Platform Requirements

To port V4-REPL to a new platform, you need:

1. **V4 HAL Implementation** - GPIO, UART, Timer, System APIs
2. **UART/Serial Driver** - For console input/output
3. **Memory** - Minimum 16KB RAM for VM + compiler
4. **Build System** - CMake or platform-specific toolchain

See the [V4 HAL API documentation](https://github.com/kirisaki/V4/blob/main/docs/hal-api.md) for HAL interface specification.

## Contributing

Platform ports are welcome! Please submit to [V4-ports](https://github.com/kirisaki/V4-ports) rather than V4-repl to keep platform-specific code centralized.

---

**Related Projects**:
- [V4](https://github.com/kirisaki/V4) - VM core implementation
- [V4-front](https://github.com/kirisaki/V4-front) - Forth compiler frontend
- [V4-ports](https://github.com/kirisaki/V4-ports) - Platform HAL implementations
