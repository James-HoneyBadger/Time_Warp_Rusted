REM ============================================
REM  08 - String Functions
REM  Learn: STR$, CHR$, LEFT$, RIGHT$, MID$, LEN
REM ============================================

10 PRINT "=== String Functions ==="
20 PRINT

30 REM String assignment and concatenation
40 LET FIRST$ = "Time"
50 LET LAST$ = "Warp"
60 LET FULL$ = FIRST$ + " " + LAST$ + " Studio"
70 PRINT "Concatenation: "; FULL$
80 PRINT "Length: "; LEN(FULL$)
90 PRINT

100 REM LEFT$, RIGHT$, MID$
110 LET TEXT$ = "Hello, World!"
120 PRINT "Text: "; TEXT$
130 PRINT "LEFT$(5):    "; LEFT$(TEXT$, 5)
140 PRINT "RIGHT$(6):   "; RIGHT$(TEXT$, 6)
150 PRINT "MID$(8,5):   "; MID$(TEXT$, 8, 5)
160 PRINT

170 REM STR$ - number to string
180 LET N = 42
190 LET S$ = "The answer is " + STR$(N)
200 PRINT S$
210 PRINT "Pi as string: "; STR$(PI)
220 PRINT

230 REM CHR$ - ASCII character codes
240 PRINT "ASCII characters:"
250 FOR I = 65 TO 90
260    PRINT CHR$(I);
270 NEXT I
280 PRINT
290 PRINT "CHR$(65) = "; CHR$(65)
300 PRINT "CHR$(48) = "; CHR$(48)
310 PRINT

320 REM Building strings with functions
330 PRINT "=== String Builder ==="
340 LET RESULT$ = ""
350 FOR I = 1 TO 5
360    RESULT$ = RESULT$ + STR$(I) + " "
370 NEXT I
380 PRINT "Numbers: "; RESULT$
390 PRINT

400 REM Practical: format a report line
410 LET ITEM$ = "Widget"
420 LET QTY = 150
430 LET PRICE = 9.99
440 LET TOTAL = QTY * PRICE
450 PRINT "=== Sales Report ==="
460 PRINT "Item:     "; ITEM$
470 PRINT "Quantity: "; STR$(QTY)
480 PRINT "Price:    $"; STR$(PRICE)
490 PRINT "Total:    $"; STR$(TOTAL)
500 END
