# Language Guide

All seven languages share the same turtle graphics primitives and can use the same canvas.  They are interpreted line-by-line; no compilation step is needed.

---

## BASIC

Classic line-numbered (or free-format) BASIC.

```basic
10 PRINT "Hello, World!"
20 FOR I = 1 TO 5
30   PRINT "Number: "; I
40 NEXT I
50 END
```

### Key Commands

| Command | Description |
|---------|-------------|
| `PRINT expr [; expr …]` | Print values; `;` suppresses newline between items |
| `INPUT var` | Read user input into variable |
| `LET var = expr` | Assignment (LET is optional) |
| `IF cond THEN … [ELSE …]` | Conditional |
| `GOTO n` | Jump to line number |
| `GOSUB n` / `RETURN` | Call / return subroutine |
| `FOR x = a TO b [STEP s]` … `NEXT x` | Counted loop |
| `WHILE cond` … `WEND` | While loop |
| `DO [WHILE/UNTIL cond]` … `LOOP [WHILE/UNTIL cond]` | Do loop |
| `SELECT CASE x` … `CASE value` … `END SELECT` | Switch |
| `SUB name(params)` … `END SUB` | Subroutine definition |
| `DIM arr(n)` | Declare array |
| `REM text` or `'text` | Comment |

### Turtle Commands (BASIC)

`FORWARD n`, `FD n`, `BACKWARD n`, `LEFT deg`, `RIGHT deg`, `PENUP`, `PENDOWN`, `HOME`, `CLEARSCREEN`, `CIRCLE r`, `LINE x1,y1,x2,y2`, `PSET x,y`, `COLOR r,g,b`

---

## PILOT

PILOT (Programmed Inquiry, Learning Or Teaching) is a CAI language built around text interaction.

```pilot
T:What is your name?
A:NAME
T:Hello, #NAME!
M:yes,yeah,yep
TY:Great!
TN:Oh well.
E:
```

### Commands

| Command | Description |
|---------|-------------|
| `T:text` | Type (output) text; `#VAR` is interpolated |
| `A:var` | Accept (read) input into variable |
| `M:pat1,pat2` | Match last input against comma-separated patterns; sets match flag |
| `J:*LABEL` | Jump to label unconditionally |
| `Y:*LABEL` | Jump if last match succeeded |
| `N:*LABEL` | Jump if last match failed |
| `C:var = expr` | Compute — evaluate expression into variable |
| `R:text` | Remark (comment) |
| `E:` | End |

Add `Y` or `N` after the command letter as a conditional: `TY:` outputs only if last match was true.

---

## Logo

Logo is a turtle-geometry language.

```logo
TO SQUARE :SIZE
  REPEAT 4 [FORWARD :SIZE RIGHT 90]
END

SQUARE 100
```

### Key Commands

| Command | Description |
|---------|-------------|
| `FORWARD n` / `FD n` | Move forward |
| `BACK n` / `BK n` | Move backward |
| `RIGHT deg` / `RT deg` | Turn right |
| `LEFT deg` / `LT deg` | Turn left |
| `PENUP` / `PU` | Lift pen |
| `PENDOWN` / `PD` | Lower pen |
| `HOME` | Return to centre |
| `CLEARSCREEN` / `CS` | Clear canvas |
| `REPEAT n [block]` | Repeat block n times; `REPCOUNT` variable set |
| `FOREVER [block]` | Loop forever |
| `IF cond [block]` | Conditional |
| `IFELSE cond [then] [else]` | If-else |
| `MAKE "var value` | Set variable |
| `:var` | Read variable |
| `TO name :p1 :p2` … `END` | Define procedure |
| `SETXY x y` / `SETX x` / `SETY y` | Set position |
| `SETHEADING deg` / `SETH deg` | Set heading |
| `ARC radius angle` | Draw arc |
| `DOT radius` | Draw dot |
| `LABEL "text"` | Draw text at turtle position |

---

## C

A subset interpreter for C, suitable for teaching fundamentals.

```c
#include <stdio.h>
int main() {
    int i;
    for (i = 1; i <= 10; i++) {
        printf("%d\n", i);
    }
    return 0;
}
```

Supported: `printf`, `scanf`, variable declarations (`int`, `float`, `double`, `char`), assignment, `+=` / `-=` / `*=` / `/=`, `++` / `--`, `if/else`, `while`, `for`, function calls, basic math library (`sqrt`, `abs`, `pow`, `sin`, `cos`, `tan`, `floor`, `ceil`).

---

## Pascal

```pascal
program Hello;
var
  i : integer;
begin
  writeln('Hello, World!');
  for i := 1 to 5 do
    writeln(i);
end.
```

Supported: `program`, `var`, `const`, `:=`, `writeln`, `write`, `readln`, `if/then/else`, `while/do`, `for/to/downto/do`, `repeat/until`, `procedure`, `function`, `begin/end`, arrays.

---

## Prolog

Facts, rules, and queries with unification and backtracking.

```prolog
parent(tom, bob).
parent(bob, ann).
ancestor(X, Y) :- parent(X, Y).
ancestor(X, Y) :- parent(X, Z), ancestor(Z, Y).

?- ancestor(tom, ann).
```

- Clauses end with `.`
- Rules use `:-`
- Queries start with `?-`
- Variables start with an uppercase letter or `_`
- `_` is the anonymous variable

---

## Forth

A stack-based, concatenative language.

```forth
: SQUARE DUP * ;
: CUBE DUP SQUARE * ;
5 SQUARE .   → 25
3 CUBE .     → 27
```

### Stack Operations

`DUP DROP SWAP OVER ROT NIP TUCK 2DUP 2DROP 2SWAP`

### Arithmetic

`+ - * / MOD /MOD NEGATE ABS MAX MIN 1+ 1- 2* 2/`

### Comparison / Logic

`= <> < > <= >= 0= 0< 0> AND OR XOR INVERT`

### I/O

`. .S CR EMIT SPACE SPACES`

### Memory

`@ ! C@ C! +! VARIABLE CONSTANT`

### Control Flow

`IF ELSE THEN BEGIN AGAIN UNTIL WHILE REPEAT DO LOOP +LOOP I J LEAVE`

### Turtle Extensions

`FD BK RT LT PU PD HOME CS PEN SETX SETY`

### Word Definition

`: WORD … ;` defines a new word.
