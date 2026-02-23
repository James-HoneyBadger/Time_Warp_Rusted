REM ============================================================
REM  PROJECT 02 — Button & LED
REM  Board:    Raspberry Pi Zero
REM  Hardware: Button on GPIO27, LED on GPIO17
REM  Wiring:   GPIO27 (pin 13) → button → 3.3V (pin 1)
REM            GPIO27 also → 10kΩ pull-down → GND
REM            GPIO17 (pin 11) → 330Ω → LED → GND
REM ============================================================

PRINT "=== Raspberry Pi Zero: Button & LED ==="
PRINT "Button on GPIO27, LED on GPIO17"
PRINT ""

buttonPin = 27
ledPin = 17

PINMODE buttonPin, INPUT
PINMODE ledPin, OUTPUT

PRINT "Monitoring button..."
PRINT "(Toggle GPIO27 in the simulator)"
PRINT ""

FOR cycle = 1 TO 20
    pressed = DIGITALREAD(buttonPin)

    IF pressed = 1 THEN
        DIGITALWRITE ledPin, HIGH
        PRINT "  ["; cycle; "] Button PRESSED → LED ON"
    ELSE
        DIGITALWRITE ledPin, LOW
        PRINT "  ["; cycle; "] Button ------- → LED off"
    END IF

    SLEEP 250
NEXT cycle

PRINT ""
PRINT "Button/LED demo complete!"
GPIORESET
