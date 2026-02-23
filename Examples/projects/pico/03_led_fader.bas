REM ============================================================
REM  PROJECT 03 — LED Fader (PWM Breathing Effect)
REM  Board:    Raspberry Pi Pico
REM  Hardware: LED + 330 ohm resistor on GP15
REM  Wiring:   GP15 → resistor → LED anode → LED cathode → GND
REM ============================================================

PRINT "=== Raspberry Pi Pico: LED Fader ==="
PRINT "PWM-controlled LED on GP15"
PRINT ""

REM Set up pin for PWM
PINMODE 15, PWM

PRINT "Breathing effect — fading up and down"
PRINT ""

REM Fade up from 0% to 100%
PRINT "--- Fading UP ---"
FOR brightness = 0 TO 255 STEP 15
    PWMWRITE 15, brightness
    pct = INT(brightness / 255 * 100)
    PRINT "Brightness: "; pct; "%"
    SLEEP 50
NEXT brightness

PRINT ""

REM Fade down from 100% to 0%
PRINT "--- Fading DOWN ---"
FOR brightness = 255 TO 0 STEP -15
    PWMWRITE 15, brightness
    pct = INT(brightness / 255 * 100)
    PRINT "Brightness: "; pct; "%"
    SLEEP 50
NEXT brightness

PRINT ""

REM Quick pulse effect
PRINT "--- Pulse Effect ---"
FOR pulse = 1 TO 5
    PRINT "Pulse "; pulse
    FOR b = 0 TO 255 STEP 25
        PWMWRITE 15, b
        SLEEP 20
    NEXT b
    FOR b = 255 TO 0 STEP -25
        PWMWRITE 15, b
        SLEEP 20
    NEXT b
NEXT pulse

PWMWRITE 15, 0
PRINT ""
PRINT "Fading complete!"
GPIORESET
