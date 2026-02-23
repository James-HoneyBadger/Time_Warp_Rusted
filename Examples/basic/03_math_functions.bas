REM ============================================
REM  03 - Math Functions
REM  Learn: SIN, COS, SQRT, ABS, INT, RND, PI
REM ============================================

10 PRINT "=== Math Functions ==="
20 PRINT

30 REM Constants
40 PRINT "PI = "; PI
50 PRINT "E  = "; E
60 PRINT

70 REM Square root and powers
80 PRINT "SQRT(144)  = "; SQRT(144)
90 PRINT "SQRT(2)    = "; SQRT(2)
100 PRINT "2 ^ 10     = "; 2 ^ 10
110 PRINT "POW(3, 4)  = "; POW(3, 4)
120 PRINT

130 REM Trigonometry (degrees)
140 PRINT "SIN(30)  = "; SIN(30)
150 PRINT "COS(60)  = "; COS(60)
160 PRINT "TAN(45)  = "; TAN(45)
170 PRINT "ASIN(0.5)= "; ASIN(0.5)
180 PRINT "ATAN(1)  = "; ATAN(1)
190 PRINT

200 REM Rounding and truncation
210 LET X = 7.68
220 PRINT "X = "; X
230 PRINT "INT(X)   = "; INT(X)
240 PRINT "FIX(X)   = "; FIX(X)
250 PRINT "FLOOR(X) = "; FLOOR(X)
260 PRINT "CEIL(X)  = "; CEIL(X)
270 PRINT "ROUND(X) = "; ROUND(X)
280 PRINT "ABS(-42) = "; ABS(-42)
290 PRINT "SGN(-5)  = "; SGN(-5)
300 PRINT

310 REM Logarithms
320 PRINT "LOG(E)   = "; LOG(E)
330 PRINT "LOG10(100) = "; LOG10(100)
340 PRINT "EXP(1)   = "; EXP(1)
350 PRINT

360 REM Random numbers
370 RANDOMIZE TIMER
380 PRINT "5 random numbers (0-99):"
390 FOR I = 1 TO 5
400     PRINT "  "; INT(RND * 100);
410 NEXT I
420 PRINT
430 PRINT

440 REM Min and Max
450 PRINT "MIN(17, 42) = "; MIN(17, 42)
460 PRINT "MAX(17, 42) = "; MAX(17, 42)
470 END
