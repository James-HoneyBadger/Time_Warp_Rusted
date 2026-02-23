REM ============================================
REM  09 - Arrays and Data Structures
REM  Learn: DIM, array access, sorting, searching
REM ============================================

10 PRINT "=== Arrays ==="
20 PRINT

30 REM Declare and fill an array
40 DIM SCORES(10)
50 RANDOMIZE TIMER
60 PRINT "Random scores:"
70 FOR I = 1 TO 10
80     SCORES(I) = INT(RND * 100) + 1
90     PRINT "  Student "; I; ": "; SCORES(I)
100 NEXT I
110 PRINT

120 REM Calculate statistics
130 LET SUM = 0
140 LET HIGHEST = 0
150 LET LOWEST = 101
160 FOR I = 1 TO 10
170    SUM = SUM + SCORES(I)
180    IF SCORES(I) > HIGHEST THEN HIGHEST = SCORES(I)
190    IF SCORES(I) < LOWEST THEN LOWEST = SCORES(I)
200 NEXT I
210 LET AVERAGE = SUM / 10
220 PRINT "=== Statistics ==="
230 PRINT "  Sum:     "; SUM
240 PRINT "  Average: "; AVERAGE
250 PRINT "  Highest: "; HIGHEST
260 PRINT "  Lowest:  "; LOWEST
270 PRINT

280 REM Bubble sort
290 PRINT "Sorting scores..."
300 FOR I = 1 TO 9
310    FOR J = 1 TO 10 - I
320       IF SCORES(J) > SCORES(J + 1) THEN
330          LET TEMP = SCORES(J)
340          SCORES(J) = SCORES(J + 1)
350          SCORES(J + 1) = TEMP
360       END IF
370    NEXT J
380 NEXT I
390 PRINT "Sorted (ascending):"
400 FOR I = 1 TO 10
410    PRINT "  "; SCORES(I);
420 NEXT I
430 PRINT
440 PRINT

450 REM Grade distribution
460 DIM GRADES(5)
470 REM GRADES: 1=A, 2=B, 3=C, 4=D, 5=F
480 FOR I = 1 TO 10
490    IF SCORES(I) >= 90 THEN
500       GRADES(1) = GRADES(1) + 1
510    ELSEIF SCORES(I) >= 80 THEN
520       GRADES(2) = GRADES(2) + 1
530    ELSEIF SCORES(I) >= 70 THEN
540       GRADES(3) = GRADES(3) + 1
550    ELSEIF SCORES(I) >= 60 THEN
560       GRADES(4) = GRADES(4) + 1
570    ELSE
580       GRADES(5) = GRADES(5) + 1
590    END IF
600 NEXT I
610 PRINT "=== Grade Distribution ==="
620 PRINT "  A (90+):  "; GRADES(1)
630 PRINT "  B (80-89):"; GRADES(2)
640 PRINT "  C (70-79):"; GRADES(3)
650 PRINT "  D (60-69):"; GRADES(4)
660 PRINT "  F (<60):  "; GRADES(5)
670 END
