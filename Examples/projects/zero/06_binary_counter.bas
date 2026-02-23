REM ============================================================
REM  PROJECT 06 — Binary Counter
REM  Board:    Raspberry Pi Zero
REM  Hardware: 4 LEDs on GPIO5, GPIO6, GPIO13, GPIO19
REM  Wiring:   Each GPIOx → 330Ω → LED → GND
REM            GPIO5=pin 29, GPIO6=pin 31, GPIO13=pin 33, GPIO19=pin 35
REM  Displays: Numbers 0-15 in binary on 4 LEDs
REM ============================================================

PRINT "=== Raspberry Pi Zero: Binary Counter ==="
PRINT "4 LEDs: GPIO5 (bit0), GPIO6 (bit1), GPIO13 (bit2), GPIO19 (bit3)"
PRINT ""

REM Define the 4 LED pins (bit 0 = LSB, bit 3 = MSB)
DIM ledPin(4)
ledPin(0) = 5
ledPin(1) = 6
ledPin(2) = 13
ledPin(3) = 19

REM Configure all as outputs
FOR i = 0 TO 3
    PINMODE ledPin(i), OUTPUT
    DIGITALWRITE ledPin(i), LOW
NEXT i

PRINT "Counting from 0 to 15 in binary..."
PRINT ""
PRINT "Decimal  Binary   GPIO19  GPIO13  GPIO6   GPIO5"
PRINT "-------  ------   ------  ------  -----   -----"

REM Count from 0 to 15
FOR num = 0 TO 15
    REM Extract each bit
    bit0 = num MOD 2
    bit1 = INT(num / 2) MOD 2
    bit2 = INT(num / 4) MOD 2
    bit3 = INT(num / 8) MOD 2

    REM Set the LEDs
    IF bit0 = 1 THEN
        DIGITALWRITE ledPin(0), HIGH
    ELSE
        DIGITALWRITE ledPin(0), LOW
    END IF
    IF bit1 = 1 THEN
        DIGITALWRITE ledPin(1), HIGH
    ELSE
        DIGITALWRITE ledPin(1), LOW
    END IF
    IF bit2 = 1 THEN
        DIGITALWRITE ledPin(2), HIGH
    ELSE
        DIGITALWRITE ledPin(2), LOW
    END IF
    IF bit3 = 1 THEN
        DIGITALWRITE ledPin(3), HIGH
    ELSE
        DIGITALWRITE ledPin(3), LOW
    END IF

    REM Display
    b3$ = "OFF"
    b2$ = "OFF"
    b1$ = "OFF"
    b0$ = "OFF"
    IF bit3 = 1 THEN b3$ = "ON "
    IF bit2 = 1 THEN b2$ = "ON "
    IF bit1 = 1 THEN b1$ = "ON "
    IF bit0 = 1 THEN b0$ = "ON "

    PRINT "  "; num; "       ";
    PRINT bit3; bit2; bit1; bit0; "     ";
    PRINT b3$; "     "; b2$; "     "; b1$; "     "; b0$

    SLEEP 500
NEXT num

PRINT ""

REM Bonus: count back down
PRINT "Counting down..."
FOR num = 15 TO 0 STEP -1
    bit0 = num MOD 2
    bit1 = INT(num / 2) MOD 2
    bit2 = INT(num / 4) MOD 2
    bit3 = INT(num / 8) MOD 2

    IF bit0 = 1 THEN DIGITALWRITE ledPin(0), HIGH ELSE DIGITALWRITE ledPin(0), LOW
    IF bit1 = 1 THEN DIGITALWRITE ledPin(1), HIGH ELSE DIGITALWRITE ledPin(1), LOW
    IF bit2 = 1 THEN DIGITALWRITE ledPin(2), HIGH ELSE DIGITALWRITE ledPin(2), LOW
    IF bit3 = 1 THEN DIGITALWRITE ledPin(3), HIGH ELSE DIGITALWRITE ledPin(3), LOW

    PRINT "  "; num; " = "; bit3; bit2; bit1; bit0
    SLEEP 300
NEXT num

REM All off
FOR i = 0 TO 3
    DIGITALWRITE ledPin(i), LOW
NEXT i

PRINT ""
PRINT "Binary counter complete!"
GPIORESET
