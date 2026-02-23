REM ============================================================
REM  PROJECT 05 — Multi-Servo Robotic Arm
REM  Board:    Raspberry Pi Zero 2 W
REM  Hardware: 3 servos — Base, Shoulder, Gripper
REM            Base servo on GPIO12 (PWM0)
REM            Shoulder servo on GPIO13 (PWM1)
REM            Gripper servo on GPIO18
REM  Wiring:   Signal wires → GPIO pins
REM            VCC → external 5V supply (servos draw too much for Pi)
REM            GND → shared ground with Pi
REM ============================================================

PRINT "=== Raspberry Pi Zero 2 W: Robotic Arm ==="
PRINT "Base=GPIO12, Shoulder=GPIO13, Gripper=GPIO18"
PRINT ""

basePin = 12
shoulderPin = 13
gripperPin = 18

PINMODE basePin, PWM
PINMODE shoulderPin, PWM
PINMODE gripperPin, PWM

REM Servo positions
REM  Base:     0=far left, 90=centre, 180=far right
REM  Shoulder: 0=up, 90=forward, 180=down
REM  Gripper:  30=open, 90=closed

SUB MoveTo(base, shoulder, gripper)
    SERVOWRITE basePin, base
    SERVOWRITE shoulderPin, shoulder
    SERVOWRITE gripperPin, gripper
END SUB

PRINT "╔══════════════════════════════════════╗"
PRINT "║      ROBOTIC ARM CONTROLLER          ║"
PRINT "║                                      ║"
PRINT "║  Base:     Rotate left/right         ║"
PRINT "║  Shoulder: Raise/lower arm           ║"
PRINT "║  Gripper:  Open/close                ║"
PRINT "╚══════════════════════════════════════╝"
PRINT ""

REM --- Home Position ---
PRINT "Moving to HOME position..."
GOSUB MoveTo(90, 45, 30)
PRINT "  Base=90  Shoulder=45  Gripper=OPEN"
SLEEP 1000
PRINT ""

REM --- Demonstration: Pick and Place ---
PRINT "═══ Pick and Place Sequence ═══"
PRINT ""

REM Step 1: Rotate to pickup position (left)
PRINT "1. Rotating to pickup zone (left)..."
FOR angle = 90 TO 30 STEP -5
    SERVOWRITE basePin, angle
    SLEEP 30
NEXT angle
PRINT "   Base at 30 degrees"
SLEEP 500

REM Step 2: Lower arm
PRINT "2. Lowering arm..."
FOR angle = 45 TO 120 STEP 5
    SERVOWRITE shoulderPin, angle
    SLEEP 30
NEXT angle
PRINT "   Shoulder at 120 degrees (low)"
SLEEP 500

REM Step 3: Close gripper (grab object)
PRINT "3. Closing gripper — grabbing object..."
FOR angle = 30 TO 90 STEP 5
    SERVOWRITE gripperPin, angle
    SLEEP 30
NEXT angle
PRINT "   Gripper CLOSED — object secured!"
SLEEP 500

REM Step 4: Raise arm
PRINT "4. Raising arm..."
FOR angle = 120 TO 45 STEP -5
    SERVOWRITE shoulderPin, angle
    SLEEP 30
NEXT angle
PRINT "   Shoulder at 45 degrees (raised)"
SLEEP 500

REM Step 5: Rotate to drop position (right)
PRINT "5. Rotating to drop zone (right)..."
FOR angle = 30 TO 150 STEP 5
    SERVOWRITE basePin, angle
    SLEEP 30
NEXT angle
PRINT "   Base at 150 degrees"
SLEEP 500

REM Step 6: Lower slightly
PRINT "6. Lowering to drop height..."
FOR angle = 45 TO 80 STEP 5
    SERVOWRITE shoulderPin, angle
    SLEEP 30
NEXT angle
SLEEP 300

REM Step 7: Open gripper (release object)
PRINT "7. Opening gripper — releasing object..."
FOR angle = 90 TO 30 STEP -5
    SERVOWRITE gripperPin, angle
    SLEEP 30
NEXT angle
PRINT "   Gripper OPEN — object placed!"
SLEEP 500

REM Step 8: Return home
PRINT "8. Returning to HOME position..."
GOSUB MoveTo(90, 45, 30)
SLEEP 1000

PRINT ""
PRINT "═══ Pick and Place Complete! ═══"
PRINT ""

REM --- Wave gesture ---
PRINT "Bonus: Robot arm wave!"
FOR wave = 1 TO 3
    FOR a = 30 TO 90 STEP 10
        SERVOWRITE gripperPin, a
        SLEEP 50
    NEXT a
    FOR a = 90 TO 30 STEP -10
        SERVOWRITE gripperPin, a
        SLEEP 50
    NEXT a
NEXT wave

PRINT ""
PRINT "Robotic arm demo complete!"
GOSUB MoveTo(90, 45, 30)
GPIORESET
