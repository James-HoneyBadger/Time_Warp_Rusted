# Language Reference

Complete syntax and command reference for all seven languages in Time Warp
Studio.

---

## Table of Contents

1. [BASIC](#basic)
2. [Logo](#logo)
3. [C (Subset)](#c-subset)
4. [Pascal](#pascal)
5. [Forth](#forth)
6. [PILOT](#pilot)
7. [Prolog](#prolog)
8. [Expression Evaluator](#expression-evaluator)

---

## BASIC

Time Warp Rusted implements a **Turbo BASIC** dialect with modern
extensions. Programs can use line numbers (optional) or structured code.

### Output

| Command | Description | Example |
|---------|-------------|---------|
| `PRINT expr` | Print with newline | `PRINT "Hello"` |
| `PRINT expr;` | Print without newline | `PRINT "Name: ";` |
| `PRINT expr, expr` | Print comma-separated | `PRINT "X =", X` |

String interpolation: `PRINT "Hello {NAME$}!"` substitutes variables
enclosed in `{…}`.

### Variables

| Syntax | Type | Example |
|--------|------|---------|
| `LET X = 5` | Numeric | `LET X = 42` |
| `X = 5` | Numeric (implicit) | `X = 3.14` |
| `LET N$ = "Alice"` | String ($ suffix) | `N$ = "Hello"` |
| `DIM A(10)` | Array declaration | `DIM SCORES(100)` |

### Input

```basic
INPUT "What is your name? ", NAME$
INPUT "Enter a number: ", N
```

### Control Flow

#### IF / THEN / ELSE (single-line)

```basic
IF X > 10 THEN PRINT "big" ELSE PRINT "small"
```

#### IF / THEN / ELSEIF / ELSE / END IF (multi-line block)

```basic
IF X > 100 THEN
    PRINT "Large"
ELSEIF X > 10 THEN
    PRINT "Medium"
ELSE
    PRINT "Small"
END IF
```

#### FOR / NEXT

```basic
FOR I = 1 TO 10
    PRINT I
NEXT I

FOR I = 10 TO 0 STEP -2
    PRINT I
NEXT I
```

#### WHILE / WEND

```basic
WHILE X < 100
    X = X * 2
WEND
```

#### DO / LOOP

```basic
DO WHILE X < 100
    X = X + 1
LOOP

DO
    X = X + 1
LOOP UNTIL X >= 100
```

#### SELECT CASE

```basic
SELECT CASE X
    CASE 1
        PRINT "One"
    CASE 2, 3
        PRINT "Two or Three"
    CASE ELSE
        PRINT "Other"
END SELECT
```

#### GOTO / GOSUB / RETURN

```basic
10 PRINT "Hello"
20 GOTO 10

100 GOSUB 200
110 END
200 PRINT "Subroutine"
210 RETURN
```

### Subroutines and Functions

```basic
SUB GREET(NAME$)
    PRINT "Hello, "; NAME$
END SUB
CALL GREET("World")

FUNCTION DOUBLE(X)
    DOUBLE = X * 2
END FUNCTION
PRINT DOUBLE(21)
```

### String Functions

| Function | Description | Example |
|----------|-------------|---------|
| `LEN(s$)` | String length | `LEN("Hello")` → 5 |
| `LEFT$(s$, n)` | Left n characters | `LEFT$("Hello", 3)` → "Hel" |
| `RIGHT$(s$, n)` | Right n characters | `RIGHT$("Hello", 2)` → "lo" |
| `MID$(s$, start, len)` | Substring | `MID$("Hello", 2, 3)` → "ell" |
| `CHR$(n)` | Character from code | `CHR$(65)` → "A" |
| `ASC(s$)` | Code from character | `ASC("A")` → 65 |
| `STR$(n)` | Number to string | `STR$(42)` → "42" |
| `VAL(s$)` | String to number | `VAL("42")` → 42 |
| `UCASE$(s$)` | Uppercase | `UCASE$("hello")` → "HELLO" |
| `LCASE$(s$)` | Lowercase | `LCASE$("HELLO")` → "hello" |
| `INSTR(s$, find$)` | Find substring | `INSTR("Hello", "ll")` → 3 |

### Comments

```basic
REM This is a comment
' This is also a comment
```

### Other Commands

| Command | Description |
|---------|-------------|
| `END` | Stop program execution |
| `CLS` | Clear the output screen |
| `SLEEP n` / `WAIT n` | Pause for n milliseconds |
| `RANDOMIZE` | Seed random number generator |
| `DIM arr(n)` | Declare array with n elements |
| `WIDTH n` | Set output column width |

### Turtle Graphics (BASIC)

| Command | Description |
|---------|-------------|
| `FORWARD n` | Move forward n pixels |
| `BACKWARD n` | Move backward n pixels |
| `LEFT n` | Turn left n degrees |
| `RIGHT n` | Turn right n degrees |
| `PENUP` | Stop drawing |
| `PENDOWN` | Resume drawing |
| `HOME` | Return to center, heading north |
| `CLEARSCREEN` / `CLS` | Clear canvas |
| `COLOR r, g, b` | Set pen color (0–255 each) |
| `LINE x1, y1, x2, y2` | Draw a line |
| `CIRCLE x, y, r` | Draw a circle |
| `PSET x, y` | Draw a point |
| `DRAW "commands"` | Draw with string commands |

### GPIO (BASIC)

| Command | Description |
|---------|-------------|
| `PINMODE pin, mode` | Set pin mode ("INPUT"/"OUTPUT"/"PWM") |
| `DIGITALWRITE pin, value` | Write HIGH (1) or LOW (0) |
| `DIGITALREAD(pin)` | Read pin state |
| `PWMWRITE pin, duty` | Set PWM duty cycle (0.0–1.0) |
| `ANALOGWRITE pin, value` | Write analog value |
| `SERVOWRITE pin, angle` | Set servo angle (0–180) |
| `GPIORESET` | Reset all pins |

---

## Logo

A full Logo turtle graphics language with procedures, variables, and
recursion.

### Movement

| Command | Abbreviation | Description |
|---------|-------------|-------------|
| `FORWARD n` | `FD n` | Move forward |
| `BACK n` | `BK n` | Move backward |
| `RIGHT n` | `RT n` | Turn right (degrees) |
| `LEFT n` | `LT n` | Turn left (degrees) |
| `HOME` | — | Return to center |
| `CLEARSCREEN` | `CS` | Clear canvas and reset turtle |
| `SETXY x y` | — | Move to absolute position |
| `SETX x` | — | Set X coordinate |
| `SETY y` | — | Set Y coordinate |
| `SETHEADING n` | — | Set absolute heading |

### Pen Control

| Command | Abbreviation | Description |
|---------|-------------|-------------|
| `PENUP` | `PU` | Stop drawing |
| `PENDOWN` | `PD` | Resume drawing |
| `SETPENCOLOR color` | `SETPC` | Set pen color (name, #hex, or CGA 0–15) |
| `SETPENWIDTH n` | — | Set pen width in pixels |
| `SETBGCOLOR color` | — | Set canvas background color |
| `HIDETURTLE` | `HT` | Hide the turtle cursor |
| `SHOWTURTLE` | `ST` | Show the turtle cursor |

### Drawing

| Command | Description | Example |
|---------|-------------|---------|
| `ARC radius angle` | Draw arc | `ARC 50 180` |
| `DOT radius` | Draw filled circle | `DOT 5` |
| `LABEL "text"` | Draw text at turtle position | `LABEL "Hello"` |
| `BEGINFILL` | Start filling a polygon | |
| `ENDFILL` | Complete and fill polygon | |

### Variables

```logo
MAKE "size 100
PRINT :size
MAKE "name "Logo
```

### Control Flow

```logo
REPEAT 4 [FD 100 RT 90]

IF :x > 10 [PRINT "big"]

IFELSE :x > 10 [PRINT "big"] [PRINT "small"]

FOREVER [FD 1 RT 1]
```

### Procedures

```logo
TO SQUARE :size
    REPEAT 4 [FD :size RT 90]
END

SQUARE 100
```

### Output

| Command | Description |
|---------|-------------|
| `PRINT expr` | Print value with newline |
| `SHOW expr` | Print value (same as PRINT) |

### GPIO (Logo)

| Command | Description |
|---------|-------------|
| `PINMODE pin mode` | Set pin mode |
| `DIGITALWRITE pin value` | Write digital value |
| `SETPIN pin value` | Alias for DIGITALWRITE |
| `DIGITALREAD pin` | Read digital pin |
| `READPIN pin` | Alias for DIGITALREAD |
| `PWMWRITE pin duty` | Set PWM duty cycle |
| `GPIORESET` | Reset all pins |

---

## C (Subset)

A teaching subset of C supporting functions, control flow, and standard I/O.

### Program Structure

```c
#include <stdio.h>

int main() {
    printf("Hello, World!\n");
    return 0;
}
```

`#include` and `#define` directives are recognized but not processed (all
standard library functions are built in).

### Variables

```c
int x = 42;
float pi = 3.14;
double e = 2.718;
char c = 'A';
char name[] = "Alice";
```

Multiple declarations: `int a = 1, b = 2, c = 3;`

### Output

```c
printf("Hello, World!\n");
printf("X = %d\n", x);
printf("Pi = %.2f\n", pi);
printf("Name: %s\n", name);
```

Format specifiers: `%d` (int), `%f` (float), `%s` (string), `%c` (char),
`%%` (literal %).

Escape sequences: `\n` (newline), `\t` (tab), `\\` (backslash),
`\"` (quote).

### Input

```c
int age;
scanf("%d", &age);

char name[50];
scanf("%s", name);
```

### Control Flow

```c
// If / else
if (x > 10) {
    printf("big\n");
} else if (x > 5) {
    printf("medium\n");
} else {
    printf("small\n");
}

// While loop
while (x > 0) {
    x--;
}

// For loop
for (int i = 0; i < 10; i++) {
    printf("%d\n", i);
}

// Do-while
do {
    x++;
} while (x < 100);

// Switch
switch (x) {
    case 1: printf("one\n"); break;
    case 2: printf("two\n"); break;
    default: printf("other\n"); break;
}
```

### Functions

```c
int add(int a, int b) {
    return a + b;
}

int factorial(int n) {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}
```

### Operators

| Category | Operators |
|----------|-----------|
| Arithmetic | `+`, `-`, `*`, `/`, `%` |
| Comparison | `==`, `!=`, `<`, `>`, `<=`, `>=` |
| Logical | `&&`, `\|\|`, `!` |
| Assignment | `=`, `+=`, `-=`, `*=`, `/=`, `%=` |
| Increment | `++`, `--` |

### Math Functions

Built-in: `sqrt`, `abs`, `pow`, `sin`, `cos`, `tan`, `log`, `exp`,
`floor`, `ceil`, `round`.

---

## Pascal

A standard Pascal implementation with procedures, functions, and arrays.

### Program Structure

```pascal
program HelloWorld;
begin
    WriteLn('Hello, World!');
end.
```

The `program` header and final `.` are optional.

### Variables

```pascal
var
    x: Integer;
    name: String;
    pi: Real;
    flag: Boolean;

const
    MAX = 100;
```

### Output

| Command | Description | Example |
|---------|-------------|---------|
| `WriteLn(expr)` | Print with newline | `WriteLn('Hello');` |
| `Write(expr)` | Print without newline | `Write('Name: ');` |
| `ClrScr` | Clear output | `ClrScr;` |

### Input

```pascal
var name: String;
begin
    Write('Enter name: ');
    ReadLn(name);
    WriteLn('Hello, ', name);
end.
```

### Control Flow

```pascal
{ If / Then / Else }
if x > 10 then
    WriteLn('big')
else
    WriteLn('small');

{ If with begin/end block }
if x > 10 then
begin
    WriteLn('x is');
    WriteLn('big');
end;

{ For loop (ascending) }
for i := 1 to 10 do
    WriteLn(i);

{ For loop (descending) }
for i := 10 downto 1 do
    WriteLn(i);

{ While loop }
while x < 100 do
    x := x + 1;

{ While with begin/end }
while x < 100 do
begin
    WriteLn(x);
    x := x + 1;
end;

{ Repeat / Until }
repeat
    x := x + 1;
until x >= 100;
```

### Procedures and Functions

```pascal
procedure Greet(name: String);
begin
    WriteLn('Hello, ', name, '!');
end;

function Double(x: Integer): Integer;
begin
    Double := x * 2;
end;

begin
    Greet('World');
    WriteLn(Double(21));
end.
```

### Arrays

```pascal
var arr: array[1..5] of Integer;
begin
    arr[1] := 10;
    arr[2] := 20;
    WriteLn(arr[1] + arr[2]);
end.
```

### Comments

```pascal
{ This is a comment }
(* This is also a comment *)
// Single-line comment
```

---

## Forth

A stack-based language with word definitions and interactive execution.

### Stack Operations

| Word | Stack effect | Description |
|------|-------------|-------------|
| `DUP` | ( a -- a a ) | Duplicate top |
| `DROP` | ( a -- ) | Remove top |
| `SWAP` | ( a b -- b a ) | Swap top two |
| `OVER` | ( a b -- a b a ) | Copy second to top |
| `ROT` | ( a b c -- b c a ) | Rotate third to top |
| `?DUP` | ( a -- a a ) or ( 0 -- 0 ) | Dup if non-zero |

### Arithmetic

| Word | Stack effect | Description |
|------|-------------|-------------|
| `+` | ( a b -- a+b ) | Add |
| `-` | ( a b -- a-b ) | Subtract |
| `*` | ( a b -- a*b ) | Multiply |
| `/` | ( a b -- a/b ) | Divide |
| `MOD` | ( a b -- a%b ) | Modulo |
| `/MOD` | ( a b -- rem quot ) | Divide with remainder |
| `NEGATE` | ( a -- -a ) | Negate |
| `ABS` | ( a -- |a| ) | Absolute value |
| `MIN` | ( a b -- min ) | Minimum |
| `MAX` | ( a b -- max ) | Maximum |
| `1+` | ( a -- a+1 ) | Increment |
| `1-` | ( a -- a-1 ) | Decrement |
| `2*` | ( a -- a*2 ) | Double |
| `2/` | ( a -- a/2 ) | Halve |

### Comparison

| Word | Stack effect | Description |
|------|-------------|-------------|
| `=` | ( a b -- flag ) | Equal |
| `<>` | ( a b -- flag ) | Not equal |
| `<` | ( a b -- flag ) | Less than |
| `>` | ( a b -- flag ) | Greater than |
| `0=` | ( a -- flag ) | Equal to zero |
| `0<` | ( a -- flag ) | Negative |
| `0>` | ( a -- flag ) | Positive |

### Logic

| Word | Stack effect | Description |
|------|-------------|-------------|
| `AND` | ( a b -- a&b ) | Bitwise AND |
| `OR` | ( a b -- a\|b ) | Bitwise OR |
| `XOR` | ( a b -- a^b ) | Bitwise XOR |
| `INVERT` | ( a -- ~a ) | Bitwise NOT |

### I/O

| Word | Description | Example |
|------|-------------|---------|
| `.` | Print top of stack | `42 .` → "42 " |
| `.S` | Print entire stack | `.S` → "<3> 1 2 3 " |
| `EMIT` | Print character by code | `65 EMIT` → "A" |
| `CR` | Print newline | `CR` |
| `SPACE` | Print one space | `SPACE` |
| `SPACES` | Print n spaces | `5 SPACES` |
| `." text"` | Print literal string | `." Hello!"` |

### Control Flow

```forth
\ If / Else / Then
: CHECK  10 > IF ." big" ELSE ." small" THEN ;

\ Begin / Until (post-test loop)
: COUNTDOWN  10 BEGIN DUP . 1- DUP 0= UNTIL DROP ;

\ Begin / While / Repeat (pre-test loop)
: COUNT-UP  0 BEGIN DUP 10 < WHILE DUP . 1+ REPEAT DROP ;

\ Begin / Again (infinite loop)
: FOREVER-LOOP  BEGIN ." tick " AGAIN ;

\ Do / Loop (counted loop)
: ONES  10 0 DO I . LOOP ;

\ Do / +Loop (counted loop with step)
: EVENS  10 0 DO I . 2 +LOOP ;

\ Leave (exit loop early)
: FIND-5  10 0 DO I 5 = IF ." found" LEAVE THEN LOOP ;
```

### Word Definitions

```forth
: SQUARE  ( n -- n^2 )  DUP * ;
5 SQUARE .    \ prints 25

: FACTORIAL  ( n -- n! )
    DUP 1 <= IF DROP 1 EXIT THEN
    DUP 1- RECURSE * ;
```

### Variables and Constants

```forth
VARIABLE counter
10 counter !     \ store 10
counter @        \ fetch value
counter @ .      \ print: 10

42 CONSTANT answer
answer .         \ print: 42
```

### Memory

| Word | Stack effect | Description |
|------|-------------|-------------|
| `!` | ( value addr -- ) | Store value |
| `@` | ( addr -- value ) | Fetch value |
| `+!` | ( delta addr -- ) | Add to variable |

### Turtle Graphics (Forth)

| Word | Description |
|------|-------------|
| `FD` / `FORWARD` | Move forward |
| `BK` / `BACK` | Move backward |
| `RT` / `RIGHT` | Turn right |
| `LT` / `LEFT` | Turn left |
| `PU` / `PENUP` | Pen up |
| `PD` / `PENDOWN` | Pen down |
| `COLOR` | Set pen color (0–15 CGA) |

### Comments

```forth
\ This is a line comment
( This is a stack-effect comment )
```

### Hex Literals

```forth
$FF .    \ prints 255
$1A .    \ prints 26
```

---

## PILOT

**Programmed Instruction, Learning, Or Teaching** — an educational scripting
language for creating interactive lessons and quizzes.

### Core Commands

Each line starts with a single-letter command followed by a colon:

| Command | Full Name | Description | Example |
|---------|-----------|-------------|---------|
| `T:` | Type | Display text | `T:Hello, World!` |
| `A:` | Accept | Read user input | `A:` |
| `M:` | Match | Check if input matches | `M:yes,yeah,yep` |
| `Y:` | Yes | Execute if last match succeeded | `Y:T:Correct!` |
| `N:` | No | Execute if last match failed | `N:T:Wrong!` |
| `C:` | Compute | Evaluate expression | `C:X = 5 + 3` |
| `J:` | Jump | Jump to label | `J:*START` |
| `U:` | Use | Call subroutine at label | `U:*GREET` |
| `R:` | Remark | Comment | `R:This is a comment` |
| `E:` | End | End program or return from subroutine | `E:` |
| `D:` | Display | Alias for Type | `D:Same as T:` |

### Verbose Syntax

Commands can also be spelled out in full:

```pilot
TYPE: Hello, World!
ACCEPT:
MATCH: yes
JUMP: *LOOP
COMPUTE: X = 10
```

### Labels

Labels are marked with `*` and used with `J:` (jump) and `U:` (use):

```pilot
*START
T:Welcome!
J:*START
```

### Conditional Execution

`Y:` and `N:` run only when the last `M:` succeeded or failed:

```pilot
T:Do you like programming?
A:
M:yes,yeah,yep
Y:T:Great! Me too!
N:T:You should try it!
```

### Variables and Interpolation

Variables are set with `C:` and interpolated with `#`:

```pilot
C:NAME$ = "World"
T:Hello, #NAME$!
C:X = 5 + 3
T:Result is #X
```

### Subroutines

```pilot
U:*GREET
T:Back from subroutine
E:

*GREET
T:Hello from subroutine!
E:
```

### Arithmetic

The `C:` command supports expressions: `+`, `-`, `*`, `/`, `%`,
parentheses, and all math functions from the expression evaluator.

---

## Prolog

A logic programming language for defining facts, rules, and queries.

### Facts

```prolog
parent(tom, bob).
parent(tom, liz).
parent(bob, ann).
```

### Rules

```prolog
grandparent(X, Z) :- parent(X, Y), parent(Y, Z).
ancestor(X, Y) :- parent(X, Y).
ancestor(X, Y) :- parent(X, Z), ancestor(Z, Y).
```

### Queries

```prolog
?- parent(tom, bob).
?- grandparent(tom, ann).
?- parent(tom, X).
```

### Variables

- **Uppercase** names are variables: `X`, `Name`, `Result`
- **Lowercase** names are atoms (constants): `tom`, `hello`
- **`_`** is the anonymous variable (matches anything, no binding)

### Built-in Predicates

| Predicate | Description | Example |
|-----------|-------------|---------|
| `write(X)` | Print a value | `?- write(hello).` |
| `writeln(X)` | Print with newline | `?- writeln(42).` |
| `nl` | Print newline | `?- nl.` |
| `is` | Arithmetic evaluation | `?- X is 2 + 3.` |
| `not(Goal)` | Negation as failure | `?- not(parent(tom, ann)).` |
| `true` | Always succeeds | `?- true.` |
| `fail` | Always fails | `?- fail.` |
| `atom(X)` | Test if atom | `?- atom(hello).` |
| `number(X)` | Test if number | `?- number(42).` |
| `var(X)` | Test if unbound variable | `?- var(X).` |
| `nonvar(X)` | Test if bound | `?- nonvar(hello).` |
| `findall(T, G, L)` | Collect all solutions | `?- findall(X, parent(tom,X), L).` |
| `member(X, L)` | List membership | `?- member(2, [1,2,3]).` |
| `append(A, B, C)` | List concatenation | `?- append([1],[2],X).` |
| `length(L, N)` | List length | `?- length([a,b,c], N).` |

### Lists

```prolog
?- member(X, [1, 2, 3]).
?- append([a, b], [c, d], Result).
?- length([1, 2, 3], N).
```

### Arithmetic

```prolog
?- X is 2 + 3 * 4.        % X = 14
?- X is sqrt(16).          % X = 4.0
?- X is 10 mod 3.          % X = 1
```

### Comments

```prolog
% This is a line comment
/* This is a
   block comment */
```

### Important Notes

- Every clause must end with a **period** (`.`)
- Queries start with **`?-`**
- Variables are scoped to a single clause
- Maximum recursion depth is enforced to prevent infinite loops

---

## Expression Evaluator

All languages share a common expression evaluator that supports:

### Arithmetic Operators

| Operator | Description | Precedence |
|----------|-------------|------------|
| `+` | Addition | Low |
| `-` | Subtraction | Low |
| `*` | Multiplication | Medium |
| `/` | Division | Medium |
| `%` / `MOD` | Modulo | Medium |
| `^` | Power | High |
| `-` (unary) | Negation | Highest |

### Comparison Operators

| Operator | Description |
|----------|-------------|
| `=` / `==` | Equal |
| `<>` / `!=` | Not equal |
| `<` | Less than |
| `>` | Greater than |
| `<=` | Less or equal |
| `>=` | Greater or equal |

### Logical Operators

| Operator | Description |
|----------|-------------|
| `AND` / `&&` | Logical AND |
| `OR` / `\|\|` | Logical OR |
| `NOT` / `!` | Logical NOT |

### Math Functions

| Function | Description | Example |
|----------|-------------|---------|
| `ABS(x)` | Absolute value | `ABS(-5)` → 5 |
| `SQRT(x)` | Square root | `SQRT(16)` → 4 |
| `SIN(x)` | Sine (radians) | `SIN(3.14159)` → ~0 |
| `COS(x)` | Cosine (radians) | `COS(0)` → 1 |
| `TAN(x)` | Tangent (radians) | `TAN(0.785)` → ~1 |
| `ASIN(x)` | Arc sine | `ASIN(1)` → 1.5708 |
| `ACOS(x)` | Arc cosine | `ACOS(0)` → 1.5708 |
| `ATAN(x)` | Arc tangent | `ATAN(1)` → 0.7854 |
| `EXP(x)` | e^x | `EXP(1)` → 2.718 |
| `LOG(x)` | Natural log (ln) | `LOG(2.718)` → ~1 |
| `LOG10(x)` | Base-10 log | `LOG10(100)` → 2 |
| `POW(x, y)` | Power | `POW(2, 10)` → 1024 |
| `FLOOR(x)` | Round down | `FLOOR(3.7)` → 3 |
| `CEIL(x)` | Round up | `CEIL(3.2)` → 4 |
| `ROUND(x)` | Round nearest | `ROUND(3.5)` → 4 |
| `SGN(x)` | Sign (-1, 0, 1) | `SGN(-5)` → -1 |
| `MIN(a, b)` | Minimum | `MIN(3, 7)` → 3 |
| `MAX(a, b)` | Maximum | `MAX(3, 7)` → 7 |
| `INT(x)` | Truncate to integer | `INT(3.9)` → 3 |
| `RND` / `RANDOM` | Random 0.0–1.0 | `RND` → 0.4217… |
| `RAND(n)` | Random 0 to n-1 | `RAND(6)` → 0–5 |

### Constants

| Constant | Value |
|----------|-------|
| `PI` | 3.14159265358979… |
| `E` | 2.71828182845905… |
