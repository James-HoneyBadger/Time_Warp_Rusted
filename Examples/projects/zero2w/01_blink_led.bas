REM ============================================================
REM  PROJECT 01 — Blink LED
REM  Board:    Raspberry Pi Zero 2 W
REM  Hardware: LED + 330Ω on GPIO17
REM  Wiring:   GPIO17 (pin 11) → resistor → LED → GND (pin 9)
REM  NOTE:     Same pinout as Pi Zero but with quad-core + WiFi
REM ============================================================

PRINT "=== Raspberry Pi Zero 2 W: Blink LED ==="
PRINT "LED on GPIO17 — same pinout as Pi Zero"
PRINT "Bonus: quad-core CPU + WiFi onboard!"
PRINT ""

ledPin = 17
PINMODE ledPin, OUTPUT

PRINT "Blinking LED (10 cycles)..."
PRINT ""

FOR i = 1 TO 10
    DIGITALWRITE ledPin, HIGH
    PRINT "  "; i; ". LED ON"
    SLEEP 500

    DIGITALWRITE ledPin, LOW
    PRINT "     LED off"
    SLEEP 500
NEXT i

DIGITALWRITE ledPin, LOW
PRINT ""
PRINT "Blink complete!"
GPIORESET
