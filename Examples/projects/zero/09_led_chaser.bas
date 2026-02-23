REM ============================================================
REM  PROJECT 09 — LED Chaser with Patterns
REM  Board:    Raspberry Pi Zero
REM  Hardware: 8 LEDs on GPIO5,6,13,19,26,16,20,21
REM  Wiring:   Each GPIOx → 330Ω → LED → GND
REM ============================================================

PRINT "=== Raspberry Pi Zero: LED Chaser ==="
PRINT "8 LEDs for stunning light patterns"
PRINT ""

REM 8 LED pins
DIM led(8)
led(0) = 5
led(1) = 6
led(2) = 13
led(3) = 19
led(4) = 26
led(5) = 16
led(6) = 20
led(7) = 21

numLeds = 8

REM Configure all as outputs
FOR i = 0 TO numLeds - 1
    PINMODE led(i), OUTPUT
    DIGITALWRITE led(i), LOW
NEXT i

SUB AllOff()
    FOR k = 0 TO 7
        DIGITALWRITE led(k), LOW
    NEXT k
END SUB

REM ═══════════════════════════════════
PRINT "--- Pattern 1: Knight Rider ---"
FOR cycle = 1 TO 3
    FOR i = 0 TO numLeds - 1
        DIGITALWRITE led(i), HIGH
        SLEEP 60
        DIGITALWRITE led(i), LOW
    NEXT i
    FOR i = numLeds - 2 TO 1 STEP -1
        DIGITALWRITE led(i), HIGH
        SLEEP 60
        DIGITALWRITE led(i), LOW
    NEXT i
NEXT cycle
GOSUB AllOff
PRINT ""

REM ═══════════════════════════════════
PRINT "--- Pattern 2: Stack Fill ---"
FOR cycle = 1 TO 2
    REM Fill from left
    FOR i = 0 TO numLeds - 1
        DIGITALWRITE led(i), HIGH
        SLEEP 100
    NEXT i
    SLEEP 300
    REM Empty from right
    FOR i = numLeds - 1 TO 0 STEP -1
        DIGITALWRITE led(i), LOW
        SLEEP 100
    NEXT i
    SLEEP 300
NEXT cycle
PRINT ""

REM ═══════════════════════════════════
PRINT "--- Pattern 3: Ping Pong ---"
FOR cycle = 1 TO 4
    REM Left side
    FOR i = 0 TO 3
        DIGITALWRITE led(i), HIGH
    NEXT i
    FOR i = 4 TO 7
        DIGITALWRITE led(i), LOW
    NEXT i
    SLEEP 200

    REM Right side
    FOR i = 0 TO 3
        DIGITALWRITE led(i), LOW
    NEXT i
    FOR i = 4 TO 7
        DIGITALWRITE led(i), HIGH
    NEXT i
    SLEEP 200
NEXT cycle
GOSUB AllOff
PRINT ""

REM ═══════════════════════════════════
PRINT "--- Pattern 4: Binary Count ---"
FOR num = 0 TO 255
    FOR bit = 0 TO 7
        IF INT(num / (2 ^ bit)) MOD 2 = 1 THEN
            DIGITALWRITE led(bit), HIGH
        ELSE
            DIGITALWRITE led(bit), LOW
        END IF
    NEXT bit
    REM Only display every 16th value
    IF num MOD 16 = 0 THEN
        PRINT "  "; num
    END IF
    SLEEP 20
NEXT num
GOSUB AllOff
PRINT ""

REM ═══════════════════════════════════
PRINT "--- Pattern 5: Converging ---"
FOR cycle = 1 TO 3
    REM Outside in
    FOR i = 0 TO 3
        DIGITALWRITE led(i), HIGH
        DIGITALWRITE led(numLeds - 1 - i), HIGH
        SLEEP 100
    NEXT i
    SLEEP 200
    GOSUB AllOff
    SLEEP 100

    REM Inside out
    FOR i = 3 TO 0 STEP -1
        DIGITALWRITE led(i), HIGH
        DIGITALWRITE led(numLeds - 1 - i), HIGH
        SLEEP 100
    NEXT i
    SLEEP 200
    GOSUB AllOff
    SLEEP 100
NEXT cycle

REM --- Finale: all flash ---
PRINT "--- Finale ---"
FOR flash = 1 TO 5
    FOR i = 0 TO numLeds - 1
        DIGITALWRITE led(i), HIGH
    NEXT i
    SLEEP 100
    GOSUB AllOff
    SLEEP 100
NEXT flash

PRINT ""
PRINT "LED chaser complete — 5 patterns!"
GPIORESET
