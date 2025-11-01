# V4 Bytecode Examples

This directory contains example V4 bytecode files for testing the CLI tool with ESP32-C6 devices.

## LED Examples

All LED examples control the onboard LED (GPIO8 on ESP32-C6):

- **led_on.bin** - Turn LED on
- **led_off.bin** - Turn LED off
- **led_toggle.bin** - Toggle LED state once
- **led_blink.bin** - Blink LED continuously (basic pattern)
- **led_fast.bin** - Fast blinking pattern
- **led_medium.bin** - Medium speed blinking
- **led_slow.bin** - Slow blinking pattern
- **led_sos.bin** - SOS morse code pattern (... --- ...)

## Usage

```bash
# Turn LED on
v4 push examples/led_on.bin --port /dev/ttyACM0

# Start blinking
v4 push examples/led_blink.bin --port /dev/ttyACM0

# SOS pattern
v4 push examples/led_sos.bin --port /dev/ttyACM0

# Turn off when done
v4 push examples/led_off.bin --port /dev/ttyACM0
```

## Notes

- These examples require a V4-link enabled device (ESP32-C6, CH32V203, etc.)
- LED GPIO pin may differ depending on your hardware
- Some patterns run in loops - use `led_off.bin` or reset the device to stop
