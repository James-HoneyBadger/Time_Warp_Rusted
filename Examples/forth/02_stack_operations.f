\ ============================================
\ 02 - Stack Operations
\ Learn: DUP, DROP, SWAP, OVER, ROT, .S
\ ============================================

." === Stack Operations ===" CR CR

." Starting stack: 10 20 30" CR
10 20 30

." .S shows the stack: " .S CR
CR

." DUP (duplicate top):" CR
DUP
." Stack: " .S CR
DROP
CR

." SWAP (swap top two):" CR
SWAP
." Stack: " .S CR
SWAP
CR

." OVER (copy second to top):" CR
OVER
." Stack: " .S CR
DROP
CR

." ROT (rotate third to top):" CR
ROT
." Stack: " .S CR
ROT ROT
CR

." DROP (remove top):" CR
DROP
." Stack: " .S CR
CR

\ Clean up the stack
DROP DROP

." === 2DUP and 2DROP ===" CR
5 10
." Stack: " .S CR
." 2DUP: "
2DUP .S CR
." 2DROP: "
2DROP .S CR
CR

DROP DROP

." === Practical Example ===" CR
." Area of rectangle 12 x 8:" CR
12 8
." Width=" OVER . ." Height=" DUP .
CR
*
." Area=" . CR
