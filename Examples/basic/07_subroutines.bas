REM ============================================
REM  07 - Subroutines and Functions
REM  Learn: GOSUB/RETURN, SUB, FUNCTION, CALL
REM ============================================

10 PRINT "=== Subroutines & Functions ==="
20 PRINT

30 REM GOSUB/RETURN (classic BASIC style)
40 PRINT "--- Classic GOSUB ---"
50 GOSUB 500
60 PRINT "Back from subroutine!"
70 PRINT

80 REM SUB with parameters
90 PRINT "--- Named SUB ---"
100 CALL PrintBanner("Time Warp Studio", 30)
110 PRINT

120 REM FUNCTION with return value
130 PRINT "--- Named FUNCTION ---"
140 LET R = 5
150 LET A = CircleArea(R)
160 PRINT "Circle area (r="; R; ") = "; A
170 PRINT "Factorial(7) = "; Factorial(7)
180 PRINT

190 REM Using functions in expressions
200 PRINT "--- Functions in Expressions ---"
210 FOR I = 1 TO 10
220    PRINT "  "; I; "! = "; Factorial(I)
230 NEXT I
240 PRINT

250 PRINT "--- Star Pattern ---"
260 FOR ROW = 1 TO 5
270    CALL PrintStars(ROW)
280 NEXT ROW
290 FOR ROW = 4 TO 1 STEP -1
300    CALL PrintStars(ROW)
310 NEXT ROW
320 PRINT

330 PRINT "Done!"
340 END

REM ---- Classic subroutine ----
500 PRINT "  Inside the subroutine!"
510 PRINT "  GOSUB jumps here, RETURN goes back."
520 RETURN

REM ---- Named subroutines ----
SUB PrintBanner(TEXT$, WIDTH)
    LET PAD = (WIDTH - LEN(TEXT$)) / 2
    FOR I = 1 TO WIDTH
        PRINT "=";
    NEXT I
    PRINT
    FOR I = 1 TO PAD
        PRINT " ";
    NEXT I
    PRINT TEXT$
    FOR I = 1 TO WIDTH
        PRINT "=";
    NEXT I
    PRINT
END SUB

SUB PrintStars(N)
    FOR I = 1 TO N
        PRINT "*";
    NEXT I
    PRINT
END SUB

FUNCTION CircleArea(R)
    CircleArea = PI * R * R
END FUNCTION

FUNCTION Factorial(N)
    IF N <= 1 THEN
        Factorial = 1
    ELSE
        Factorial = N * Factorial(N - 1)
    END IF
END FUNCTION
