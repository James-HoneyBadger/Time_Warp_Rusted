REM ============================================================
REM  PROJECT 07 — Reaction Timer Game
REM  Board:    Raspberry Pi Zero
REM  Hardware: LED on GPIO17, Button on GPIO27
REM  Wiring:   GPIO17 → 330Ω → LED → GND
REM            GPIO27 → button → 3.3V (with 10k pull-down)
REM  Game:     LED lights up after random delay; press button ASAP
REM ============================================================

PRINT "=== Raspberry Pi Zero: Reaction Timer ==="
PRINT "LED on GPIO17, Button on GPIO27"
PRINT ""

ledPin = 17
buttonPin = 27

PINMODE ledPin, OUTPUT
PINMODE buttonPin, INPUT

DIGITALWRITE ledPin, LOW

PRINT "╔══════════════════════════════════════╗"
PRINT "║     REACTION TIMER GAME              ║"
PRINT "║                                      ║"
PRINT "║  Watch the LED — when it lights up,  ║"
PRINT "║  press the button as fast as you can! ║"
PRINT "╚══════════════════════════════════════╝"
PRINT ""

RANDOMIZE

numRounds = 5
DIM times(5)
totalTime = 0

FOR round = 1 TO numRounds
    PRINT "--- Round "; round; " of "; numRounds; " ---"
    PRINT "  Get ready..."

    REM Ensure LED is off
    DIGITALWRITE ledPin, LOW

    REM Random delay between 1-4 seconds (simulated)
    waitTime = INT(RND * 3000) + 1000
    PRINT "  (Waiting "; waitTime; "ms...)"
    SLEEP waitTime

    REM LED on — start timing!
    DIGITALWRITE ledPin, HIGH
    PRINT "  >>> GO! LED IS ON! <<<"

    REM Simulate reaction time (in real hardware, this would poll the button)
    REM Simulated reaction: 150-500ms
    reactionMs = INT(RND * 350) + 150

    SLEEP reactionMs
    DIGITALWRITE ledPin, LOW

    PRINT "  Reaction time: "; reactionMs; " ms"

    REM Rate the reaction
    IF reactionMs < 200 THEN
        PRINT "  Rating: ⚡ LIGHTNING FAST!"
    ELSE IF reactionMs < 300 THEN
        PRINT "  Rating: 🏃 Quick!"
    ELSE IF reactionMs < 400 THEN
        PRINT "  Rating: 👍 Good"
    ELSE
        PRINT "  Rating: 🐢 Keep practising!"
    END IF

    times(round) = reactionMs
    totalTime = totalTime + reactionMs
    PRINT ""
NEXT round

REM --- Results ---
PRINT "╔══════════════════════════════════════╗"
PRINT "║         FINAL RESULTS                ║"
PRINT "╚══════════════════════════════════════╝"
PRINT ""

REM Find best and worst
best = times(1)
worst = times(1)
FOR i = 2 TO numRounds
    IF times(i) < best THEN best = times(i)
    IF times(i) > worst THEN worst = times(i)
NEXT i

average = INT(totalTime / numRounds)

PRINT "  Best time:    "; best; " ms"
PRINT "  Worst time:   "; worst; " ms"
PRINT "  Average time: "; average; " ms"
PRINT ""

IF average < 250 THEN
    PRINT "  Overall: ⚡ INCREDIBLE reflexes!"
ELSE IF average < 350 THEN
    PRINT "  Overall: 🏃 Great reactions!"
ELSE
    PRINT "  Overall: 💪 Room to improve — try again!"
END IF

PRINT ""
PRINT "Game over!"
GPIORESET
