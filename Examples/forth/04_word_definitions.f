\ ============================================
\ 04 - Word Definitions
\ Learn: : ; (colon definitions), reuse
\ ============================================

." === Word Definitions ===" CR CR

\ Define a word to square a number
: SQUARE ( n -- n*n ) DUP * ;

." Squares:" CR
."   5^2 = " 5 SQUARE . CR
."   12^2 = " 12 SQUARE . CR
."   -3^2 = " -3 SQUARE . CR
CR

\ Define a word to cube a number
: CUBE ( n -- n*n*n ) DUP DUP * * ;

." Cubes:" CR
."   3^3 = " 3 CUBE . CR
."   4^3 = " 4 CUBE . CR
CR

\ Words can use other words
: SUM-OF-SQUARES ( a b -- a*a+b*b ) SQUARE SWAP SQUARE + ;

." Sum of squares:" CR
."   3^2 + 4^2 = " 3 4 SUM-OF-SQUARES . CR
."   5^2 + 12^2 = " 5 12 SUM-OF-SQUARES . CR
CR

\ Practical: temperature conversion
: F-TO-C ( f -- c ) 32 - 5 * 9 / ;
: C-TO-F ( c -- f ) 9 * 5 / 32 + ;

." Temperature conversion:" CR
."   212F = " 212 F-TO-C . ." C" CR
."   100C = " 100 C-TO-F . ." F" CR
."   32F  = " 32 F-TO-C . ." C" CR
."   0C   = " 0 C-TO-F . ." F" CR
CR

\ Factorial
: FACTORIAL ( n -- n! )
    DUP 1 <= IF DROP 1
    ELSE DUP 1- FACTORIAL *
    THEN ;

." Factorials:" CR
."   5! = " 5 FACTORIAL . CR
."   7! = " 7 FACTORIAL . CR
."   10! = " 10 FACTORIAL . CR
CR

." Done!" CR
