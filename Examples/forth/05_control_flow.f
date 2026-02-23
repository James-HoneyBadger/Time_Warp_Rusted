\ ============================================
\ 05 - Control Flow
\ Learn: IF/ELSE/THEN, BEGIN/UNTIL, DO/LOOP
\ ============================================

." === Control Flow ===" CR CR

\ IF / ELSE / THEN
: SIGN ( n -- )
    DUP 0 > IF ." positive"
    ELSE DUP 0 < IF ." negative"
    ELSE ." zero"
    THEN THEN DROP ;

." Sign test:" CR
."   42: " 42 SIGN CR
."   -7: " -7 SIGN CR
."   0:  " 0 SIGN CR
CR

\ Classify a number
: CLASSIFY ( n -- )
    DUP 0 > IF
        DUP 2 MOD 0= IF ." positive even"
        ELSE ." positive odd"
        THEN
    ELSE ." non-positive"
    THEN DROP ;

." Classify:" CR
."   6: " 6 CLASSIFY CR
."   7: " 7 CLASSIFY CR
."   -3: " -3 CLASSIFY CR
CR

\ DO / LOOP
." DO/LOOP (0 to 9):" CR
."   "
10 0 DO I . LOOP CR
CR

\ DO / +LOOP (step 3)
." +LOOP (0 to 30 step 3):" CR
."   "
31 0 DO I . 3 +LOOP CR
CR

\ Nested DO/LOOP
." Nested loops (I*J):" CR
4 1 DO
    4 1 DO
        I J * . SPACE
    LOOP CR
LOOP
CR

\ BEGIN / UNTIL
." BEGIN/UNTIL (countdown):" CR
."   "
10 BEGIN DUP . 1- DUP 0= UNTIL DROP
." Liftoff!" CR
CR

." Done!" CR
