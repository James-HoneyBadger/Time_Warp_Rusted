\ =============================================
\ Time Warp Forth — Grand Demo
\ A comprehensive Forth showcase
\ =============================================

." ============================================" CR
."       TIME WARP FORTH - GRAND DEMO" CR
." ============================================" CR CR

\ === Section 1: Stack Operations ===
." --- Section 1: Stack Operations ---" CR
42 DUP ." DUP 42:     " . . CR
10 20 SWAP ." SWAP 10 20: " . . CR
1 2 3 ROT ." ROT 1 2 3:  " . . . CR
5 6 2DUP ." 2DUP 5 6:   " . . . . CR
99 DROP ." DROP 99:     (stack clear)" CR
CR

\ === Section 2: Arithmetic ===
." --- Section 2: Arithmetic ---" CR
100 37 + ." 100 + 37 = " . CR
100 37 - ." 100 - 37 = " . CR
12 9 * ." 12 * 9  = " . CR
144 12 / ." 144 / 12 = " . CR
17 5 MOD ." 17 MOD 5 = " . CR
-42 ABS ." ABS(-42) = " . CR
7 NEGATE ." NEGATE 7 = " . CR
3 8 MIN ." MIN(3,8) = " . CR
3 8 MAX ." MAX(3,8) = " . CR
10 1+ ." 10 1+   = " . CR
20 2* ." 20 2*   = " . CR
CR

\ === Section 3: Word Definitions ===
." --- Section 3: Custom Words ---" CR

: SQUARE DUP * ;
: CUBE DUP DUP * * ;
: DOUBLE 2 * ;
: HALVE 2 / ;
: NEGATE? 0 < ;

." 7 SQUARE = " 7 SQUARE . CR
." 4 CUBE   = " 4 CUBE . CR
." 25 DOUBLE = " 25 DOUBLE . CR
." 100 HALVE = " 100 HALVE . CR
CR

\ Factorial
: FACTORIAL
  DUP 1 <= IF DROP 1
  ELSE DUP 1 - FACTORIAL *
  THEN ;

." Factorials:" CR
1 FACTORIAL ."   1! = " . CR
5 FACTORIAL ."   5! = " . CR
7 FACTORIAL ."   7! = " . CR
10 FACTORIAL ."  10! = " . CR
CR

\ === Section 4: Control Flow ===
." --- Section 4: Control Flow ---" CR

\ IF/ELSE/THEN
." IF/ELSE: " CR
42 0 > IF ."   42 is positive" CR THEN
-5 0 < IF ."   -5 is negative" CR THEN
7 2 MOD 0 = IF ."   7 is even" CR ELSE ."   7 is odd" CR THEN
CR

\ DO LOOP
." DO LOOP (1 to 10):" CR ."   "
10 1 DO I . SPACE LOOP CR

\ DO +LOOP (step)
." +LOOP (0 to 100 by 10):" CR ."   "
101 0 DO I . SPACE 10 +LOOP CR

\ Nested DO LOOP
." Nested (multiplication):" CR
4 1 DO
  ."   "
  4 1 DO
    I J * 4 .R
  LOOP CR
LOOP
CR

\ BEGIN UNTIL
." BEGIN UNTIL (powers of 2):" CR ."   "
1
BEGIN
  DUP .
  2 *
  DUP 1024 >
UNTIL DROP CR
CR

\ === Section 5: Variables & Constants ===
." --- Section 5: Variables & Constants ---" CR

VARIABLE COUNTER
VARIABLE TOTAL

0 COUNTER !
0 TOTAL !

10 1 DO
  I COUNTER !
  TOTAL @ I + TOTAL !
LOOP

."   Counter: " COUNTER @ . CR
."   Total (1-9): " TOTAL @ . CR

CONSTANT DOZEN 12
CONSTANT CENTURY 100
."   DOZEN:   " DOZEN . CR
."   CENTURY: " CENTURY . CR
CR

\ === Section 6: String Output ===
." --- Section 6: String Output ---" CR
."   "
42 EMIT 42 EMIT 42 EMIT   \ ***
CR

."   Spaces demo: [" 10 SPACES ." ]" CR

\ Box drawing
: HLINE 20 0 DO 45 EMIT LOOP ;
."   +" HLINE ." +" CR
."   |  TIME WARP FORTH   |" CR
."   +" HLINE ." +" CR
CR

\ === Section 7: Turtle Graphics ===
." --- Section 7: Turtle Graphics ---" CR

\ Star
." Drawing star..." CR
1 PEN
5 0 DO
  80 FD
  144 RT
LOOP

\ Hexagon
." Drawing hexagon..." CR
3 PEN
6 0 DO
  50 FD
  60 RT
LOOP

\ Spiral
." Drawing spiral..." CR
5 PEN
36 0 DO
  I 2 * FD
  20 RT
LOOP
CR

\ === Section 8: Algorithms ===
." --- Section 8: Algorithms ---" CR

\ Fibonacci
: FIB-PRINT
  ."   Fibonacci: "
  0 1
  12 0 DO
    OVER .
    OVER + SWAP
  LOOP
  DROP DROP CR ;
FIB-PRINT

\ GCD
: GCD
  BEGIN
    DUP
  WHILE
    SWAP OVER MOD
  REPEAT
  DROP ;

."   GCD(48,36) = " 48 36 GCD . CR
."   GCD(100,75) = " 100 75 GCD . CR
."   GCD(17,5)  = " 17 5 GCD . CR

\ Prime check
: PRIME?
  DUP 2 < IF DROP 0
  ELSE
    DUP 2 = IF DROP 1
    ELSE
      1
      OVER 2 / 1 + 2 DO
        OVER I MOD 0 = IF DROP 0 LEAVE THEN
      LOOP
      SWAP DROP
    THEN
  THEN ;

." Primes to 30:" CR ."   "
31 2 DO
  I PRIME? IF I . THEN
LOOP CR

\ Power
: POWER
  1 SWAP
  0 DO
    OVER *
  LOOP
  SWAP DROP ;

CR
."   2^10 = " 2 10 POWER . CR
."   3^5  = " 3 5 POWER . CR
CR

\ === Finale ===
." ============================================" CR
."       GRAND DEMO COMPLETE!" CR
." ============================================" CR
."   Features demonstrated:" CR
."     - Stack ops (DUP SWAP ROT DROP OVER)" CR
."     - Arithmetic (+  - * / MOD ABS MIN MAX)" CR
."     - Word definitions (: ; RECURSIVE)" CR
."     - IF/ELSE/THEN conditionals" CR
."     - DO/LOOP, DO/+LOOP, BEGIN/UNTIL" CR
."     - VARIABLE, CONSTANT, @ ! +!" CR
."     - String output (." ." EMIT SPACE SPACES)" CR
."     - Turtle graphics (FD BK RT LT PEN)" CR
."     - Algorithms (factorial, GCD, primes)" CR
." ============================================" CR
