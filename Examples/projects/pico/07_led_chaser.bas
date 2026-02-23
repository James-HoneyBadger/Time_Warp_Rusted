REM ============================================================
REM  PROJECT 07 — LED Chaser (Knight Rider Effect)
REM  Board:    Raspberry Pi Pico
REM  Hardware: 6 LEDs on GP10-GP15
REM  Wiring:   Each GPx → 330Ω → LED → GND
REM ============================================================

PRINT "=== Raspberry Pi Pico: LED Chaser ==="
PRINT "6 LEDs on GP10 through GP15"
PRINT ""

REM Number of LEDs
numLeds = 6
startPin = 10

REM Configure all LED pins as outputs
FOR i = 0 TO numLeds - 1
    PINMODE startPin + i, OUTPUT
    DIGITALWRITE startPin + i, LOW
NEXT i

PRINT "Running LED chaser — Knight Rider style!"
PRINT ""

REM --- Pattern 1: Simple chase ---
PRINT "--- Pattern 1: Simple Chase ---"
FOR cycle = 1 TO 3
    REM Forward sweep
    FOR led = 0 TO numLeds - 1
        pin = startPin + led

        REM Turn this LED on
        DIGITALWRITE pin, HIGH
        PRINT "  LED "; led + 1; " ON  (GP"; pin; ")"
        SLEEP 100

        REM Turn it off
        DIGITALWRITE pin, LOW
    NEXT led

    REM Reverse sweep
    FOR led = numLeds - 2 TO 1 STEP -1
        pin = startPin + led
        DIGITALWRITE pin, HIGH
        PRINT "  LED "; led + 1; " ON  (GP"; pin; ")"
        SLEEP 100
        DIGITALWRITE pin, LOW
    NEXT led
NEXT cycle
PRINT ""

REM --- Pattern 2: Fill up, empty down ---
PRINT "--- Pattern 2: Fill & Empty ---"
FOR cycle = 1 TO 2
    REM Fill up one by one
    PRINT "  Filling..."
    FOR led = 0 TO numLeds - 1
        DIGITALWRITE startPin + led, HIGH
        SLEEP 150
    NEXT led
    SLEEP 300

    REM Empty down one by one
    PRINT "  Emptying..."
    FOR led = numLeds - 1 TO 0 STEP -1
        DIGITALWRITE startPin + led, LOW
        SLEEP 150
    NEXT led
    SLEEP 300
NEXT cycle
PRINT ""

REM --- Pattern 3: Alternating pairs ---
PRINT "--- Pattern 3: Alternating ---"
FOR cycle = 1 TO 4
    REM Even LEDs on, odd off
    FOR led = 0 TO numLeds - 1
        IF led MOD 2 = 0 THEN
            DIGITALWRITE startPin + led, HIGH
        ELSE
            DIGITALWRITE startPin + led, LOW
        END IF
    NEXT led
    PRINT "  Even LEDs ON"
    SLEEP 300

    REM Odd LEDs on, even off
    FOR led = 0 TO numLeds - 1
        IF led MOD 2 = 1 THEN
            DIGITALWRITE startPin + led, HIGH
        ELSE
            DIGITALWRITE startPin + led, LOW
        END IF
    NEXT led
    PRINT "  Odd LEDs ON"
    SLEEP 300
NEXT cycle

REM All off
FOR i = 0 TO numLeds - 1
    DIGITALWRITE startPin + i, LOW
NEXT i

PRINT ""
PRINT "LED chaser complete!"
GPIORESET
