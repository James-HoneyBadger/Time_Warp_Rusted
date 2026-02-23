REM ============================================================
REM  PROJECT 01 — Blink LED
REM  Board:    Raspberry Pi Zero
REM  Hardware: LED + 330Ω resistor on GPIO17
REM  Wiring:   GPIO17 (pin 11) → resistor → LED → GND (pin 9)
REM  NOTE:     Pi Zero uses BCM GPIO numbering (0-27)
REM ============================================================

PRINT "=== Raspberry Pi Zero: Blink LED ==="
PRINT "LED connected to GPIO17 (physical pin 11)"
PRINT ""

REM On Pi Zero, GPIO numbers are BCM numbering
REM Common GPIO pins: 17, 27, 22, 5, 6, 13, 19, 26
ledPin = 17

PINMODE ledPin, OUTPUT

PRINT "Blinking LED..."
PRINT ""

FOR i = 1 TO 10
    DIGITALWRITE ledPin, HIGH
    PRINT "Blink "; i; ": LED ON"
    SLEEP 500

    DIGITALWRITE ledPin, LOW
    PRINT "          LED OFF"
    SLEEP 500
NEXT i

DIGITALWRITE ledPin, LOW
PRINT ""
PRINT "Blink complete — 10 cycles!"
GPIORESET
