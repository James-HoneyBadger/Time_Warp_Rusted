REM ============================================================
REM  PROJECT 08 — Smart Home Light Controller
REM  Board:    Raspberry Pi Zero 2 W
REM  Hardware: 4 PWM LEDs (simulating room lights)
REM            Living room=GPIO17, Kitchen=GPIO27
REM            Bedroom=GPIO22, Hallway=GPIO5
REM            Motion sensor on GPIO6, Light sensor on GPIO13
REM  Concept:  Automated home lighting with scenes and schedules
REM            WiFi enables remote control via network
REM ============================================================

PRINT "=== Raspberry Pi Zero 2 W: Smart Home Lights ==="
PRINT "Room lights on GPIO17,27,22,5 — Motion=GPIO6"
PRINT ""

livingPin = 17
kitchenPin = 27
bedroomPin = 22
hallwayPin = 5
motionPin = 6
lightSensorPin = 13

PINMODE livingPin, PWM
PINMODE kitchenPin, PWM
PINMODE bedroomPin, PWM
PINMODE hallwayPin, PWM
PINMODE motionPin, INPUT
PINMODE lightSensorPin, INPUT

SUB AllLightsOff()
    PWMWRITE livingPin, 0
    PWMWRITE kitchenPin, 0
    PWMWRITE bedroomPin, 0
    PWMWRITE hallwayPin, 0
END SUB

SUB SetScene(living, kitchen, bedroom, hallway)
    PWMWRITE livingPin, living
    PWMWRITE kitchenPin, kitchen
    PWMWRITE bedroomPin, bedroom
    PWMWRITE hallwayPin, hallway
END SUB

PRINT "╔══════════════════════════════════════════╗"
PRINT "║     SMART HOME LIGHTING CONTROLLER       ║"
PRINT "║     Raspberry Pi Zero 2 W (WiFi)         ║"
PRINT "╚══════════════════════════════════════════╝"
PRINT ""

REM --- Scene 1: Good Morning ---
PRINT "═══ Scene: GOOD MORNING ═══"
PRINT "  Gradually brightening bedroom and kitchen..."
FOR b = 0 TO 200 STEP 10
    PWMWRITE bedroomPin, b
    IF b > 50 THEN PWMWRITE kitchenPin, b - 50
    IF b > 100 THEN PWMWRITE hallwayPin, 80
    SLEEP 50
NEXT b
PRINT "  Bedroom:  78%    Kitchen: 59%"
PRINT "  Hallway:  31%    Living:  0%"
SLEEP 1000
PRINT ""

REM --- Scene 2: Cooking ---
PRINT "═══ Scene: COOKING ═══"
PRINT "  Kitchen to full, others dimmed..."
GOSUB SetScene(80, 255, 0, 120)
PRINT "  Living:   31%    Kitchen: 100%"
PRINT "  Bedroom:  0%     Hallway: 47%"
SLEEP 1000
PRINT ""

REM --- Scene 3: Movie Night ---
PRINT "═══ Scene: MOVIE NIGHT ═══"
PRINT "  Dimming all lights..."
FOR b = 255 TO 20 STEP -10
    PWMWRITE kitchenPin, b
NEXT b
GOSUB SetScene(30, 0, 0, 15)
PRINT "  Living:   12%    Kitchen: 0%"
PRINT "  Bedroom:  0%     Hallway: 6%"
SLEEP 1000
PRINT ""

REM --- Scene 4: Goodnight ---
PRINT "═══ Scene: GOODNIGHT ═══"
PRINT "  Turning off all rooms, soft bedroom light..."
GOSUB AllLightsOff
SLEEP 200
PWMWRITE bedroomPin, 40
PWMWRITE hallwayPin, 15
PRINT "  Living:   0%     Kitchen: 0%"
PRINT "  Bedroom:  16%    Hallway: 6%"
SLEEP 800

PRINT "  Fading bedroom to sleep..."
FOR b = 40 TO 0 STEP -2
    PWMWRITE bedroomPin, b
    SLEEP 50
NEXT b
PWMWRITE hallwayPin, 0
PRINT "  All lights OFF — goodnight!"
SLEEP 1000
PRINT ""

REM --- Motion-activated hallway ---
PRINT "═══ Motion Detection Mode ═══"
PRINT "  Hallway light responds to motion sensor..."

FOR check = 1 TO 10
    motion = DIGITALREAD(motionPin)
    IF motion = 1 THEN
        PRINT "  ["; check; "] Motion detected → Hallway ON (100%)"
        PWMWRITE hallwayPin, 255
        SLEEP 500
    ELSE
        PRINT "  ["; check; "] No motion → Hallway dimming..."
        PWMWRITE hallwayPin, 20
        SLEEP 300
        PWMWRITE hallwayPin, 0
    END IF
    SLEEP 200
NEXT check

PRINT ""

REM --- Departure mode ---
PRINT "═══ Scene: LEAVING HOME ═══"
PRINT "  Shutting down all lights..."
GOSUB AllLightsOff
PRINT "  All lights OFF"
PRINT "  WiFi status: System armed, remote monitoring active"
PRINT ""
PRINT "Smart home lighting demo complete!"
GPIORESET
