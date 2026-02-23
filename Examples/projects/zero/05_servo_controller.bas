REM ============================================================
REM  PROJECT 05 — Servo Controller
REM  Board:    Raspberry Pi Zero
REM  Hardware: Micro servo on GPIO18 (PWM0)
REM  Wiring:   Servo signal → GPIO18 (pin 12)
REM            Servo VCC → 5V (pin 2)
REM            Servo GND → GND (pin 6)
REM ============================================================

PRINT "=== Raspberry Pi Zero: Servo Controller ==="
PRINT "Servo on GPIO18 (hardware PWM0)"
PRINT ""

servoPin = 18
PINMODE servoPin, PWM

REM --- Smooth sweep ---
PRINT "--- Smooth Forward Sweep ---"
FOR angle = 0 TO 180 STEP 10
    SERVOWRITE servoPin, angle
    PRINT "  Angle: "; angle; " deg"
    SLEEP 150
NEXT angle

PRINT ""
PRINT "--- Smooth Reverse Sweep ---"
FOR angle = 180 TO 0 STEP -10
    SERVOWRITE servoPin, angle
    PRINT "  Angle: "; angle; " deg"
    SLEEP 150
NEXT angle

PRINT ""

REM --- Dial Positions (like a gauge) ---
PRINT "--- Gauge Demo ---"
PRINT "  Simulating a fuel gauge..."
PRINT ""

DIM labels$(5)
DIM positions(5)
labels$(0) = "EMPTY"
labels$(1) = "1/4"
labels$(2) = "1/2"
labels$(3) = "3/4"
labels$(4) = "FULL"
positions(0) = 0
positions(1) = 45
positions(2) = 90
positions(3) = 135
positions(4) = 180

FOR i = 0 TO 4
    SERVOWRITE servoPin, positions(i)
    PRINT "  Fuel: "; labels$(i); " ("; positions(i); " deg)"
    SLEEP 800
NEXT i

PRINT ""

REM --- Scanning radar effect ---
PRINT "--- Radar Scan ---"
FOR scan = 1 TO 3
    PRINT "  Scan "; scan
    FOR a = 0 TO 180 STEP 5
        SERVOWRITE servoPin, a
        SLEEP 30
    NEXT a
    FOR a = 180 TO 0 STEP -5
        SERVOWRITE servoPin, a
        SLEEP 30
    NEXT a
NEXT scan

REM Centre and finish
SERVOWRITE servoPin, 90
PRINT ""
PRINT "Servo at centre position (90 deg)"
PRINT "Demo complete!"
GPIORESET
