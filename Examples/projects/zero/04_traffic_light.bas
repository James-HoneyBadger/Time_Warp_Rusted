REM ============================================================
REM  PROJECT 04 — Traffic Light
REM  Board:    Raspberry Pi Zero
REM  Hardware: Red LED on GPIO22, Amber LED on GPIO27, Green LED on GPIO17
REM  Wiring:   Each GPIOx → 330Ω → LED → GND
REM            GPIO22=pin 15, GPIO27=pin 13, GPIO17=pin 11
REM ============================================================

PRINT "=== Raspberry Pi Zero: Traffic Light ==="
PRINT "Red=GPIO22  Amber=GPIO27  Green=GPIO17"
PRINT ""

redPin = 22
amberPin = 27
greenPin = 17

PINMODE redPin, OUTPUT
PINMODE amberPin, OUTPUT
PINMODE greenPin, OUTPUT

REM All off initially
DIGITALWRITE redPin, LOW
DIGITALWRITE amberPin, LOW
DIGITALWRITE greenPin, LOW

REM --- Pedestrian Crossing Sequence ---
PRINT "Simulating a pedestrian crossing..."
PRINT ""

FOR cycle = 1 TO 3
    PRINT "=== Cycle "; cycle; " ==="

    REM Normal traffic: GREEN
    DIGITALWRITE redPin, LOW
    DIGITALWRITE amberPin, LOW
    DIGITALWRITE greenPin, HIGH
    PRINT "  🟢 GREEN — Traffic flowing"
    SLEEP 3000

    REM Pedestrian presses button...
    PRINT "  [Pedestrian presses button]"
    SLEEP 1000

    REM Amber warning
    DIGITALWRITE greenPin, LOW
    DIGITALWRITE amberPin, HIGH
    PRINT "  🟡 AMBER — Slowing down"
    SLEEP 2000

    REM Red — pedestrians cross
    DIGITALWRITE amberPin, LOW
    DIGITALWRITE redPin, HIGH
    PRINT "  🔴 RED — STOP — Pedestrians crossing"
    SLEEP 4000

    REM Flashing amber — prepare to go
    PRINT "  🟡 Flashing AMBER — Prepare"
    DIGITALWRITE redPin, LOW
    FOR flash = 1 TO 6
        DIGITALWRITE amberPin, HIGH
        SLEEP 300
        DIGITALWRITE amberPin, LOW
        SLEEP 300
    NEXT flash

    REM Back to green
    DIGITALWRITE greenPin, HIGH
    PRINT "  🟢 GREEN — Traffic resumes"
    SLEEP 1000
    PRINT ""
NEXT cycle

REM Safe shutdown — RED
DIGITALWRITE greenPin, LOW
DIGITALWRITE amberPin, LOW
DIGITALWRITE redPin, HIGH
PRINT "System stopped — RED for safety"
GPIORESET
