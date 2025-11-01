# V4 Bytecode Examples

This directory contains example V4 bytecode files for testing the CLI tool with ESP32-C6 devices.

## LED Examples

All LED examples control the onboard LED (GPIO8 on ESP32-C6):

- **led_on.v4b** - Turn LED on
- **led_off.v4b** - Turn LED off
- **led_toggle.v4b** - Toggle LED state once
- **led_blink.v4b** - Blink LED continuously (basic pattern)
- **led_fast.v4b** - Fast blinking pattern
- **led_medium.v4b** - Medium speed blinking
- **led_slow.v4b** - Slow blinking pattern
- **led_sos.v4b** - SOS morse code pattern (... --- ...)

## Usage

```bash
# Turn LED on
v4 push examples/led_on.v4b --port /dev/ttyACM0

# Start blinking
v4 push examples/led_blink.v4b --port /dev/ttyACM0

# SOS pattern
v4 push examples/led_sos.v4b --port /dev/ttyACM0

# Turn off when done
v4 push examples/led_off.v4b --port /dev/ttyACM0
```

## Notes

- These examples require a V4-link enabled device (ESP32-C6, CH32V203, etc.)
- LED GPIO pin may differ depending on your hardware
- Some patterns run in loops - use `led_off.bin` or reset the device to stop
