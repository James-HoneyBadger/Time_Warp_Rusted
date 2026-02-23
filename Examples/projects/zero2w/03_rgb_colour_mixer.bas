REM ============================================================
REM  PROJECT 03 — RGB LED Colour Mixer (PWM)
REM  Board:    Raspberry Pi Zero 2 W
REM  Hardware: Common-cathode RGB LED
REM            Red on GPIO17 (PWM), Green on GPIO27, Blue on GPIO22
REM  Wiring:   GPIO17 → 220Ω → Red pin
REM            GPIO27 → 220Ω → Green pin
REM            GPIO22 → 220Ω → Blue pin
REM            Common cathode → GND
REM ============================================================

PRINT "=== Raspberry Pi Zero 2 W: RGB Colour Mixer ==="
PRINT "R=GPIO17  G=GPIO27  B=GPIO22 (all PWM)"
PRINT ""

redPin = 17
greenPin = 27
bluePin = 22

PINMODE redPin, PWM
PINMODE greenPin, PWM
PINMODE bluePin, PWM

SUB SetColour(r, g, b)
    PWMWRITE redPin, r
    PWMWRITE greenPin, g
    PWMWRITE bluePin, b
END SUB

REM --- Named colours ---
PRINT "--- Named Colours ---"
PRINT ""

GOSUB SetColour(255, 0, 0)
PRINT "  RED     (255, 0, 0)"
SLEEP 800

GOSUB SetColour(0, 255, 0)
PRINT "  GREEN   (0, 255, 0)"
SLEEP 800

GOSUB SetColour(0, 0, 255)
PRINT "  BLUE    (0, 0, 255)"
SLEEP 800

GOSUB SetColour(255, 255, 0)
PRINT "  YELLOW  (255, 255, 0)"
SLEEP 800

GOSUB SetColour(255, 0, 255)
PRINT "  MAGENTA (255, 0, 255)"
SLEEP 800

GOSUB SetColour(0, 255, 255)
PRINT "  CYAN    (0, 255, 255)"
SLEEP 800

GOSUB SetColour(255, 128, 0)
PRINT "  ORANGE  (255, 128, 0)"
SLEEP 800

GOSUB SetColour(255, 255, 255)
PRINT "  WHITE   (255, 255, 255)"
SLEEP 800

GOSUB SetColour(0, 0, 0)
PRINT ""

REM --- Rainbow fade ---
PRINT "--- Rainbow Fade ---"
PRINT "Smoothly cycling through the colour wheel..."
PRINT ""

REM Simplified rainbow: R→Y→G→C→B→M→R
REM Phase 1: Red to Yellow (increase green)
PRINT "  Red → Yellow"
FOR g = 0 TO 255 STEP 5
    GOSUB SetColour(255, g, 0)
    SLEEP 15
NEXT g

REM Phase 2: Yellow to Green (decrease red)
PRINT "  Yellow → Green"
FOR r = 255 TO 0 STEP -5
    GOSUB SetColour(r, 255, 0)
    SLEEP 15
NEXT r

REM Phase 3: Green to Cyan (increase blue)
PRINT "  Green → Cyan"
FOR b = 0 TO 255 STEP 5
    GOSUB SetColour(0, 255, b)
    SLEEP 15
NEXT b

REM Phase 4: Cyan to Blue (decrease green)
PRINT "  Cyan → Blue"
FOR g = 255 TO 0 STEP -5
    GOSUB SetColour(0, g, 255)
    SLEEP 15
NEXT g

REM Phase 5: Blue to Magenta (increase red)
PRINT "  Blue → Magenta"
FOR r = 0 TO 255 STEP 5
    GOSUB SetColour(r, 0, 255)
    SLEEP 15
NEXT r

REM Phase 6: Magenta to Red (decrease blue)
PRINT "  Magenta → Red"
FOR b = 255 TO 0 STEP -5
    GOSUB SetColour(255, 0, b)
    SLEEP 15
NEXT b

GOSUB SetColour(0, 0, 0)

PRINT ""
PRINT "RGB colour mixer complete!"
GPIORESET
