REM ============================================================
REM  PROJECT 08 — Simon Says (Memory Game)
REM  Board:    Raspberry Pi Zero
REM  Hardware: 4 LEDs + 4 Buttons
REM            LED Red=GPIO17, Green=GPIO27, Blue=GPIO22, Yellow=GPIO5
REM            Button Red=GPIO6, Green=GPIO13, Blue=GPIO19, Yellow=GPIO26
REM  Wiring:   Each LED: GPIOx → 330Ω → LED → GND
REM            Each Button: GPIOx → button → 3.3V (10k pull-down)
REM ============================================================

PRINT "=== Raspberry Pi Zero: Simon Says ==="
PRINT "4 LEDs + 4 Buttons — memory game!"
PRINT ""

REM LED pins
DIM ledPin(4)
ledPin(0) = 17
ledPin(1) = 27
ledPin(2) = 22
ledPin(3) = 5

REM Button pins
DIM btnPin(4)
btnPin(0) = 6
btnPin(1) = 13
btnPin(2) = 19
btnPin(3) = 26

REM Colour names
DIM colour$(4)
colour$(0) = "RED"
colour$(1) = "GREEN"
colour$(2) = "BLUE"
colour$(3) = "YELLOW"

REM Configure pins
FOR i = 0 TO 3
    PINMODE ledPin(i), OUTPUT
    DIGITALWRITE ledPin(i), LOW
    PINMODE btnPin(i), INPUT
NEXT i

SUB AllLedsOff()
    FOR j = 0 TO 3
        DIGITALWRITE ledPin(j), LOW
    NEXT j
END SUB

SUB FlashLed(index, duration)
    DIGITALWRITE ledPin(index), HIGH
    SLEEP duration
    DIGITALWRITE ledPin(index), LOW
    SLEEP 200
END SUB

REM --- Welcome sequence ---
PRINT "╔══════════════════════════════════════╗"
PRINT "║          SIMON SAYS!                 ║"
PRINT "║                                      ║"
PRINT "║  Watch the LED sequence, then        ║"
PRINT "║  repeat it by pressing the buttons   ║"
PRINT "║  in the same order!                  ║"
PRINT "╚══════════════════════════════════════╝"
PRINT ""

REM Flash all LEDs as intro
FOR i = 0 TO 3
    DIGITALWRITE ledPin(i), HIGH
NEXT i
SLEEP 500
GOSUB AllLedsOff
SLEEP 300

RANDOMIZE

REM Game sequence (max 10 rounds)
maxRounds = 8
DIM sequence(10)

PRINT "Game starting..."
PRINT ""

REM Generate the full sequence upfront
FOR i = 0 TO maxRounds - 1
    sequence(i) = INT(RND * 4)
NEXT i

REM Play through increasing lengths
score = 0
FOR round = 1 TO maxRounds
    PRINT "═══ Round "; round; " ═══"
    PRINT "  Watch the sequence..."
    SLEEP 500

    REM Show the sequence
    FOR i = 0 TO round - 1
        idx = sequence(i)
        PRINT "    Flash: "; colour$(idx)
        GOSUB FlashLed(idx, 400)
    NEXT i

    PRINT ""
    PRINT "  Your turn! Repeat the sequence:"

    REM Simulate player input (in real hardware, poll buttons)
    REM We simulate correct answers for rounds 1-6, then a mistake
    correct = 1
    FOR i = 0 TO round - 1
        expected = sequence(i)

        REM Simulate: player gets it right most of the time
        IF round <= 6 THEN
            playerChoice = expected
        ELSE
            REM Higher rounds: chance of mistake
            IF RND > 0.5 THEN
                playerChoice = expected
            ELSE
                playerChoice = INT(RND * 4)
            END IF
        END IF

        PRINT "    You pressed: "; colour$(playerChoice);

        IF playerChoice = expected THEN
            PRINT " ✓"
            GOSUB FlashLed(playerChoice, 200)
        ELSE
            PRINT " ✗ (was "; colour$(expected); ")"
            correct = 0

            REM Error flash — all LEDs blink
            FOR e = 1 TO 3
                FOR j = 0 TO 3
                    DIGITALWRITE ledPin(j), HIGH
                NEXT j
                SLEEP 100
                GOSUB AllLedsOff
                SLEEP 100
            NEXT e

            GOTO GameOver
        END IF
    NEXT i

    IF correct = 1 THEN
        score = round
        PRINT "  ✅ Correct! Score: "; score
        PRINT ""
        SLEEP 500
    END IF
NEXT round

GameOver:
PRINT ""
PRINT "╔══════════════════════════════════════╗"
PRINT "║          GAME OVER!                  ║"
PRINT "╚══════════════════════════════════════╝"
PRINT ""
PRINT "  Final Score: "; score; " out of "; maxRounds
PRINT ""

IF score >= 8 THEN
    PRINT "  🏆 PERFECT! Incredible memory!"
ELSE IF score >= 6 THEN
    PRINT "  ⭐ Excellent recall!"
ELSE IF score >= 4 THEN
    PRINT "  👍 Good effort!"
ELSE
    PRINT "  💪 Keep practising!"
END IF

REM Victory lap — chase pattern
FOR lap = 1 TO 2
    FOR i = 0 TO 3
        DIGITALWRITE ledPin(i), HIGH
        SLEEP 100
        DIGITALWRITE ledPin(i), LOW
    NEXT i
NEXT lap

GOSUB AllLedsOff
PRINT ""
PRINT "Simon Says complete!"
GPIORESET
