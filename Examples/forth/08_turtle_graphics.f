\ ============================================
\ 08 - Turtle Graphics
\ Learn: FD, BK, RT, LT, PU, PD, PEN, CS
\ ============================================

." Drawing with Forth Turtle!" CR

\ Star
4 PEN
5 0 DO 80 FD 144 RT LOOP

\ Move to draw hexagon
PU 90 RT 150 FD 90 LT PD
1 PEN
6 0 DO 50 FD 60 RT LOOP

\ Spiral
PU HOME 90 LT 150 FD 90 RT PD
5 PEN
40 0 DO
    I 3 * 1+ FD
    91 RT
LOOP

\ Circle
PU HOME 120 BK PD
13 PEN
36 0 DO 10 FD 10 RT LOOP

\ Square
PU HOME 100 FD 90 RT 80 FD 90 LT PD
9 PEN
4 0 DO 60 FD 90 RT LOOP

." Graphics complete!" CR
