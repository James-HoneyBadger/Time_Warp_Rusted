# Tutorial 06 — Forth: Stack-Based Thinking

## Introduction

Forth was created by Charles Moore in 1970. Unlike most languages, Forth uses
a **stack** — like a stack of plates. You push numbers onto the stack, and
operations pop numbers off, compute, and push results back. It's a completely
different way of thinking about programming!

**What you'll learn:**
- The stack and Reverse Polish Notation
- Stack manipulation (DUP, SWAP, DROP, etc.)
- Arithmetic
- Defining new words (functions)
- Control flow (IF, DO LOOP, BEGIN UNTIL)
- Variables and constants
- String output
- Turtle graphics

---

## Lesson 1: The Stack

In Forth, you type numbers and they go onto a stack. Operations work on
what's on top of the stack.

```forth
5          \ Push 5 onto the stack
3          \ Push 3 onto the stack
+          \ Pop 5 and 3, push 8
.          \ Pop and print: 8
```

This is called **Reverse Polish Notation** (RPN): the operator comes AFTER
the operands. Instead of `5 + 3`, you write `5 3 +`.

```forth
10 3 + .           \ Prints 13
100 37 - .         \ Prints 63
6 7 * .            \ Prints 42
20 4 / .           \ Prints 5
```

**Think of the stack like a pile of plates:** you can only add to or remove from the top.

**Challenge:** Calculate `(3 + 4) * (10 - 2)` using the stack.

---

## Lesson 2: Stack Operations

These commands manipulate what's on the stack:

| Command | Before → After | What It Does |
|---------|---------------|-------------|
| `DUP` | `a → a a` | Duplicate the top |
| `DROP` | `a →` | Discard the top |
| `SWAP` | `a b → b a` | Swap top two |
| `OVER` | `a b → a b a` | Copy second to top |
| `ROT` | `a b c → b c a` | Rotate third to top |
| `2DUP` | `a b → a b a b` | Duplicate top pair |
| `2DROP` | `a b →` | Discard top pair |

```forth
5 DUP * .          \ 5 × 5 = 25 (squaring!)
10 20 SWAP . .     \ Prints 10 20 (swapped)
3 4 OVER . . .     \ Prints 3 4 3

\ See what's on the stack with .S
1 2 3 .S           \ Shows current stack
```

**Challenge:** Given `3 5` on the stack, use stack operations to compute `3² + 5²`.

---

## Lesson 3: Arithmetic

```forth
\ Basic arithmetic
100 37 + .         \ 137
100 37 - .         \ 63
12 9 * .           \ 108
144 12 / .         \ 12
17 5 MOD .         \ 2  (remainder)

\ Special operations
-42 ABS .          \ 42  (absolute value)
7 NEGATE .         \ -7  (negate)
3 8 MIN .          \ 3   (minimum)
3 8 MAX .          \ 8   (maximum)
10 1+ .            \ 11  (add 1)
20 2* .            \ 40  (multiply by 2)
```

**Challenge:** Calculate the average of 10, 20, 30, 40, and 50 using the stack.

---

## Lesson 4: Defining Words

In Forth, a function is called a **word**. Define new words with `:` and `;`:

```forth
: SQUARE   DUP * ;
: CUBE     DUP DUP * * ;
: DOUBLE   2 * ;

5 SQUARE .         \ 25
3 CUBE .           \ 27
7 DOUBLE .         \ 14
```

### Words with Multiple Inputs

```forth
: SUM-OF-SQUARES   \ ( a b -- result )
  SWAP DUP *       \ square first
  SWAP DUP *       \ square second
  + ;              \ add

3 4 SUM-OF-SQUARES .   \ 25 (9 + 16)
```

### Recursive Words

```forth
: FACTORIAL
  DUP 1 <= IF DROP 1
  ELSE DUP 1 - FACTORIAL *
  THEN ;

5 FACTORIAL .      \ 120
10 FACTORIAL .     \ 3628800
```

**Challenge:** Define a word `F-TO-C` that converts Fahrenheit to Celsius: `(F - 32) * 5 / 9`.

---

## Lesson 5: Control Flow

### IF/ELSE/THEN

**Heads up:** In Forth, `THEN` means "end of IF" — it comes AFTER the if/else blocks!

```forth
: CHECK-SIGN       \ ( n -- )
  DUP 0 > IF
    ." positive" CR
  ELSE
    DUP 0 < IF
      ." negative" CR
    ELSE
      ." zero" CR
    THEN
  THEN
  DROP ;

42 CHECK-SIGN      \ positive
-5 CHECK-SIGN      \ negative
0 CHECK-SIGN       \ zero
```

### DO LOOP

```forth
\ Count from 0 to 9
10 0 DO
  I .              \ I is the loop counter
LOOP CR

\ Count from 1 to 10
11 1 DO I . LOOP CR
```

### DO +LOOP (with step)

```forth
\ Count by 2s: 0, 2, 4, 6, 8
10 0 DO I . 2 +LOOP CR

\ Count by 5s: 0, 5, 10, 15, 20
25 0 DO I . 5 +LOOP CR
```

### BEGIN UNTIL

```forth
\ Count up until we reach 10
1
BEGIN
  DUP .
  1+
  DUP 10 >
UNTIL
DROP CR
```

### Nested Loops

```forth
\ Multiplication table (3×3)
4 1 DO
  4 1 DO
    I J * 4 .R     \ .R prints right-aligned
  LOOP CR
LOOP
```

**Challenge:** Print all even numbers from 2 to 20 using a DO +LOOP.

---

## Lesson 6: Variables and Constants

### Variables

```forth
VARIABLE COUNTER
VARIABLE TOTAL

0 COUNTER !        \ Store 0 in COUNTER
0 TOTAL !          \ Store 0 in TOTAL

COUNTER @ .        \ Fetch and print: 0

5 COUNTER !        \ Store 5
COUNTER @ .        \ Prints 5

10 COUNTER +!      \ Add 10 to COUNTER
COUNTER @ .        \ Prints 15
```

| Operation | What It Does |
|-----------|-------------|
| `!` (store) | Store value in variable |
| `@` (fetch) | Read value from variable |
| `+!` (plus-store) | Add to variable's value |

### Constants

```forth
CONSTANT DOZEN 12
CONSTANT CENTURY 100

DOZEN .            \ Prints 12
CENTURY .          \ Prints 100
```

**Challenge:** Create a COUNTER variable and increment it in a loop, printing each value.

---

## Lesson 7: String Output

```forth
." Hello, World!" CR     \ Print a string

\ EMIT prints a single ASCII character
65 EMIT CR              \ Prints A
72 EMIT 73 EMIT CR      \ Prints HI

\ SPACE prints a space
42 . SPACE 43 . CR

\ SPACES prints multiple spaces
." [" 10 SPACES ." ]" CR
```

### Box Drawing

```forth
: HLINE   20 0 DO 45 EMIT LOOP ;    \ 45 = '-'

." +" HLINE ." +" CR
." |                    |" CR
." +" HLINE ." +" CR
```

**Challenge:** Create a `BOX` word that draws a bordered box with a message inside.

---

## Lesson 8: Turtle Graphics

Forth in Time Warp supports turtle graphics:

```forth
\ Draw a square
1 PEN              \ Pen colour 1 (usually white)
4 0 DO
  100 FD           \ Forward 100 pixels
  90 RT            \ Turn right 90 degrees
LOOP

\ Draw a star
5 0 DO
  80 FD
  144 RT
LOOP

\ Draw a spiral
36 0 DO
  I 2 * FD
  20 RT
LOOP
```

| Command | What It Does |
|---------|-------------|
| `FD n` | Move forward n pixels |
| `BK n` | Move backward |
| `RT n` | Turn right n degrees |
| `LT n` | Turn left n degrees |
| `PU` | Pen up (stop drawing) |
| `PD` | Pen down (start drawing) |
| `n PEN` | Set pen colour |

**Challenge:** Draw a hexagon (6 sides, 60-degree turns) and a triangle inside it.

---

## Lesson 9: Algorithms

### Fibonacci

```forth
: FIB-SERIES
  0 1
  10 0 DO
    OVER .
    OVER + SWAP
  LOOP
  2DROP CR ;

FIB-SERIES
```

### GCD (Euclid's Algorithm)

```forth
: GCD
  BEGIN
    DUP
  WHILE
    SWAP OVER MOD
  REPEAT
  DROP ;

48 36 GCD .        \ 12
100 75 GCD .       \ 25
```

### Prime Check

```forth
: PRIME?
  DUP 2 < IF DROP 0
  ELSE
    1
    OVER 2 / 1 + 2 DO
      OVER I MOD 0 = IF DROP 0 LEAVE THEN
    LOOP
    SWAP DROP
  THEN ;

7 PRIME? .         \ 1 (true)
10 PRIME? .        \ 0 (false)
```

**Challenge:** Use the PRIME? word to find and print all primes from 2 to 50.

---

## Projects

### Project 1: Stack Calculator
Define words for common calculations: area of circle, Fahrenheit-to-Celsius,
compound interest, etc.

### Project 2: Turtle Art
Draw a mandala pattern using nested DO LOOPs and turtle commands.

### Project 3: Number Classifier
Define a word that takes a number and prints `even`/`odd`, `prime`/`composite`,
`positive`/`negative`.

### Project 4: Statistics
Store 10 numbers in variables, compute sum, average, min, and max.

---

## Quick Reference

| Feature | Syntax |
|---------|--------|
| Push number | `42` |
| Print top | `.` |
| Print string | `." text"` |
| Newline | `CR` |
| Define word | `: NAME ... ;` |
| Duplicate | `DUP` |
| Swap | `SWAP` |
| Drop | `DROP` |
| Over | `OVER` |
| Rotate | `ROT` |
| Arithmetic | `+ - * / MOD` |
| Absolute | `ABS` |
| Negate | `NEGATE` |
| If/Else | `cond IF ... ELSE ... THEN` |
| Do Loop | `limit start DO ... LOOP` |
| Step Loop | `limit start DO ... n +LOOP` |
| Begin Until | `BEGIN ... cond UNTIL` |
| Variable | `VARIABLE name` |
| Store | `value var !` |
| Fetch | `var @` |
| Constant | `CONSTANT name value` |
| Turtle | `FD BK RT LT PU PD PEN` |
