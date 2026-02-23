REM ============================================
REM  05 - Conditionals
REM  Learn: IF/THEN/ELSE, block IF, SELECT CASE
REM ============================================

10 PRINT "=== Conditionals ==="
20 PRINT

30 REM Single-line IF/THEN/ELSE
40 LET SCORE = 85
50 IF SCORE >= 90 THEN PRINT "Grade: A" ELSE PRINT "Grade: Below A"
60 PRINT

70 REM Multi-line IF/THEN/ELSE block
80 PRINT "Checking score "; SCORE; ":"
90 IF SCORE >= 90 THEN
100    PRINT "  Outstanding! Grade A"
110    PRINT "  You made the honor roll!"
120 ELSEIF SCORE >= 80 THEN
130    PRINT "  Great work! Grade B"
140    PRINT "  Keep it up!"
150 ELSEIF SCORE >= 70 THEN
160    PRINT "  Good job! Grade C"
170 ELSEIF SCORE >= 60 THEN
180    PRINT "  Grade D - study harder"
190 ELSE
200    PRINT "  Grade F - see teacher"
210 END IF
220 PRINT

230 REM Nested IF blocks
240 LET AGE = 16
250 LET HAS_LICENSE = 0
260 IF AGE >= 16 THEN
270    PRINT "Old enough to drive."
280    IF HAS_LICENSE = 1 THEN
290       PRINT "You have a license - drive safe!"
300    ELSE
310       PRINT "But you need a license first."
320    END IF
330 ELSE
340    PRINT "Too young to drive."
350 END IF
360 PRINT

370 REM Logical operators
380 LET X = 15
390 IF X > 10 AND X < 20 THEN PRINT X; " is between 10 and 20"
400 IF X = 10 OR X = 15 THEN PRINT X; " is either 10 or 15"
410 IF NOT (X > 20) THEN PRINT X; " is not greater than 20"
420 PRINT

430 REM SELECT CASE
440 LET DAY = 3
450 PRINT "Day "; DAY; " of the week:"
460 SELECT CASE DAY
470 CASE 1
480    PRINT "  Monday"
490 CASE 2
500    PRINT "  Tuesday"
510 CASE 3
520    PRINT "  Wednesday"
530 CASE 4
540    PRINT "  Thursday"
550 CASE 5
560    PRINT "  Friday"
570 CASE ELSE
580    PRINT "  Weekend!"
590 END SELECT
600 END
