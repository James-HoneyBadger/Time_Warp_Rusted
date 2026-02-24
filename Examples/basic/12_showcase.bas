REM ============================================
REM  12 - Complete Showcase
REM  Demonstrates EVERY BASIC feature in one program
REM ============================================

PRINT "======================================="
PRINT "  Time Warp Rusted - BASIC Showcase"
PRINT "======================================="
PRINT

REM --- Variables & Math ---
PRINT "--- Variables & Math ---"
LET A = 25
LET B = 7
PI_VAL = PI
PRINT "A="; A; " B="; B; " PI="; PI_VAL
PRINT "A+B="; A+B; " A*B="; A*B; " A^2="; A^2
PRINT "SQRT(A)="; SQRT(A); " ABS(-B)="; ABS(-B)
PRINT "SIN(45)="; SIN(45); " COS(45)="; COS(45)
PRINT "LOG10(1000)="; LOG10(1000)
PRINT "MIN(A,B)="; MIN(A,B); " MAX(A,B)="; MAX(A,B)
PRINT

REM --- Strings ---
PRINT "--- String Functions ---"
LET MSG$ = "Hello, BASIC World!"
PRINT "String:  "; MSG$
PRINT "Length:  "; LEN(MSG$)
PRINT "Left 5:  "; LEFT$(MSG$, 5)
PRINT "Right 6: "; RIGHT$(MSG$, 6)
PRINT "Mid 8,5: "; MID$(MSG$, 8, 5)
PRINT "STR$(42):"; STR$(42)
PRINT "CHR$(65):"; CHR$(65)
PRINT "Concat:  "; "A" + "B" + "C"
PRINT

REM --- Arrays ---
PRINT "--- Arrays ---"
DIM DATA(5)
FOR I = 1 TO 5
    DATA(I) = I * I
NEXT I
PRINT "Squares: ";
FOR I = 1 TO 5
    PRINT DATA(I); " ";
NEXT I
PRINT
PRINT

REM --- Control Flow ---
PRINT "--- Conditionals ---"
LET TEMP = 72
IF TEMP > 80 THEN
    PRINT "It's hot!"
ELSEIF TEMP > 60 THEN
    PRINT "It's pleasant ("; TEMP; " degrees)"
ELSE
    PRINT "It's cold!"
END IF
PRINT

PRINT "--- SELECT CASE ---"
LET MONTH = 6
SELECT CASE MONTH
CASE 1
    PRINT "January"
CASE 6
    PRINT "June - Summer!"
CASE 12
    PRINT "December - Winter!"
CASE ELSE
    PRINT "Month #"; MONTH
END SELECT
PRINT

REM --- Loops ---
PRINT "--- FOR/NEXT ---"
PRINT "Evens: ";
FOR I = 2 TO 20 STEP 2
    PRINT I; " ";
NEXT I
PRINT

PRINT "--- WHILE/WEND ---"
LET FIB_A = 0
LET FIB_B = 1
PRINT "Fibonacci: ";
WHILE FIB_A < 50
    PRINT FIB_A; " ";
    LET T = FIB_A + FIB_B
    FIB_A = FIB_B
    FIB_B = T
WEND
PRINT
PRINT

REM --- Subroutines ---
PRINT "--- GOSUB/RETURN ---"
GOSUB 900
PRINT

REM --- Random Numbers ---
PRINT "--- Random ---"
RANDOMIZE TIMER
PRINT "Random: ";
FOR I = 1 TO 5
    PRINT INT(RND * 100); " ";
NEXT I
PRINT
PRINT

REM --- Turtle Graphics ---
PRINT "--- Drawing Graphics ---"
COLOR RED
WIDTH 3
FOR I = 1 TO 5
    FORWARD 80
    RIGHT 144
NEXT I
PENUP
HOME
RIGHT 90
FORWARD 120
LEFT 90
PENDOWN
COLOR BLUE
WIDTH 2
FOR I = 1 TO 6
    FORWARD 50
    RIGHT 60
NEXT I

PRINT
PRINT "======================================="
PRINT "  All BASIC features demonstrated!"
PRINT "======================================="
END

REM --- Subroutine section ---
900 PRINT "  Inside subroutine via GOSUB"
910 PRINT "  Variables: A="; A; " B="; B
920 RETURN
