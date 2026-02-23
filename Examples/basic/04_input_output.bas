REM ============================================
REM  04 - Input and Output
REM  Learn: INPUT, PRINT formatting, interaction
REM ============================================

10 PRINT "=== Interactive Calculator ==="
20 PRINT
30 INPUT "Enter your name: "; NAME$
40 PRINT "Hello, "; NAME$; "!"
50 PRINT
60 INPUT "Enter first number: "; A
70 INPUT "Enter second number: "; B
80 PRINT
90 PRINT "=== Results ==="
100 PRINT A; " + "; B; " = "; A + B
110 PRINT A; " - "; B; " = "; A - B
120 PRINT A; " * "; B; " = "; A * B
130 IF B <> 0 THEN PRINT A; " / "; B; " = "; A / B
140 IF B = 0 THEN PRINT "Cannot divide by zero!"
150 PRINT A; " ^ "; B; " = "; A ^ B
160 PRINT
170 PRINT "Thank you, "; NAME$; "!"
180 END
