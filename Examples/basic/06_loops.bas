REM ============================================
REM  06 - Loops
REM  Learn: FOR/NEXT, WHILE/WEND, DO/LOOP
REM ============================================

10 PRINT "=== Loop Structures ==="
20 PRINT

30 REM FOR/NEXT with STEP
40 PRINT "Counting by 2s:"
50 FOR I = 2 TO 20 STEP 2
60     PRINT "  "; I;
70 NEXT I
80 PRINT
90 PRINT

100 REM FOR/NEXT countdown
110 PRINT "Countdown:"
120 FOR I = 10 TO 1 STEP -1
130    PRINT "  "; I; "...";
140 NEXT I
150 PRINT " Liftoff!"
160 PRINT

170 REM Nested FOR loops - multiplication table
180 PRINT "Multiplication Table (1-5):"
190 PRINT "     ";
200 FOR J = 1 TO 5
210    PRINT J; "   ";
220 NEXT J
230 PRINT
240 PRINT "    ---------------"
250 FOR I = 1 TO 5
260    PRINT I; " | ";
270    FOR J = 1 TO 5
280       LET P = I * J
290       IF P < 10 THEN PRINT " ";
300       PRINT P; "  ";
310    NEXT J
320    PRINT
330 NEXT I
340 PRINT

350 REM WHILE/WEND
360 PRINT "Fibonacci sequence (< 100):"
370 LET A = 0
380 LET B = 1
390 WHILE A < 100
400    PRINT "  "; A;
410    LET TEMP = A + B
420    A = B
430    B = TEMP
440 WEND
450 PRINT
460 PRINT

470 REM DO/LOOP UNTIL
480 PRINT "Powers of 2:"
490 LET N = 1
500 LET COUNT = 0
510 DO
520    PRINT "  2^"; COUNT; " = "; N
530    N = N * 2
540    COUNT = COUNT + 1
550 LOOP UNTIL N > 1024
560 PRINT

570 REM Sum with accumulator
580 LET SUM = 0
590 FOR I = 1 TO 100
600    SUM = SUM + I
610 NEXT I
620 PRINT "Sum of 1 to 100 = "; SUM
630 END
