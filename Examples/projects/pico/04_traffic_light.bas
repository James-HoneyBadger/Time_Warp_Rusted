REM ============================================================
REM  PROJECT 04 — Traffic Light Controller
REM  Board:    Raspberry Pi Pico
REM  Hardware: Red LED on GP13, Amber LED on GP14, Green LED on GP15
REM  Wiring:   Each: GPx → 330Ω → LED → GND
REM ============================================================

PRINT "=== Raspberry Pi Pico: Traffic Light ==="
PRINT "Red=GP13  Amber=GP14  Green=GP15"
PRINT ""

REM Define pin numbers
redPin = 13
amberPin = 14
greenPin = 15

REM Configure all three as outputs
PINMODE redPin, OUTPUT
PINMODE amberPin, OUTPUT
PINMODE greenPin, OUTPUT

REM Turn all off to start
DIGITALWRITE redPin, LOW
DIGITALWRITE amberPin, LOW
DIGITALWRITE greenPin, LOW

REM --- UK Traffic Light Sequence ---
REM 1. RED              (stop)
REM 2. RED + AMBER      (prepare)
REM 3. GREEN            (go)
REM 4. AMBER            (caution)
REM 5. Back to RED

FOR cycle = 1 TO 3
    PRINT "--- Cycle "; cycle; " of 3 ---"
    PRINT ""

    REM Phase 1: RED
    DIGITALWRITE redPin, HIGH
    DIGITALWRITE amberPin, LOW
    DIGITALWRITE greenPin, LOW
    PRINT "  🔴 RED — STOP"
    SLEEP 3000

    REM Phase 2: RED + AMBER
    DIGITALWRITE redPin, HIGH
    DIGITALWRITE amberPin, HIGH
    DIGITALWRITE greenPin, LOW
    PRINT "  🔴🟡 RED+AMBER — Get Ready"
    SLEEP 1500

    REM Phase 3: GREEN
    DIGITALWRITE redPin, LOW
    DIGITALWRITE amberPin, LOW
    DIGITALWRITE greenPin, HIGH
    PRINT "  🟢 GREEN — GO"
    SLEEP 4000

    REM Phase 4: AMBER only
    DIGITALWRITE redPin, LOW
    DIGITALWRITE amberPin, HIGH
    DIGITALWRITE greenPin, LOW
    PRINT "  🟡 AMBER — Caution"
    SLEEP 2000

    PRINT ""
NEXT cycle

REM End on RED (safe state)
DIGITALWRITE redPin, HIGH
DIGITALWRITE amberPin, LOW
DIGITALWRITE greenPin, LOW
PRINT "Traffic light stopped on RED (safe)"
GPIORESET
