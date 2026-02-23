REM =============================================
REM  Time Warp BASIC — Grand Demo
REM  The ultimate BASIC showcase
REM =============================================

REM --- Banner ---
PRINT "============================================"
PRINT "      TIME WARP BASIC - GRAND DEMO"
PRINT "============================================"
PRINT

REM === Mathematical Art with Turtle ===
PRINT ">>> Drawing Mathematical Spirograph <<<"
COLOR "magenta"
WIDTH 2

REM Spirograph pattern
FOR I = 1 TO 360
  FORWARD I / 5
  RIGHT 89
NEXT I

PENUP
HOME
PENDOWN

REM === Colourful Star Burst ===
PRINT
PRINT ">>> Drawing Rainbow Star Burst <<<"
DIM COLOURS$(6)
COLOURS$(1) = "red"
COLOURS$(2) = "orange"
COLOURS$(3) = "yellow"
COLOURS$(4) = "green"
COLOURS$(5) = "blue"
COLOURS$(6) = "purple"

FOR STAR = 1 TO 6
  COLOR COLOURS$(STAR)
  WIDTH 3
  FOR S = 1 TO 5
    FORWARD 80
    RIGHT 144
  NEXT S
  RIGHT 60
NEXT STAR

PENUP
HOME
PENDOWN

REM === Data Analysis ===
PRINT
PRINT "============================================"
PRINT "       DATA ANALYSIS MODULE"
PRINT "============================================"

DIM DATA(10)
RANDOMIZE
FOR I = 1 TO 10
  DATA(I) = INT(RND * 100) + 1
NEXT I

PRINT "Generated Data Set:"
FOR I = 1 TO 10
  PRINT "  Data["; I; "] = "; DATA(I)
NEXT I

REM Calculate statistics
SUM = 0
MIN_VAL = DATA(1)
MAX_VAL = DATA(1)
FOR I = 1 TO 10
  SUM = SUM + DATA(I)
  IF DATA(I) < MIN_VAL THEN MIN_VAL = DATA(I)
  IF DATA(I) > MAX_VAL THEN MAX_VAL = DATA(I)
NEXT I
AVERAGE = SUM / 10

PRINT
PRINT "Statistics:"
PRINT "  Sum     = "; SUM
PRINT "  Average = "; AVERAGE
PRINT "  Minimum = "; MIN_VAL
PRINT "  Maximum = "; MAX_VAL
PRINT "  Range   = "; MAX_VAL - MIN_VAL

REM Bubble sort
FOR I = 1 TO 9
  FOR J = 1 TO 10 - I
    IF DATA(J) > DATA(J + 1) THEN
      TEMP = DATA(J)
      DATA(J) = DATA(J + 1)
      DATA(J + 1) = TEMP
    END IF
  NEXT J
NEXT I

PRINT
PRINT "Sorted Data:"
FOR I = 1 TO 10
  PRINT "  ["; I; "] = "; DATA(I)
NEXT I

REM === String Manipulation ===
PRINT
PRINT "============================================"
PRINT "       STRING FUNCTIONS"
PRINT "============================================"

WORD$ = "Time Warp Studio"
PRINT "Original:  "; WORD$
PRINT "Length:    "; LEN(WORD$)
PRINT "Left 4:   "; LEFT$(WORD$, 4)
PRINT "Right 6:  "; RIGHT$(WORD$, 6)
PRINT "Mid 6,4:  "; MID$(WORD$, 6, 4)
PRINT "CHR$(65): "; CHR$(65)
PRINT "STR$(42): "; STR$(42)

REM === Mathematical Functions ===
PRINT
PRINT "============================================"
PRINT "       MATH FUNCTION TABLE"
PRINT "============================================"
PRINT "  X    SIN(X)    COS(X)    SQRT(X)   LOG(X)"
PRINT "  --   ------    ------    -------   ------"
FOR X = 1 TO 8
  PRINT "  "; X; "    ";
  PRINT SIN(X); "  ";
  PRINT COS(X); "  ";
  PRINT SQRT(X); "  ";
  PRINT LOG(X)
NEXT X

REM === GW-BASIC Graphics ===
PRINT
PRINT "============================================"
PRINT "       GW-BASIC GRAPHICS"
PRINT "============================================"
CLS
REM Night sky scene
LINE (0, 200)-(400, 200), "darkgreen"
LINE (0, 200)-(400, 300), "darkgreen", BF

REM Moon
CIRCLE (320, 60), 30, "yellow"

REM Stars
PSET (50, 30), "white"
PSET (100, 50), "white"
PSET (150, 20), "white"
PSET (200, 60), "white"
PSET (250, 25), "white"
PSET (80, 70), "white"

REM House
LINE (100, 120)-(200, 200), "brown", BF
LINE (100, 120)-(150, 80), "red"
LINE (150, 80)-(200, 120), "red"
LINE (140, 150)-(160, 200), "darkred", BF

REM Windows
LINE (110, 135)-(130, 155), "yellow", BF
LINE (170, 135)-(190, 155), "yellow", BF

PRINT "Night scene drawn!"

REM === Select Case ===
PRINT
PRINT "============================================"
PRINT "       SELECT CASE DEMO"
PRINT "============================================"

FOR MONTH = 1 TO 12
  SELECT CASE MONTH
    CASE 12, 1, 2
      SEASON$ = "Winter"
    CASE 3, 4, 5
      SEASON$ = "Spring"
    CASE 6, 7, 8
      SEASON$ = "Summer"
    CASE 9, 10, 11
      SEASON$ = "Autumn"
  END SELECT
  PRINT "  Month "; MONTH; " => "; SEASON$
NEXT MONTH

REM === Fibonacci with FUNCTION ===
PRINT
PRINT "============================================"
PRINT "       FIBONACCI SEQUENCE"
PRINT "============================================"

A = 0
B = 1
FOR I = 1 TO 15
  PRINT "  Fib("; I; ") = "; A
  C = A + B
  A = B
  B = C
NEXT I

REM === Final turtle art ===
PRINT
PRINT ">>> Drawing Hexagonal Flower <<<"
PENUP
HOME
PENDOWN
FOR P = 1 TO 6
  COLOR COLOURS$(P)
  WIDTH 2
  FOR S = 1 TO 6
    FORWARD 40
    RIGHT 60
  NEXT S
  RIGHT 60
NEXT P

PRINT
PRINT "============================================"
PRINT "    TIME WARP BASIC DEMO COMPLETE!"
PRINT "============================================"
PRINT "  Features shown:"
PRINT "    - PRINT, variables, arrays (DIM)"
PRINT "    - FOR/NEXT loops, IF/THEN/ELSE/END IF"
PRINT "    - SELECT CASE"
PRINT "    - Turtle graphics (FORWARD, RIGHT, COLOR)"
PRINT "    - GW-BASIC graphics (LINE, CIRCLE, PSET)"
PRINT "    - String functions (LEFT$, RIGHT$, MID$)"
PRINT "    - Math functions (SIN, COS, SQRT, LOG)"
PRINT "    - RANDOMIZE, RND, bubble sort"
PRINT "    - CHR$, STR$, LEN"
PRINT "============================================"
END
