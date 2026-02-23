REM ============================================================
REM  PROJECT 06 — Servo Sweep
REM  Board:    Raspberry Pi Pico
REM  Hardware: Micro servo on GP16
REM  Wiring:   Servo signal (orange) → GP16
REM            Servo VCC (red) → 5V (VBUS)
REM            Servo GND (brown) → GND
REM ============================================================

PRINT "=== Raspberry Pi Pico: Servo Sweep ==="
PRINT "Servo motor on GP16"
PRINT ""

servoPin = 16

REM SERVOWRITE converts angle (0-180) to appropriate PWM duty
REM Servo PWM: 2.5% duty = 0°, 12.5% duty = 180°

PINMODE servoPin, PWM

PRINT "Sweeping servo from 0 to 180 degrees..."
PRINT ""

REM Sweep from 0° to 180° in 15° steps
PRINT "--- Forward Sweep ---"
FOR angle = 0 TO 180 STEP 15
    SERVOWRITE servoPin, angle
    PRINT "Servo angle: "; angle; " degrees"
    SLEEP 200
NEXT angle

PRINT ""

REM Sweep back from 180° to 0°
PRINT "--- Reverse Sweep ---"
FOR angle = 180 TO 0 STEP -15
    SERVOWRITE servoPin, angle
    PRINT "Servo angle: "; angle; " degrees"
    SLEEP 200
NEXT angle

PRINT ""

REM Move to specific positions
PRINT "--- Named Positions ---"
SERVOWRITE servoPin, 0
PRINT "Position: LEFT   (0 degrees)"
SLEEP 1000

SERVOWRITE servoPin, 90
PRINT "Position: CENTER (90 degrees)"
SLEEP 1000

SERVOWRITE servoPin, 180
PRINT "Position: RIGHT  (180 degrees)"
SLEEP 1000

SERVOWRITE servoPin, 90
PRINT "Position: CENTER (90 degrees)"
SLEEP 500

PRINT ""
PRINT "Servo demo complete!"
GPIORESET
