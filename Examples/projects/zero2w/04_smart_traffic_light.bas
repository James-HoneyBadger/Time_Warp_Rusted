REM ============================================================
REM  PROJECT 04 — Smart Traffic Light with Pedestrian Button
REM  Board:    Raspberry Pi Zero 2 W
REM  Hardware: Red LED=GPIO22, Amber=GPIO27, Green=GPIO17
REM            Pedestrian Walk LED=GPIO5, Don't Walk LED=GPIO6
REM            Pedestrian Button=GPIO13
REM  Wiring:   Each LED: GPIOx → 330Ω → LED → GND
REM            Button: GPIO13 → button → 3.3V (10k pull-down)
REM ============================================================

PRINT "=== Raspberry Pi Zero 2 W: Smart Traffic Light ==="
PRINT "Traffic: R=GPIO22, A=GPIO27, G=GPIO17"
PRINT "Pedestrian: Walk=GPIO5, Dont=GPIO6, Button=GPIO13"
PRINT ""

REM Traffic lights
redPin = 22
amberPin = 27
greenPin = 17

REM Pedestrian signals
walkPin = 5
dontWalkPin = 6
buttonPin = 13

REM Configure all pins
PINMODE redPin, OUTPUT
PINMODE amberPin, OUTPUT
PINMODE greenPin, OUTPUT
PINMODE walkPin, OUTPUT
PINMODE dontWalkPin, OUTPUT
PINMODE buttonPin, INPUT

SUB TrafficOff()
    DIGITALWRITE redPin, LOW
    DIGITALWRITE amberPin, LOW
    DIGITALWRITE greenPin, LOW
END SUB

SUB PedestrianOff()
    DIGITALWRITE walkPin, LOW
    DIGITALWRITE dontWalkPin, LOW
END SUB

PRINT "╔══════════════════════════════════════════╗"
PRINT "║    SMART TRAFFIC LIGHT CONTROLLER        ║"
PRINT "║    With pedestrian crossing              ║"
PRINT "╚══════════════════════════════════════════╝"
PRINT ""

REM --- Normal operation cycle ---
FOR cycle = 1 TO 3
    PRINT "═══ Cycle "; cycle; " ═══"

    REM Phase 1: GREEN for traffic, DON'T WALK for pedestrians
    GOSUB TrafficOff
    GOSUB PedestrianOff
    DIGITALWRITE greenPin, HIGH
    DIGITALWRITE dontWalkPin, HIGH
    PRINT "  🟢 Traffic GREEN | 🔴 Don't Walk"
    SLEEP 3000

    REM Check for pedestrian button press
    pedPressed = DIGITALREAD(buttonPin)
    IF pedPressed = 1 THEN
        PRINT "  [Pedestrian button pressed!]"
    ELSE
        PRINT "  [Automatic cycle — no button press]"
    END IF

    REM Phase 2: AMBER — prepare to stop
    GOSUB TrafficOff
    DIGITALWRITE amberPin, HIGH
    PRINT "  🟡 Traffic AMBER | 🔴 Don't Walk"
    SLEEP 2000

    REM Phase 3: RED — traffic stops, pedestrians can cross
    GOSUB TrafficOff
    GOSUB PedestrianOff
    DIGITALWRITE redPin, HIGH
    DIGITALWRITE walkPin, HIGH
    PRINT "  🔴 Traffic RED   | 🟢 WALK"
    SLEEP 4000

    REM Phase 4: Flashing WALK — hurry up
    PRINT "  🔴 Traffic RED   | ⚠️ HURRY (flashing)"
    FOR flash = 1 TO 5
        DIGITALWRITE walkPin, LOW
        SLEEP 300
        DIGITALWRITE walkPin, HIGH
        SLEEP 300
    NEXT flash

    REM Phase 5: Back to DON'T WALK
    GOSUB PedestrianOff
    DIGITALWRITE dontWalkPin, HIGH
    PRINT "  🔴 Traffic RED   | 🔴 Don't Walk"
    SLEEP 1000

    REM Phase 6: RED + AMBER — prepare to go
    DIGITALWRITE amberPin, HIGH
    PRINT "  🔴🟡 RED+AMBER   | 🔴 Don't Walk"
    SLEEP 1500

    PRINT ""
NEXT cycle

REM Safe shutdown
GOSUB TrafficOff
GOSUB PedestrianOff
DIGITALWRITE redPin, HIGH
DIGITALWRITE dontWalkPin, HIGH
PRINT "System stopped — traffic RED, pedestrians DON'T WALK"
GPIORESET
