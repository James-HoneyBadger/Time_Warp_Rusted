REM ============================================================
REM  PROJECT 03 — PWM LED Fader
REM  Board:    Raspberry Pi Zero
REM  Hardware: LED + 330Ω on GPIO18 (hardware PWM pin)
REM  Wiring:   GPIO18 (pin 12) → 330Ω → LED → GND
REM  NOTE:     GPIO18 is the Pi Zero's hardware PWM0 pin
REM ============================================================

PRINT "=== Raspberry Pi Zero: PWM LED Fader ==="
PRINT "LED on GPIO18 (hardware PWM0)"
PRINT ""

pwmPin = 18
PINMODE pwmPin, PWM

REM --- Smooth fade up ---
PRINT "Fading up..."
FOR duty = 0 TO 255 STEP 5
    PWMWRITE pwmPin, duty
    pct = INT(duty / 255 * 100)
    IF pct MOD 20 = 0 THEN
        PRINT "  Brightness: "; pct; "%"
    END IF
    SLEEP 30
NEXT duty

PRINT ""

REM --- Smooth fade down ---
PRINT "Fading down..."
FOR duty = 255 TO 0 STEP -5
    PWMWRITE pwmPin, duty
    pct = INT(duty / 255 * 100)
    IF pct MOD 20 = 0 THEN
        PRINT "  Brightness: "; pct; "%"
    END IF
    SLEEP 30
NEXT duty

PRINT ""

REM --- Breathing effect ---
PRINT "Breathing effect (3 breaths)..."
FOR breath = 1 TO 3
    PRINT "  Breath "; breath
    REM Inhale
    FOR d = 0 TO 255 STEP 3
        PWMWRITE pwmPin, d
        SLEEP 10
    NEXT d
    REM Hold
    SLEEP 200
    REM Exhale
    FOR d = 255 TO 0 STEP -3
        PWMWRITE pwmPin, d
        SLEEP 10
    NEXT d
    SLEEP 400
NEXT breath

PWMWRITE pwmPin, 0
PRINT ""
PRINT "PWM fader demo complete!"
GPIORESET
