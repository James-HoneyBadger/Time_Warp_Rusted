REM ============================================================
REM  PROJECT 05 — Analog Sensor Reader (Pico ADC)
REM  Board:    Raspberry Pi Pico  (has ADC on GP26-GP28)
REM  Hardware: Potentiometer on GP26 (ADC0)
REM  Wiring:   Pot outer pins → 3.3V and GND
REM            Pot wiper → GP26
REM  NOTE:     ADC is Pico-specific — not available on Pi Zero
REM ============================================================

PRINT "=== Raspberry Pi Pico: Analog Sensor Reader ==="
PRINT "Potentiometer on GP26 (ADC0)"
PRINT "NOTE: ADC is unique to the Pico!"
PRINT ""

REM The Pico has 3 ADC channels:
REM   GP26 = ADC0
REM   GP27 = ADC1
REM   GP28 = ADC2
adcPin = 26

PINMODE adcPin, INPUT

PRINT "Reading analog values..."
PRINT "(In simulator, value is 0.0 — on real Pico, 0.0 to 1.0)"
PRINT ""

PRINT "Sample   Raw Value   Voltage    Bar Graph"
PRINT "------   ---------   -------    ---------"

FOR sample = 1 TO 20
    REM Read analog value (0.0 to 1.0)
    REM On simulator this returns the pin's stored value
    sensorVal = 0.5

    REM Convert to voltage (Pico ADC reference is 3.3V)
    voltage = sensorVal * 3.3

    REM Create a simple text bar graph
    bars = INT(sensorVal * 20)
    bar$ = ""
    FOR b = 1 TO bars
        bar$ = bar$ + "#"
    NEXT b

    PRINT "  "; sample; "       "; sensorVal; "       ";
    PRINT voltage; "V    "; bar$

    SLEEP 200
NEXT sample

REM Classify the final reading
PRINT ""
IF sensorVal < 0.33 THEN
    PRINT "Sensor zone: LOW (0-33%)"
ELSE IF sensorVal < 0.66 THEN
    PRINT "Sensor zone: MEDIUM (33-66%)"
ELSE
    PRINT "Sensor zone: HIGH (66-100%)"
END IF

PRINT ""
PRINT "Analog sensor demo complete!"
GPIORESET
