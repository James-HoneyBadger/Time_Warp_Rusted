\ ============================================
\ 06 - Variables and Constants
\ Learn: VARIABLE, CONSTANT, @, !, +!
\ ============================================

." === Variables & Constants ===" CR CR

\ Constants
42 CONSTANT ANSWER
100 CONSTANT MAX-SCORE

." Constants:" CR
."   ANSWER = " ANSWER . CR
."   MAX-SCORE = " MAX-SCORE . CR
CR

\ Variables
VARIABLE COUNTER
VARIABLE TOTAL

\ Store values with !
0 COUNTER !
0 TOTAL !

." Initial counter: " COUNTER @ . CR
CR

\ Increment with +!
." Counting up:" CR
10 0 DO
    1 COUNTER +!
    I 1+ TOTAL +!
    ."   Step " COUNTER @ . ."  Total=" TOTAL @ . CR
LOOP
CR

." Final counter: " COUNTER @ . CR
." Final total: " TOTAL @ . CR
CR

\ Practical: running average
VARIABLE SUM
VARIABLE COUNT
0 SUM !
0 COUNT !

: ADD-VALUE ( n -- )
    SUM +!
    1 COUNT +! ;

: AVERAGE ( -- avg )
    SUM @ COUNT @ / ;

." Running average:" CR
10 ADD-VALUE
20 ADD-VALUE
30 ADD-VALUE
40 ADD-VALUE
50 ADD-VALUE
."   Values: 10, 20, 30, 40, 50" CR
."   Count: " COUNT @ . CR
."   Sum: " SUM @ . CR
."   Average: " AVERAGE . CR
