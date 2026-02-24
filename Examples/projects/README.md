# Raspberry Pi Hardware Projects

Real-world GPIO projects for three Raspberry Pi boards, written in
BASIC and Logo using Time Warp Rusted's IoT commands.

## Boards

| Board | Directory | GPIO Pins | Special Features |
|-------|-----------|-----------|------------------|
| Raspberry Pi Pico | `pico/` | GP0–GP25 (26) | ADC (analog input), PWM |
| Raspberry Pi Zero | `zero/` | GPIO0–GPIO27 (28) | PWM, I2C, SPI |
| Raspberry Pi Zero 2 W | `zero2w/` | GPIO0–GPIO27 (28) | PWM, I2C, SPI, **WiFi** |

## GPIO Commands (BASIC)

```basic
PINMODE pin, mode          ' mode: INPUT, OUTPUT, PWM
DIGITALWRITE pin, value    ' value: HIGH/1 or LOW/0
var = DIGITALREAD(pin)     ' read a digital input pin
PWMWRITE pin, duty         ' duty: 0-255 (or 0.0-1.0)
SERVOWRITE pin, angle      ' angle: 0-180 degrees
GPIORESET                  ' reset all pins to default
```

## GPIO Commands (Logo)

```logo
PINMODE pin mode            ; mode: INPUT, OUTPUT, PWM
DIGITALWRITE pin value      ; value: 1 or 0
READPIN pin                 ; read a digital input pin
PWMWRITE pin duty           ; duty: 0-255 (or 0.0-1.0)
GPIORESET                   ; reset all pins
```

---

## Pico Projects

| # | File | Description | Hardware |
|---|------|-------------|----------|
| 01 | `01_blink_led.bas` | Blink an LED on/off | 1 LED |
| 02 | `02_button_led.bas` | Button controls an LED | 1 button, 1 LED |
| 03 | `03_led_fader.bas` | PWM breathing effect | 1 LED |
| 04 | `04_traffic_light.bas` | UK traffic light sequence | 3 LEDs (R/A/G) |
| 05 | `05_analog_sensor.bas` | Read potentiometer (ADC) | 1 potentiometer |
| 06 | `06_servo_sweep.bas` | Sweep a servo motor | 1 micro servo |
| 07 | `07_led_chaser.bas` | Knight Rider LED patterns | 6 LEDs |
| 08 | `08_led_dice.bas` | Electronic dice with button | 7 LEDs, 1 button |
| 09 | `09_temp_monitor.bas` | Temperature alert system (ADC) | TMP36 sensor, 3 LEDs |
| 10 | `10_turtle_robot.logo` | Logo turtle drives a robot | 2 DC motors, H-bridge |

**Pico-exclusive:** Projects 05 and 09 use the Pico's analog-to-digital
converter (ADC) — not available on Pi Zero boards.

---

## Zero Projects

| # | File | Description | Hardware |
|---|------|-------------|----------|
| 01 | `01_blink_led.bas` | Blink an LED | 1 LED |
| 02 | `02_button_led.bas` | Button controls LED | 1 button, 1 LED |
| 03 | `03_pwm_fader.bas` | PWM breathing effect | 1 LED |
| 04 | `04_traffic_light.bas` | Pedestrian crossing sequence | 3 LEDs |
| 05 | `05_servo_controller.bas` | Servo with gauge positions | 1 micro servo |
| 06 | `06_binary_counter.bas` | Count 0–15 in binary | 4 LEDs |
| 07 | `07_reaction_timer.bas` | Reaction speed game | 1 LED, 1 button |
| 08 | `08_simon_says.bas` | Simon memory game | 4 LEDs, 4 buttons |
| 09 | `09_led_chaser.bas` | 5 LED patterns (8 LEDs) | 8 LEDs |
| 10 | `10_logo_gpio_art.logo` | Logo art with GPIO LEDs | 4 LEDs |

---

## Zero 2 W Projects

| # | File | Description | Hardware |
|---|------|-------------|----------|
| 01 | `01_blink_led.bas` | Blink an LED | 1 LED |
| 02 | `02_button_debounce.bas` | Button with edge detection | 1 button, 1 LED |
| 03 | `03_rgb_colour_mixer.bas` | PWM RGB LED rainbow | 1 RGB LED |
| 04 | `04_smart_traffic_light.bas` | Traffic + pedestrian signals | 5 LEDs, 1 button |
| 05 | `05_robotic_arm.bas` | 3-servo pick-and-place arm | 3 servos |
| 06 | `06_environment_monitor.bas` | Temp/humidity dashboard | 3 LEDs, buzzer |
| 07 | `07_vu_meter.bas` | 8-LED bar graph VU meter | 8 LEDs |
| 08 | `08_smart_home_lights.bas` | Automated home lighting scenes | 4 LEDs, sensor |
| 09 | `09_morse_code.bas` | Morse code LED + buzzer | 1 LED, 1 buzzer |
| 10 | `10_drawing_robot.logo` | Logo drawing robot with pen | 2 motors, 1 servo |

**Zero 2 W advantage:** WiFi onboard enables remote monitoring, cloud
data logging, and network-based control for projects 06 and 08.

---

## Difficulty Guide

- **01–03** — Beginner: Single components, basic I/O
- **04–06** — Intermediate: Multiple components, timing logic
- **07–09** — Advanced: Games, algorithms, sensor processing
- **10** — Expert: Multi-component systems, Logo + GPIO integration

## Getting Started

1. Open Time Warp Rusted
2. Open the **IoT Panel** (View → IoT Panel)
3. Select your board from the board selector
4. Load a project file
5. Click **Run** — GPIO commands appear in the IoT panel
6. On real hardware, the matching pins will activate!

## Wiring Safety

- Always use **330Ω resistors** with LEDs to limit current
- Never connect a GPIO pin directly to **5V** or **ground**
- Use **10kΩ pull-down resistors** with buttons
- Power servos from an **external 5V supply**, not the Pi
- Double-check wiring before powering on
