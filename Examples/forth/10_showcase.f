\ ============================================================
\ 10 - Complete Forth Showcase
\ Demonstrates EVERY Forth feature in Time Warp
\ ============================================================

." ========================================" CR
."     Time Warp Studio - Forth Showcase" CR
." ========================================" CR CR

\ --- Stack Operations ---
." --- Stack Operations ---" CR
10 20 30
." Stack: " .S CR
DUP ." DUP: " .S CR DROP
SWAP ." SWAP: " .S CR SWAP
OVER ." OVER: " .S CR DROP
ROT ." ROT: " .S CR ROT ROT
DROP DROP DROP
CR

\ --- Arithmetic ---
." --- Arithmetic ---" CR
." 25 + 7  = " 25 7 + . CR
." 25 * 7  = " 25 7 * . CR
." 25 MOD 7 = " 25 7 MOD . CR
." ABS(-42) = " -42 ABS . CR
." MIN(5,9) = " 5 9 MIN . CR
." MAX(5,9) = " 5 9 MAX . CR
." 10 1+   = " 10 1+ . CR
." 10 2*   = " 10 2* . CR
CR

\ --- Word Definitions ---
." --- Word Definitions ---" CR
: SQUARE DUP * ;
: CUBE DUP DUP * * ;
: HYPOTENUSE ( a b -- c ) SQUARE SWAP SQUARE + ;
."   5 SQUARE = " 5 SQUARE . CR
."   3 CUBE = " 3 CUBE . CR
."   3,4 HYPOTENUSE = " 3 4 HYPOTENUSE . CR
CR

\ --- Variables & Constants ---
." --- Variables & Constants ---" CR
42 CONSTANT ANSWER
VARIABLE ACCUM
0 ACCUM !
5 0 DO I 1+ ACCUM +! LOOP
."   ANSWER = " ANSWER . CR
."   ACCUM (1+2+3+4+5) = " ACCUM @ . CR
CR

\ --- Control Flow ---
." --- IF/ELSE/THEN ---" CR
: GRADE ( n -- )
    DUP 90 >= IF ." A"
    ELSE DUP 80 >= IF ." B"
    ELSE DUP 70 >= IF ." C"
    ELSE ." F"
    THEN THEN THEN DROP ;
."   95: " 95 GRADE CR
."   82: " 82 GRADE CR
."   55: " 55 GRADE CR
CR

." --- DO/LOOP ---" CR ."   "
10 0 DO I . LOOP CR

." --- DO/+LOOP ---" CR ."   "
31 0 DO I . 5 +LOOP CR

." --- BEGIN/UNTIL ---" CR ."   "
1 BEGIN DUP . 2* DUP 1024 > UNTIL DROP CR
CR

\ --- Algorithms ---
: FACT DUP 1 <= IF DROP 1 ELSE DUP 1- FACT * THEN ;

." --- Factorials ---" CR
8 1 DO ."   " I . ." ! = " I FACT . CR LOOP
CR

\ --- String & I/O ---
." --- I/O ---" CR
."   EMIT: " 72 EMIT 73 EMIT 33 EMIT CR
."   5 SPACES: |" 5 SPACES ." |" CR
CR

\ --- Turtle Graphics ---
." --- Turtle Graphics ---" CR
4 PEN
5 0 DO 60 FD 144 RT LOOP

PU 90 RT 120 FD 90 LT PD
1 PEN
6 0 DO 40 FD 60 RT LOOP

PU HOME 90 LT 120 FD 90 RT PD
14 PEN
36 0 DO I 2 * 1+ FD 91 RT LOOP

." ========================================" CR
."     All Forth features demonstrated!" CR
." ========================================" CR
