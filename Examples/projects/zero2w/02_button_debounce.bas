REM ============================================================
REM  PROJECT 02 — Button & LED with Debounce
REM  Board:    Raspberry Pi Zero 2 W
REM  Hardware: Button on GPIO27, LED on GPIO17
REM  Wiring:   GPIO27 → button → 3.3V (10k pull-down to GND)
REM            GPIO17 → 330Ω → LED → GND
REM ============================================================

PRINT "=== Raspberry Pi Zero 2 W: Button & LED ==="
PRINT "Button=GPIO27, LED=GPIO17"
PRINT ""

buttonPin = 27
ledPin = 17

PINMODE buttonPin, INPUT
PINMODE ledPin, OUTPUT

PRINT "Monitoring button with debounce logic..."
PRINT "(Toggle GPIO27 in the simulator)"
PRINT ""

REM Debounce: track previous state to detect edges
prevState = 0
pressCount = 0

FOR cycle = 1 TO 30
    currentState = DIGITALREAD(buttonPin)

    REM Detect rising edge (button just pressed)
    IF currentState = 1 AND prevState = 0 THEN
        pressCount = pressCount + 1
        DIGITALWRITE ledPin, HIGH
        PRINT "  Press #"; pressCount; " detected → LED ON"
    END IF

    REM Detect falling edge (button just released)
    IF currentState = 0 AND prevState = 1 THEN
        DIGITALWRITE ledPin, LOW
        PRINT "  Released → LED OFF"
    END IF

    prevState = currentState
    SLEEP 50
NEXT cycle

PRINT ""
PRINT "Total presses detected: "; pressCount
PRINT "Button demo complete!"
GPIORESET
