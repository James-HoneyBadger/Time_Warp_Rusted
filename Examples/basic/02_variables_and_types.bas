REM ============================================
REM  02 - Variables and Data Types
REM  Learn: LET, numeric vars, string vars ($)
REM ============================================

10 REM Numeric variables
20 LET X = 42
30 LET PI = 3.14159
40 LET RADIUS = 10
50 PRINT "X = "; X
60 PRINT "PI = "; PI
70 PRINT "Radius = "; RADIUS
80 PRINT

90 REM You can omit LET
100 AREA = PI * RADIUS * RADIUS
110 CIRCUMFERENCE = 2 * PI * RADIUS
120 PRINT "Circle with radius "; RADIUS; ":"
130 PRINT "  Area = "; AREA
140 PRINT "  Circumference = "; CIRCUMFERENCE
150 PRINT

160 REM String variables end with $
170 LET NAME$ = "Time Warp"
180 LET VERSION$ = "Studio"
190 LET GREETING$ = NAME$ + " " + VERSION$
200 PRINT "Name: "; NAME$
210 PRINT "Version: "; VERSION$
220 PRINT "Full: "; GREETING$
230 PRINT

240 REM Arithmetic operators
250 LET A = 25
260 LET B = 7
270 PRINT A; " + "; B; " = "; A + B
280 PRINT A; " - "; B; " = "; A - B
290 PRINT A; " * "; B; " = "; A * B
300 PRINT A; " / "; B; " = "; A / B
310 PRINT A; " MOD "; B; " = "; A MOD B
320 PRINT A; " ^ 2 = "; A ^ 2
330 END
