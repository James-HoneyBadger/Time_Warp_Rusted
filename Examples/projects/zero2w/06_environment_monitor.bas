REM ============================================================
REM  PROJECT 06 — Environmental Monitor Dashboard
REM  Board:    Raspberry Pi Zero 2 W
REM  Hardware: Green LED=GPIO17, Amber LED=GPIO27, Red LED=GPIO22
REM            Buzzer on GPIO5 (PWM for tone)
REM  Concept:  Simulates reading temperature, humidity, and light
REM            sensors and displays a dashboard with alert LEDs.
REM  NOTE:     The Zero 2 W's WiFi could send this data to the cloud!
REM ============================================================

PRINT "=== Raspberry Pi Zero 2 W: Environment Monitor ==="
PRINT "Status LEDs: G=GPIO17, A=GPIO27, R=GPIO22"
PRINT "Buzzer: GPIO5 (alarm)"
PRINT ""

greenPin = 17
amberPin = 27
redPin = 22
buzzerPin = 5

PINMODE greenPin, OUTPUT
PINMODE amberPin, OUTPUT
PINMODE redPin, OUTPUT
PINMODE buzzerPin, PWM

SUB StatusLeds(level)
    DIGITALWRITE greenPin, LOW
    DIGITALWRITE amberPin, LOW
    DIGITALWRITE redPin, LOW
    IF level = 1 THEN DIGITALWRITE greenPin, HIGH
    IF level = 2 THEN DIGITALWRITE amberPin, HIGH
    IF level = 3 THEN DIGITALWRITE redPin, HIGH
END SUB

PRINT "╔══════════════════════════════════════════════╗"
PRINT "║       ENVIRONMENTAL MONITORING DASHBOARD     ║"
PRINT "║       Raspberry Pi Zero 2 W                 ║"
PRINT "║       (WiFi-ready for cloud reporting)       ║"
PRINT "╚══════════════════════════════════════════════╝"
PRINT ""

RANDOMIZE

REM Simulate 12 readings (one per "hour")
PRINT "Time    Temp(C)  Humidity(%)  Light(lux)  Status"
PRINT "─────   ─────── ───────────  ──────────  ──────"

DIM temps(12)
DIM humids(12)
DIM lights(12)
alerts = 0

FOR reading = 1 TO 12
    REM Simulate realistic sensor data
    REM Temperature: follows a daily curve (cooler morning, warm afternoon)
    baseTemp = 18 + 8 * SIN((reading - 3) * 0.5)
    temp = baseTemp + (RND * 3) - 1.5
    temp = INT(temp * 10) / 10

    REM Humidity: inversely related to temperature
    humidity = 70 - (temp - 18) * 2 + (RND * 10) - 5
    humidity = INT(humidity)
    IF humidity > 95 THEN humidity = 95
    IF humidity < 20 THEN humidity = 20

    REM Light: peaks midday
    lightBase = 500 * SIN((reading - 1) * 0.5)
    IF lightBase < 0 THEN lightBase = 0
    light = INT(lightBase + RND * 100)

    temps(reading) = temp
    humids(reading) = humidity
    lights(reading) = light

    REM Determine status
    IF temp > 28 THEN
        status$ = "🔴 HOT!"
        GOSUB StatusLeds(3)
        alerts = alerts + 1
        PWMWRITE buzzerPin, 128
        SLEEP 100
        PWMWRITE buzzerPin, 0
    ELSE IF temp < 12 THEN
        status$ = "🔵 COLD!"
        GOSUB StatusLeds(3)
        alerts = alerts + 1
    ELSE IF humidity > 80 THEN
        status$ = "🟡 HUMID"
        GOSUB StatusLeds(2)
    ELSE
        status$ = "🟢 OK"
        GOSUB StatusLeds(1)
    END IF

    REM Format time
    hour = 6 + reading
    IF hour < 10 THEN
        time$ = "0" + STR$(hour) + ":00"
    ELSE
        time$ = STR$(hour) + ":00"
    END IF

    PRINT time$; "   "; temp; "     "; humidity; "          "; light; "       "; status$

    SLEEP 300
NEXT reading

REM --- Summary ---
GOSUB StatusLeds(0)
PWMWRITE buzzerPin, 0

PRINT ""
PRINT "═══════════════════════════════════════════════"
PRINT "                DAILY SUMMARY"
PRINT "═══════════════════════════════════════════════"

REM Calculate averages
tempSum = 0
humSum = 0
lightSum = 0
maxTemp = temps(1)
minTemp = temps(1)

FOR i = 1 TO 12
    tempSum = tempSum + temps(i)
    humSum = humSum + humids(i)
    lightSum = lightSum + lights(i)
    IF temps(i) > maxTemp THEN maxTemp = temps(i)
    IF temps(i) < minTemp THEN minTemp = temps(i)
NEXT i

avgTemp = INT(tempSum / 12 * 10) / 10
avgHum = INT(humSum / 12)
avgLight = INT(lightSum / 12)

PRINT ""
PRINT "  Temperature:  min="; minTemp; "C  max="; maxTemp; "C  avg="; avgTemp; "C"
PRINT "  Humidity avg: "; avgHum; "%"
PRINT "  Light avg:    "; avgLight; " lux"
PRINT "  Alerts:       "; alerts
PRINT ""
PRINT "  WiFi Status:  Connected (Zero 2 W)"
PRINT "  Cloud sync:   12 readings uploaded"
PRINT ""
PRINT "Monitor complete!"
GPIORESET
