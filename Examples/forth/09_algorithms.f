\ ============================================
\ 09 - Algorithms
\ Learn: Fibonacci, primes, GCD, factorials
\ ============================================

." === Forth Algorithms ===" CR CR

\ Fibonacci
: FIB ( n -- fib )
    DUP 1 <= IF
    ELSE
        DUP 1- FIB
        SWAP 2 - FIB +
    THEN ;

." Fibonacci:" CR ."   "
12 0 DO I FIB . LOOP CR CR

\ Factorial
: FACT ( n -- n! )
    DUP 1 <= IF DROP 1
    ELSE DUP 1- FACT *
    THEN ;

." Factorials:" CR
10 1 DO
    ."   " I . ." ! = " I FACT . CR
LOOP
CR

\ Prime check
: PRIME? ( n -- flag )
    DUP 2 < IF DROP 0
    ELSE
        DUP 2 = IF DROP -1
        ELSE
            -1 SWAP                 ( flag n )
            DUP 2 DO
                DUP I MOD 0= IF
                    SWAP DROP 0 SWAP
                    LEAVE
                THEN
            LOOP DROP
        THEN
    THEN ;

." Primes to 50:" CR ."   "
51 2 DO I PRIME? IF I . THEN LOOP CR CR

\ GCD using Euclid's algorithm
: GCD ( a b -- gcd )
    BEGIN DUP WHILE
        SWAP OVER MOD
    REPEAT DROP ;

." GCD:" CR
."   GCD(48,18) = " 48 18 GCD . CR
."   GCD(100,75) = " 100 75 GCD . CR
."   GCD(17,13) = " 17 13 GCD . CR
CR

\ Power
: POWER ( base exp -- result )
    1 SWAP 0 DO OVER * LOOP NIP ;

." Powers of 2:" CR
11 0 DO
    ."   2^" I . ." = " 2 I POWER . CR
LOOP
CR

." Done!" CR
