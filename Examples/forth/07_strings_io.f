\ ============================================
\ 07 - String Output and I/O
\ Learn: ." , EMIT, SPACE, SPACES, CR
\ ============================================

." === String & I/O ===" CR CR

\ Print literal strings
." Hello, Forth!" CR
." Strings use .\" to begin and \" to end" CR
CR

\ EMIT prints a character by ASCII code
." Alphabet via EMIT:" CR
."   "
91 65 DO I EMIT LOOP
CR CR

\ SPACE and SPACES
." SPACE and SPACES:" CR
." |" SPACE ." |" CR
." |" 10 SPACES ." |" CR
CR

\ Building output character by character
." Spelling H-E-L-L-O:" CR
."   "
72 EMIT 69 EMIT 76 EMIT 76 EMIT 79 EMIT
CR CR

\ Box drawing
: HLINE ( width -- )
    0 DO 45 EMIT LOOP ;

: BOX ( width height -- )
    SWAP
    DUP HLINE CR
    SWAP 2 - 0 DO
        124 EMIT                    \ |
        DUP 2 - SPACES
        124 EMIT CR                 \ |
    LOOP
    HLINE CR ;

." Box:" CR
20 5 BOX
CR

\ Number table
: NUM-TABLE ( n -- )
    ."   N  N^2  N^3" CR
    ."   ----------- " CR
    1+ 1 DO
        ."   " I .
        I DUP * .
        I DUP DUP * * .
        CR
    LOOP ;

." Number table:" CR
8 NUM-TABLE
