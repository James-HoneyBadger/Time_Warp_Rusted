# Raspberry Pi & IoT Guide

Time Warp Rusted includes built-in GPIO simulation and serial
communication for Raspberry Pi boards. Write hardware control programs in
any supported language, test them with the software simulator, then deploy
to real hardware.

---

## Supported Boards

| Board | GPIO Pins | WiFi | ADC |
|-------|-----------|------|-----|
| **Raspberry Pi Pico** | 26 | No | Yes |
| **Raspberry Pi Pico W** | 26 | Yes | Yes |
| **Raspberry Pi Zero** | 26 | No | No |
| **Raspberry Pi Zero 2W** | 26 | Yes | No |
| **Raspberry Pi 4** | 26 | Yes | No |
| **Raspberry Pi 5** | 26 | Yes | No |
| **Simulator** | 26 | Yes | Yes |

The **Simulator** board runs entirely in software — no hardware required.
It is the default and recommended starting point.

---

## Getting Started

### 1. Select a Board

Open the **🔌 Raspberry Pi** menu and select **Setup Board**. Choose your
board model from the list, or select **Simulator** to test without
hardware.

### 2. Open the IoT Panel

Toggle the IoT panel from **View → IoT Panel** or the Raspberry Pi menu.
The panel shows:

- Current board model and connection status
- Pin states (mode, digital value, PWM duty cycle)
- Serial port connection controls
- Activity log

### 3. Write GPIO Code

Use GPIO commands in your chosen language. Here is a BASIC example:

```basic
' Blink an LED on pin 25
PINMODE 25, "OUTPUT"

FOR I = 1 TO 10
    DIGITALWRITE 25, 1
    PRINT "LED ON"
    SLEEP 500

    DIGITALWRITE 25, 0
    PRINT "LED OFF"
    SLEEP 500
NEXT I
```

### 4. Run and Observe

Press **F5** to run. Watch pin states change in the IoT panel. On the
Simulator, you can click pins to toggle input states interactively.

---

## GPIO Commands by Language

### BASIC

```basic
PINMODE pin, mode       ' "INPUT", "OUTPUT", "PWM"
DIGITALWRITE pin, value ' 1 (HIGH) or 0 (LOW)
DIGITALREAD(pin)        ' Returns 1 or 0
PWMWRITE pin, duty      ' 0.0 to 1.0
ANALOGWRITE pin, value  ' Analog output
SERVOWRITE pin, angle   ' 0 to 180 degrees
GPIORESET               ' Reset all pins
```

### Logo

```logo
PINMODE pin mode        ; "INPUT" "OUTPUT" "PWM"
DIGITALWRITE pin value  ; 1 or 0
SETPIN pin value        ; Alias for DIGITALWRITE
DIGITALREAD pin         ; Returns 1 or 0
READPIN pin             ; Alias for DIGITALREAD
PWMWRITE pin duty       ; 0.0 to 1.0
GPIORESET               ; Reset all pins
```

### C

```c
pin_mode(25, OUTPUT);
digital_write(25, HIGH);
int val = digital_read(25);
pwm_write(25, 0.5);
gpio_reset();
```

### Pascal

```pascal
PinMode(25, 'OUTPUT');
DigitalWrite(25, 1);
value := DigitalRead(25);
PwmWrite(25, 0.5);
GpioReset;
```

### Forth

```forth
25 OUTPUT PINMODE
25 1 DIGITALWRITE
25 DIGITALREAD .
25 0.5 PWMWRITE
GPIORESET
```

---

## Pin Modes

| Mode | Description | Use Case |
|------|-------------|----------|
| **Input** | Read digital signals | Buttons, switches, sensors |
| **Output** | Write digital signals | LEDs, relays, buzzers |
| **PWM** | Pulse-width modulation | LED dimming, motor speed |
| **I2C** | Inter-IC communication | Sensors, displays |
| **SPI** | Serial Peripheral Interface | SD cards, fast peripherals |
| **Unset** | Pin not configured (default) | — |

---

## Serial Communication

For connecting to real Raspberry Pi hardware:

1. Connect the board via USB
2. Open the IoT panel
3. Select the serial port from the dropdown (auto-detected)
4. Set the baud rate (default: 115200)
5. Click **Connect**

The serial connection enables:
- Sending GPIO commands to real hardware
- Receiving sensor data
- Real-time pin state monitoring

---

## Simulator Features

The Simulator board provides a complete GPIO testing environment:

- **All 26 GPIO pins** with configurable modes
- **Interactive pin toggling** — click input pins to simulate button
  presses or sensor signals
- **PWM visualization** — see duty cycle values in the IoT panel
- **ADC simulation** — analog read returns simulated values
- **WiFi flag** — always reports as available
- **Activity log** — every GPIO operation is logged for debugging

### Using the Simulator

1. Select **Simulator** as your board
2. Write and run your GPIO program
3. Watch pin states update in the IoT panel
4. Click input pins to simulate external signals
5. Check the activity log for command history

---

## Example Projects

The `Examples/` directory includes 30 Raspberry Pi GPIO projects
organized by difficulty:

### Beginner
- LED blink patterns
- Button input reading
- PWM LED dimming

### Intermediate
- Traffic light controller
- Servo motor control
- Temperature sensor reading

### Advanced
- Multi-sensor data logging
- Motor speed control with feedback
- I2C device communication

Load examples from the **Examples** panel in the left sidebar under
the Raspberry Pi category.

---

## Tips

1. **Start with the Simulator** — get your logic right before connecting
   real hardware.

2. **Check pin modes** — always set `PINMODE` before reading or writing
   a pin.

3. **Use the activity log** — it records every GPIO operation, making it
   easy to debug timing and sequence issues.

4. **PWM values are 0.0–1.0** — not 0–255 like some Arduino libraries.

5. **Reset pins between runs** — call `GPIORESET` at the start of your
   program or press Stop (F6) which resets automatically.

6. **Serial baud rate** — make sure both ends (computer and board) use
   the same baud rate (115200 by default).
