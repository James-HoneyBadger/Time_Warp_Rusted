# Tutorial 02 — BASIC for Beginners

## Introduction

BASIC (Beginner's All-purpose Symbolic Instruction Code) was created in 1964
to make programming accessible to everyone. It's one of the most popular
languages ever created and a perfect starting point.

**What you'll learn:**
- Printing text and numbers
- Variables and arithmetic
- Getting user input
- Making decisions (IF/THEN)
- Repeating things (loops)
- Turtle and pixel graphics
- String manipulation
- Arrays and subroutines

---

## Lesson 1: Printing Output

The `PRINT` command displays text and numbers.

```basic
REM This is a comment — the computer ignores it
PRINT "Hello!"
PRINT "My name is BASIC."
PRINT 42
PRINT 3 + 4
```

**Try it:** Change the text inside the quotes. What happens if you remove the quotes around a number?

### Printing on the Same Line

Use a semicolon `;` to keep printing on the same line:

```basic
PRINT "Hello ";
PRINT "World!"
```

**Challenge:** Print your full name using two PRINT statements on one line.

---

## Lesson 2: Variables

Variables store values that can change. Think of them as labelled boxes.

```basic
LET name$ = "Alice"
LET age = 12
LET height = 1.52

PRINT "Name: "; name$
PRINT "Age: "; age
PRINT "Height: "; height
```

**Rules:**
- String variables end with `$` (like `name$`)
- Number variables have no `$` (like `age`)
- `LET` is optional: `age = 12` works too

**Challenge:** Create variables for your favourite colour, lucky number, and pet's name. Print them all.

---

## Lesson 3: Arithmetic

BASIC can do maths with `+`, `-`, `*`, `/`.

```basic
PRINT "Addition:       "; 10 + 3
PRINT "Subtraction:    "; 10 - 3
PRINT "Multiplication: "; 10 * 3
PRINT "Division:       "; 10 / 3

REM Using variables
length = 5
width = 3
area = length * width
perimeter = 2 * (length + width)
PRINT "Area: "; area
PRINT "Perimeter: "; perimeter
```

### Math Functions

```basic
PRINT "Square root of 144: "; SQR(144)
PRINT "Absolute value of -7: "; ABS(-7)
PRINT "Pi: "; PI
PRINT "Sin(1): "; SIN(1)
PRINT "Random number: "; RND
```

**Challenge:** Calculate the area of a circle with radius 10 using `PI * r * r`.

---

## Lesson 4: Getting Input

`INPUT` asks the user to type something.

```basic
INPUT "What is your name? ", name$
INPUT "How old are you? ", age
PRINT "Hello, "; name$; "! You are "; age; " years old."
PRINT "In 10 years you will be "; age + 10; "!"
```

**Challenge:** Write a program that asks for two numbers and prints their sum, difference, product, and quotient.

---

## Lesson 5: Making Decisions

`IF...THEN` lets your program choose what to do.

### Simple IF
```basic
INPUT "Enter a number: ", n
IF n > 0 THEN PRINT "Positive!"
IF n < 0 THEN PRINT "Negative!"
IF n = 0 THEN PRINT "Zero!"
```

### Multi-line IF/ELSE
```basic
INPUT "Enter your score: ", score
IF score >= 90 THEN
  PRINT "Grade: A"
ELSEIF score >= 80 THEN
  PRINT "Grade: B"
ELSEIF score >= 70 THEN
  PRINT "Grade: C"
ELSE
  PRINT "Grade: F"
END IF
```

### SELECT CASE
```basic
INPUT "Pick a number 1-3: ", choice
SELECT CASE choice
  CASE 1
    PRINT "You picked one!"
  CASE 2
    PRINT "You picked two!"
  CASE 3
    PRINT "You picked three!"
END SELECT
```

**Challenge:** Write a program that asks for a month number (1-12) and prints the season.

---

## Lesson 6: Loops

Loops repeat code multiple times.

### FOR/NEXT Loop
```basic
FOR i = 1 TO 10
  PRINT i; " squared = "; i * i
NEXT i
```

### FOR with STEP
```basic
REM Count by 5s
FOR i = 0 TO 50 STEP 5
  PRINT i
NEXT i

REM Count backwards
FOR i = 10 TO 1 STEP -1
  PRINT i; "..."
NEXT i
PRINT "Blast off!"
```

### WHILE/WEND Loop
```basic
n = 1
WHILE n <= 1000
  PRINT n
  n = n * 2
WEND
```

### DO/LOOP UNTIL
```basic
total = 0
DO
  INPUT "Enter a number (0 to stop): ", n
  total = total + n
LOOP UNTIL n = 0
PRINT "Total: "; total
```

**Challenge:** Write a loop that prints the first 20 Fibonacci numbers.

---

## Lesson 7: Subroutines and Functions

### GOSUB/RETURN
```basic
PRINT "Before subroutine"
GOSUB DrawLine
PRINT "Between calls"
GOSUB DrawLine
PRINT "After subroutine"
END

DrawLine:
  PRINT "========================"
RETURN
```

### SUB and FUNCTION
```basic
SUB Greet(name$)
  PRINT "Hello, "; name$; "!"
END SUB

FUNCTION Square(n)
  Square = n * n
END FUNCTION

CALL Greet("Alice")
CALL Greet("Bob")
PRINT "5 squared = "; Square(5)
PRINT "9 squared = "; Square(9)
```

**Challenge:** Write a FUNCTION that calculates the factorial of a number.

---

## Lesson 8: String Functions

BASIC has powerful string handling:

```basic
word$ = "Time Warp Rusted"
PRINT "String:    "; word$
PRINT "Length:    "; LEN(word$)
PRINT "Left 4:   "; LEFT$(word$, 4)
PRINT "Right 6:  "; RIGHT$(word$, 6)
PRINT "Mid 6,4:  "; MID$(word$, 6, 4)
PRINT "CHR$(65): "; CHR$(65)
PRINT "STR$(42): "; STR$(42)
```

**Challenge:** Ask the user for their full name, then print just their first initial and last name.

---

## Lesson 9: Arrays

Arrays store multiple values under one name.

```basic
DIM scores(5)
scores(1) = 85
scores(2) = 92
scores(3) = 78
scores(4) = 95
scores(5) = 88

total = 0
FOR i = 1 TO 5
  total = total + scores(i)
NEXT i
PRINT "Average: "; total / 5
```

**Challenge:** Fill an array with 10 random numbers, sort them, and print the sorted list.

---

## Lesson 10: Turtle Graphics

Move the turtle to draw pictures!

```basic
REM Draw a colourful square
COLOR "red"
WIDTH 3
FOR i = 1 TO 4
  FORWARD 100
  RIGHT 90
NEXT i

REM Draw a star
PENUP
FORWARD 120
PENDOWN
COLOR "gold"
FOR i = 1 TO 5
  FORWARD 80
  RIGHT 144
NEXT i
```

### Turtle Commands
| Command | What It Does |
|---------|-------------|
| `FORWARD n` | Move forward n pixels |
| `RIGHT n` | Turn right n degrees |
| `LEFT n` | Turn left n degrees |
| `PENUP` | Stop drawing |
| `PENDOWN` | Start drawing |
| `COLOR "name"` | Set drawing colour |
| `WIDTH n` | Set line thickness |
| `HOME` | Return to centre |

**Challenge:** Draw a house with a door and two windows using turtle graphics.

---

## Lesson 11: Pixel Graphics (GW-BASIC Style)

For pixel-level drawing:

```basic
CLS
LINE (10, 10)-(200, 10), "red"
LINE (50, 50)-(150, 150), "blue", B
LINE (60, 60)-(140, 140), "yellow", BF
CIRCLE (200, 100), 40, "green"
PSET (100, 100), "white"
```

| Command | What It Does |
|---------|-------------|
| `LINE (x1,y1)-(x2,y2), colour` | Draw a line |
| `LINE ..., B` | Draw a box outline |
| `LINE ..., BF` | Draw a filled box |
| `CIRCLE (x,y), r, colour` | Draw a circle |
| `PSET (x,y), colour` | Draw a point |

**Challenge:** Draw a simple scene (sky, ground, sun, house) using pixel graphics.

---

## Projects

### Project 1: Number Guessing Game
Write a program that picks a random number 1–100 and lets the user guess,
giving "higher" or "lower" hints.

### Project 2: Times Table Tester
Quiz the user on multiplication facts, keep score, and give a grade at the end.

### Project 3: ASCII Art Generator
Ask for a size and draw a diamond pattern made of stars.

### Project 4: Turtle Art Gallery
Draw a series of shapes: triangle, square, pentagon, hexagon, octagon,
each in a different colour, arranged in a row.

---

## Quick Reference

| Command | Example |
|---------|---------|
| `PRINT` | `PRINT "hello"` |
| `INPUT` | `INPUT "name? ", n$` |
| `LET` | `LET x = 5` |
| `IF...THEN` | `IF x > 0 THEN PRINT "yes"` |
| `FOR...NEXT` | `FOR i = 1 TO 10` |
| `WHILE...WEND` | `WHILE x < 100` |
| `GOSUB` | `GOSUB label` |
| `SUB` | `SUB Name(params)` |
| `FUNCTION` | `FUNCTION Name(params)` |
| `DIM` | `DIM arr(10)` |
| `SELECT CASE` | `SELECT CASE x` |
| `FORWARD` | `FORWARD 100` |
| `COLOR` | `COLOR "red"` |
| `LINE` | `LINE (0,0)-(100,100), "white"` |
| `CIRCLE` | `CIRCLE (100,100), 50, "blue"` |
