REM ============================================================
REM  PROJECT 08 — LED Dice
REM  Board:    Raspberry Pi Pico
REM  Hardware: 7 LEDs arranged as dice face, button on GP2
REM
REM  LED Layout (on a dice face):
REM     GP10  .  GP11        (top-left, top-right)
REM      .  GP12  .          (middle-centre)
REM     GP13  .  GP14        (mid-left, mid-right)
REM      .  GP15  .          (bottom-centre — for 1, 3, 5)
REM     GP16  .  GP17        (bottom-left, bottom-right)
REM
REM  Button on GP2 (pressed = HIGH)
REM ============================================================

PRINT "=== Raspberry Pi Pico: LED Dice ==="
PRINT "7 LEDs (GP10-GP17), Button on GP2"
PRINT ""

REM Configure button input
PINMODE 2, INPUT

REM Configure 7 LED outputs (using GP10-GP16 for 7 LEDs)
FOR p = 10 TO 16
    PINMODE p, OUTPUT
    DIGITALWRITE p, LOW
NEXT p

REM LED mapping for dice faces:
REM   Face 1: centre only (GP12)
REM   Face 2: top-right + bottom-left (GP11, GP16)
REM   Face 3: top-right + centre + bottom-left (GP11, GP12, GP16)
REM   Face 4: four corners (GP10, GP11, GP15, GP16)
REM   Face 5: four corners + centre (GP10, GP11, GP12, GP15, GP16)
REM   Face 6: all six outer (GP10, GP11, GP13, GP14, GP15, GP16)

RANDOMIZE

PRINT "Press the button (GP2) to roll!"
PRINT "(Simulating 10 automatic rolls)"
PRINT ""

SUB AllOff()
    FOR p = 10 TO 16
        DIGITALWRITE p, LOW
    NEXT p
END SUB

SUB ShowDice(face)
    GOSUB AllOff

    IF face = 1 THEN
        DIGITALWRITE 12, HIGH
    END IF
    IF face = 2 THEN
        DIGITALWRITE 11, HIGH
        DIGITALWRITE 16, HIGH
    END IF
    IF face = 3 THEN
        DIGITALWRITE 11, HIGH
        DIGITALWRITE 12, HIGH
        DIGITALWRITE 16, HIGH
    END IF
    IF face = 4 THEN
        DIGITALWRITE 10, HIGH
        DIGITALWRITE 11, HIGH
        DIGITALWRITE 15, HIGH
        DIGITALWRITE 16, HIGH
    END IF
    IF face = 5 THEN
        DIGITALWRITE 10, HIGH
        DIGITALWRITE 11, HIGH
        DIGITALWRITE 12, HIGH
        DIGITALWRITE 15, HIGH
        DIGITALWRITE 16, HIGH
    END IF
    IF face = 6 THEN
        DIGITALWRITE 10, HIGH
        DIGITALWRITE 11, HIGH
        DIGITALWRITE 13, HIGH
        DIGITALWRITE 14, HIGH
        DIGITALWRITE 15, HIGH
        DIGITALWRITE 16, HIGH
    END IF
END SUB

REM --- Roll animation ---
FOR roll = 1 TO 10
    PRINT "Roll #"; roll; ": ";

    REM Quick animation — flash random faces
    FOR spin = 1 TO 6
        face = INT(RND * 6) + 1
        GOSUB ShowDice(face)
        SLEEP 80
    NEXT spin

    REM Final result
    result = INT(RND * 6) + 1
    GOSUB ShowDice(result)

    PRINT "🎲 "; result
    SLEEP 1000

    GOSUB AllOff
    SLEEP 300
NEXT roll

PRINT ""
PRINT "Dice game complete!"
GPIORESET
