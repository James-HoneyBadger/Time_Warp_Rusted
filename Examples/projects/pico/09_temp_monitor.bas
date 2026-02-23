REM ============================================================
REM  PROJECT 09 — Temperature Monitor (ADC + Alert System)
REM  Board:    Raspberry Pi Pico
REM  Hardware: TMP36 temperature sensor on GP26 (ADC0)
REM            Green LED on GP13, Amber LED on GP14, Red LED on GP15
REM  Wiring:   TMP36 pin 1 → 3.3V, pin 2 → GP26, pin 3 → GND
REM            Each LED: GPx → 330Ω → LED → GND
REM  NOTE:     ADC is Pico-specific!
REM ============================================================

PRINT "=== Raspberry Pi Pico: Temperature Monitor ==="
PRINT "TMP36 on GP26 (ADC0), Alert LEDs on GP13-GP15"
PRINT ""

REM Configure pins
sensorPin = 26
greenPin = 13
amberPin = 14
redPin = 15

PINMODE sensorPin, INPUT
PINMODE greenPin, OUTPUT
PINMODE amberPin, OUTPUT
PINMODE redPin, OUTPUT

REM Alert thresholds (Celsius)
coldThreshold = 18
warmThreshold = 25
hotThreshold = 30

PRINT "Temperature Thresholds:"
PRINT "  COLD (blue):   below "; coldThreshold; " C"
PRINT "  NORMAL (green): "; coldThreshold; "-"; warmThreshold; " C"
PRINT "  WARM (amber):  "; warmThreshold; "-"; hotThreshold; " C"
PRINT "  HOT (red):     above "; hotThreshold; " C"
PRINT ""
PRINT "Reading   Voltage   Temp (C)   Status       LEDs"
PRINT "-------   -------   --------   ------       ----"

REM Simulate temperature readings (in real hardware, read from ADC)
RANDOMIZE
baseTemp = 20

FOR reading = 1 TO 15
    REM Simulate: gradually rising temperature with noise
    simTemp = baseTemp + (reading * 0.8) + (RND * 3) - 1.5

    REM Convert temperature to simulated voltage
    REM TMP36: voltage = temp * 0.01 + 0.5
    voltage = simTemp * 0.01 + 0.5

    REM Round for display
    tempC = INT(simTemp * 10) / 10

    REM Determine status and set LEDs
    DIGITALWRITE greenPin, LOW
    DIGITALWRITE amberPin, LOW
    DIGITALWRITE redPin, LOW

    IF simTemp < coldThreshold THEN
        status$ = "COLD  ❄️"
        REM All LEDs blink = cold warning
        DIGITALWRITE greenPin, HIGH
    ELSE IF simTemp < warmThreshold THEN
        status$ = "NORMAL ✓"
        DIGITALWRITE greenPin, HIGH
    ELSE IF simTemp < hotThreshold THEN
        status$ = "WARM  ⚠️"
        DIGITALWRITE amberPin, HIGH
    ELSE
        status$ = "HOT!  🔥"
        DIGITALWRITE redPin, HIGH
    END IF

    PRINT "  "; reading; "       ";
    PRINT voltage; "V    ";
    PRINT tempC; " C     ";
    PRINT status$

    SLEEP 500
NEXT reading

REM Summary
PRINT ""
PRINT "Monitoring complete."
PRINT "In real hardware, this runs continuously"
PRINT "and triggers alerts when thresholds are crossed."

DIGITALWRITE greenPin, LOW
DIGITALWRITE amberPin, LOW
DIGITALWRITE redPin, LOW
GPIORESET
