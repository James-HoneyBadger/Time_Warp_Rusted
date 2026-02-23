REM ============================================================
REM  PROJECT 01 — Blink LED
REM  Board:    Raspberry Pi Pico
REM  Hardware: LED + 330 ohm resistor on GP15
REM  Wiring:   GP15 → resistor → LED anode → LED cathode → GND
REM ============================================================

PRINT "=== Raspberry Pi Pico: Blink LED ==="
PRINT "LED connected to GP15"
PRINT ""

REM Set up the LED pin as output
PINMODE 15, OUTPUT

REM Blink pattern: on for ~500ms, off for ~500ms
REM SLEEP is a timing hint — the simulator steps instantly
FOR i = 1 TO 10
    DIGITALWRITE 15, HIGH
    PRINT "LED ON  (cycle "; i; " of 10)"
    SLEEP 500

    DIGITALWRITE 15, LOW
    PRINT "LED OFF"
    SLEEP 500
NEXT i

REM Clean up
DIGITALWRITE 15, LOW
PRINT ""
PRINT "Blink complete — 10 cycles done!"
GPIORESET
