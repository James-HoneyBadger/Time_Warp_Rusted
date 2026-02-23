REM ============================================================
REM  PROJECT 02 — Button & LED
REM  Board:    Raspberry Pi Pico
REM  Hardware: Push button on GP14 (pulled LOW), LED on GP15
REM  Wiring:   GP14 → button → 3.3V  (with 10k pull-down to GND)
REM            GP15 → 330Ω → LED → GND
REM ============================================================

PRINT "=== Raspberry Pi Pico: Button & LED ==="
PRINT "Button on GP14, LED on GP15"
PRINT ""

REM Configure pins
PINMODE 14, INPUT
PINMODE 15, OUTPUT

PRINT "Reading button state..."
PRINT "(In simulator, toggle GP14 to simulate button press)"
PRINT ""

REM Read button and control LED
FOR cycle = 1 TO 20
    reading = DIGITALREAD(14)

    IF reading = 1 THEN
        DIGITALWRITE 15, HIGH
        PRINT "Cycle "; cycle; ": Button PRESSED  → LED ON"
    ELSE
        DIGITALWRITE 15, LOW
        PRINT "Cycle "; cycle; ": Button released → LED OFF"
    END IF

    SLEEP 250
NEXT cycle

PRINT ""
PRINT "Button/LED demo complete!"
GPIORESET
