REM ============================================================
REM  PROJECT 09 — Morse Code Transmitter
REM  Board:    Raspberry Pi Zero 2 W
REM  Hardware: LED on GPIO17, Buzzer on GPIO18 (PWM)
REM  Wiring:   GPIO17 → 330Ω → LED → GND
REM            GPIO18 → piezo buzzer → GND
REM  Concept:  Encodes text messages into Morse code,
REM            blinking the LED and buzzing in sync
REM ============================================================

PRINT "=== Raspberry Pi Zero 2 W: Morse Code ==="
PRINT "LED=GPIO17, Buzzer=GPIO18"
PRINT ""

ledPin = 17
buzzerPin = 18

PINMODE ledPin, OUTPUT
PINMODE buzzerPin, PWM

REM Timing (in ms) — based on standard Morse timing
dotTime = 150
dashTime = 450
elementGap = 150
letterGap = 450
wordGap = 1050

SUB MorseDot()
    DIGITALWRITE ledPin, HIGH
    PWMWRITE buzzerPin, 128
    SLEEP dotTime
    DIGITALWRITE ledPin, LOW
    PWMWRITE buzzerPin, 0
    SLEEP elementGap
END SUB

SUB MorseDash()
    DIGITALWRITE ledPin, HIGH
    PWMWRITE buzzerPin, 128
    SLEEP dashTime
    DIGITALWRITE ledPin, LOW
    PWMWRITE buzzerPin, 0
    SLEEP elementGap
END SUB

PRINT "╔══════════════════════════════════════╗"
PRINT "║      MORSE CODE TRANSMITTER          ║"
PRINT "║                                      ║"
PRINT "║  .  = short flash (dot)              ║"
PRINT "║  -  = long flash (dash)              ║"
PRINT "╚══════════════════════════════════════╝"
PRINT ""

REM --- Morse Code Table ---
PRINT "Morse Code Reference:"
PRINT "  A .-    B -...  C -.-.  D -.."
PRINT "  E .     F ..-.  G --.   H ...."
PRINT "  I ..    J .---  K -.-   L .-.."
PRINT "  M --    N -.    O ---   P .--."
PRINT "  Q --.-  R .-.   S ...   T -"
PRINT "  U ..-   V ...-  W .--   X -..-"
PRINT "  Y -.--  Z --.."
PRINT "  1 .---- 2 ..--- 3 ...-- 4 ....-"
PRINT "  5 ..... 6 -.... 7 --... 8 ---.."
PRINT "  9 ----. 0 -----"
PRINT ""

REM --- Transmit "SOS" ---
PRINT "═══ Transmitting: SOS ═══"
PRINT ""

REM S = ...
PRINT "  S = ...";
GOSUB MorseDot
GOSUB MorseDot
GOSUB MorseDot
SLEEP letterGap
PRINT ""

REM O = ---
PRINT "  O = ---";
GOSUB MorseDash
GOSUB MorseDash
GOSUB MorseDash
SLEEP letterGap
PRINT ""

REM S = ...
PRINT "  S = ...";
GOSUB MorseDot
GOSUB MorseDot
GOSUB MorseDot
PRINT ""
PRINT ""

SLEEP wordGap

REM --- Transmit "HI" ---
PRINT "═══ Transmitting: HI ═══"
PRINT ""

REM H = ....
PRINT "  H = ....";
GOSUB MorseDot
GOSUB MorseDot
GOSUB MorseDot
GOSUB MorseDot
SLEEP letterGap
PRINT ""

REM I = ..
PRINT "  I = ..";
GOSUB MorseDot
GOSUB MorseDot
PRINT ""
PRINT ""

SLEEP wordGap

REM --- Transmit "PI" ---
PRINT "═══ Transmitting: PI ═══"
PRINT ""

REM P = .--.
PRINT "  P = .--.";
GOSUB MorseDot
GOSUB MorseDash
GOSUB MorseDash
GOSUB MorseDot
SLEEP letterGap
PRINT ""

REM I = ..
PRINT "  I = ..";
GOSUB MorseDot
GOSUB MorseDot
PRINT ""

SLEEP 500

PRINT ""
PRINT "Morse code transmission complete!"
PRINT "The LED and buzzer flashed each letter."
DIGITALWRITE ledPin, LOW
PWMWRITE buzzerPin, 0
GPIORESET
