REM ============================================================
REM  PROJECT 07 — LED Bar Graph VU Meter
REM  Board:    Raspberry Pi Zero 2 W
REM  Hardware: 8 LEDs creating a bar graph display
REM            GPIO5,6,13,19,26,16,20,21
REM  Concept:  Simulates an audio VU meter / signal strength display
REM ============================================================

PRINT "=== Raspberry Pi Zero 2 W: VU Meter ==="
PRINT "8-LED bar graph on GPIO5-GPIO21"
PRINT ""

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

FOR i = 0 TO numLeds - 1
    PINMODE led(i), OUTPUT
    DIGITALWRITE led(i), LOW
NEXT i

SUB AllOff()
    FOR k = 0 TO 7
        DIGITALWRITE led(k), LOW
    NEXT k
END SUB

SUB ShowLevel(level)
    REM level = 0 to 8, lights that many LEDs from bottom
    FOR k = 0 TO 7
        IF k < level THEN
            DIGITALWRITE led(k), HIGH
        ELSE
            DIGITALWRITE led(k), LOW
        END IF
    NEXT k
END SUB

PRINT "╔══════════════════════════════════════╗"
PRINT "║        LED VU METER                  ║"
PRINT "║                                      ║"
PRINT "║  ████████  8 = Peak / Clipping!      ║"
PRINT "║  ███████   7 = Very Loud             ║"
PRINT "║  ██████    6 = Loud                  ║"
PRINT "║  █████     5 = Above Average         ║"
PRINT "║  ████      4 = Average               ║"
PRINT "║  ███       3 = Moderate              ║"
PRINT "║  ██        2 = Quiet                 ║"
PRINT "║  █         1 = Very Quiet            ║"
PRINT "║            0 = Silent                ║"
PRINT "╚══════════════════════════════════════╝"
PRINT ""

RANDOMIZE

REM --- Simulation 1: Music beat pattern ---
PRINT "--- Simulating Music Beat ---"

DIM beatPattern(16)
beatPattern(0) = 6
beatPattern(1) = 8
beatPattern(2) = 4
beatPattern(3) = 7
beatPattern(4) = 3
beatPattern(5) = 5
beatPattern(6) = 8
beatPattern(7) = 2
beatPattern(8) = 6
beatPattern(9) = 7
beatPattern(10) = 3
beatPattern(11) = 8
beatPattern(12) = 5
beatPattern(13) = 4
beatPattern(14) = 7
beatPattern(15) = 1

FOR beat = 0 TO 15
    level = beatPattern(beat)
    GOSUB ShowLevel(level)

    bar$ = ""
    FOR b = 1 TO level
        bar$ = bar$ + "█"
    NEXT b
    PRINT "  Beat "; beat + 1; ": "; bar$

    SLEEP 200
NEXT beat

GOSUB AllOff
SLEEP 500
PRINT ""

REM --- Simulation 2: Rising signal ---
PRINT "--- Signal Strength Rising ---"
FOR level = 0 TO 8
    GOSUB ShowLevel(level)
    PRINT "  Signal: "; level; "/8"
    SLEEP 300
NEXT level
SLEEP 500

PRINT ""

REM --- Simulation 3: Falling signal ---
PRINT "--- Signal Strength Falling ---"
FOR level = 8 TO 0 STEP -1
    GOSUB ShowLevel(level)
    PRINT "  Signal: "; level; "/8"
    SLEEP 300
NEXT level
SLEEP 500

PRINT ""

REM --- Simulation 4: Random noise ---
PRINT "--- Random Signal Noise ---"
FOR sample = 1 TO 20
    level = INT(RND * 9)
    GOSUB ShowLevel(level)
    SLEEP 150
NEXT sample

GOSUB AllOff

PRINT ""
PRINT "VU meter demo complete!"
GPIORESET
