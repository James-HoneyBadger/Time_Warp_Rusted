# Tutorial 03 — Logo & Turtle Graphics

## Introduction

Logo was created in 1967 by Seymour Papert to teach children programming
through art and geometry. You control a **turtle** on screen that leaves
a trail as it moves — drawing pictures by giving it simple commands.

**What you'll learn:**
- Moving the turtle (forward, back, turn)
- Drawing shapes with REPEAT
- Using colours and pen widths
- Creating procedures (your own commands)
- Variables and arithmetic
- Recursion (procedures that call themselves)
- Filled shapes, dots, labels, and arcs

---

## Lesson 1: Moving the Turtle

The turtle starts in the centre, facing up. Give it commands:

```logo
FD 100       ; Move forward 100 pixels
RT 90        ; Turn right 90 degrees
FD 100       ; Move forward again
```

### Basic Movement Commands

| Command | Short | What It Does |
|---------|-------|-------------|
| `FORWARD 100` | `FD 100` | Move forward 100 pixels |
| `BACK 50` | `BK 50` | Move backward 50 pixels |
| `RIGHT 90` | `RT 90` | Turn right 90 degrees |
| `LEFT 45` | `LT 45` | Turn left 45 degrees |

**Try it:** Make the turtle walk in a zigzag pattern.

---

## Lesson 2: Drawing a Square

To draw a square, go forward and turn right — four times:

```logo
FD 100
RT 90
FD 100
RT 90
FD 100
RT 90
FD 100
RT 90
```

That's a lot of typing! Use `REPEAT` instead:

```logo
REPEAT 4 [FD 100 RT 90]
```

**The rule:** An N-sided shape needs turns of `360 / N` degrees.

**Challenge:** Draw a triangle (`REPEAT 3 [FD 100 RT 120]`).

---

## Lesson 3: Shapes Gallery

```logo
; Triangle
REPEAT 3 [FD 80 RT 120]

; Pentagon
PU FD 120 PD
REPEAT 5 [FD 60 RT 72]

; Hexagon
PU FD 120 PD
REPEAT 6 [FD 50 RT 60]

; Octagon
PU FD 120 PD
REPEAT 8 [FD 40 RT 45]

; Circle (approximation)
PU FD 120 PD
REPEAT 36 [FD 5 RT 10]
```

**Challenge:** Draw all regular polygons from triangle (3) to dodecagon (12).

---

## Lesson 4: Colours and Pen

### Setting Colours

```logo
SETPENCOLOR "red"
FD 100
SETPENCOLOR "blue"
RT 90
FD 100
SETPENCOLOR "green"
RT 90
FD 100
```

You can use:
- **Named colours:** `"red"`, `"blue"`, `"green"`, `"purple"`, `"gold"`, etc.
- **Hex colours:** `"#FF6600"` for orange
- **RGB:** Some colours have descriptive names like `"deepskyblue"`

### Pen Width

```logo
PW 1
FD 50
PW 3
FD 50
PW 5
FD 50
PW 10
FD 50
```

### Pen Up and Down

```logo
FD 50          ; Draw a line
PU             ; Lift pen (stop drawing)
FD 30          ; Move without drawing
PD             ; Put pen down (start drawing)
FD 50          ; Draw again
```

**Challenge:** Draw a dashed line by alternating PU and PD in a loop.

---

## Lesson 5: Variables

Use `MAKE` to create variables and `:NAME` to read them:

```logo
MAKE "SIZE 100
MAKE "ANGLE 90

REPEAT 4 [FD :SIZE RT :ANGLE]

MAKE "SIZE 50
REPEAT 4 [FD :SIZE RT :ANGLE]
```

### Arithmetic with Variables

```logo
MAKE "A 10
MAKE "B :A * 2
MAKE "C :A + :B
PRINT :C
```

**Challenge:** Make a variable called `SIDES`, set it to different values, and draw the matching polygon.

---

## Lesson 6: Procedures

Procedures let you create your own commands:

```logo
TO SQUARE :SIZE
  REPEAT 4 [FD :SIZE RT 90]
END

TO TRIANGLE :SIZE
  REPEAT 3 [FD :SIZE RT 120]
END

TO POLYGON :SIDES :SIZE
  REPEAT :SIDES [FD :SIZE RT 360 / :SIDES]
END

; Use your procedures:
SQUARE 100
PU FD 120 PD
TRIANGLE 80
PU FD 120 PD
POLYGON 6 50
```

**Challenge:** Create a `STAR` procedure that draws a five-pointed star of any size.

---

## Lesson 7: Recursion

A procedure that calls itself creates amazing fractal art!

### Recursive Tree

```logo
TO TREE :SIZE
  IF :SIZE < 5 [STOP]
  FD :SIZE
  LT 30
  TREE :SIZE * 0.7
  RT 60
  TREE :SIZE * 0.7
  LT 30
  BK :SIZE
END

PU SETXY 0 -120 PD
SETPENCOLOR "forestgreen"
TREE 60
```

**How it works:**
1. Draw a trunk (`FD :SIZE`)
2. Turn left and draw a smaller tree
3. Turn right and draw another smaller tree
4. Back up to where we started
5. Stop when the size gets too small

### Koch Snowflake

```logo
TO KOCH :SIZE :DEPTH
  IF :DEPTH = 0 [FD :SIZE STOP]
  KOCH :SIZE / 3 :DEPTH - 1
  LT 60
  KOCH :SIZE / 3 :DEPTH - 1
  RT 120
  KOCH :SIZE / 3 :DEPTH - 1
  LT 60
  KOCH :SIZE / 3 :DEPTH - 1
END

TO SNOWFLAKE :SIZE :DEPTH
  REPEAT 3 [KOCH :SIZE :DEPTH RT 120]
END

SETPENCOLOR "deepskyblue"
SNOWFLAKE 200 3
```

**Challenge:** Modify the tree to use different colours for each branch level.

---

## Lesson 8: Filled Shapes

```logo
SETPENCOLOR "gold"
BEGINFILL
REPEAT 5 [FD 60 RT 144]
ENDFILL
```

### DOT and LABEL

```logo
; Draw coloured dots
SETPENCOLOR "red"
DOT 20
PU FD 50 PD
SETPENCOLOR "blue"
DOT 20
PU FD 50 PD
SETPENCOLOR "green"
DOT 20

; Add a label
PU SETXY -50 -80 PD
SETPENCOLOR "white"
LABEL [My Logo Art]
```

### ARC

```logo
SETPENCOLOR "orange"
ARC 180 50     ; Half circle
RT 90
SETPENCOLOR "purple"
ARC 90 40      ; Quarter circle
```

**Challenge:** Draw a filled flower with 6 coloured petals.

---

## Lesson 9: Positioning

### SETXY — Jump to Coordinates

```logo
PU
SETXY -100 100
PD
REPEAT 4 [FD 50 RT 90]

PU
SETXY 100 100
PD
REPEAT 3 [FD 50 RT 120]
```

### HOME — Return to Centre

```logo
FD 100
RT 90
FD 100
HOME          ; Jump back to centre
```

### CS — Clear Screen

```logo
; Draw something
REPEAT 50 [FD 5 RT 10]
CS            ; Clear everything
; Start fresh
REPEAT 4 [FD 100 RT 90]
```

---

## Lesson 10: Control Flow

### IF

```logo
MAKE "X 42
IF :X > 0 [PRINT [Positive!]]
```

### IFELSE

```logo
TO CHECK_EVEN :N
  IFELSE :N % 2 = 0 [
    PRINT [Even]
  ] [
    PRINT [Odd]
  ]
END

CHECK_EVEN 4
CHECK_EVEN 7
```

### STOP

Used inside procedures to exit early:

```logo
TO COUNTDOWN :N
  IF :N = 0 [PRINT [Blast off!] STOP]
  PRINT :N
  COUNTDOWN :N - 1
END

COUNTDOWN 10
```

---

## Projects

### Project 1: Shape Sampler
Draw every regular polygon from triangle to dodecagon, each in a
different colour, arranged in a circle.

### Project 2: Fractal Forest
Draw 5 trees of different sizes in a row on a green "ground" line.

### Project 3: Spirograph
Use `REPEAT` with slightly off-angle turns to create spirograph patterns.
Try: `REPEAT 200 [FD 3 RT 91]`

### Project 4: Mandala
Create a symmetrical mandala by rotating shapes around a centre point.
Use filled shapes for extra impact.

---

## Quick Reference

| Command | What It Does |
|---------|-------------|
| `FD n` / `FORWARD n` | Move forward |
| `BK n` / `BACK n` | Move backward |
| `RT n` / `RIGHT n` | Turn right |
| `LT n` / `LEFT n` | Turn left |
| `PU` / `PENUP` | Stop drawing |
| `PD` / `PENDOWN` | Start drawing |
| `SETPENCOLOR "c"` | Set colour |
| `PW n` | Set pen width |
| `HOME` | Return to centre |
| `CS` | Clear screen |
| `SETXY x y` | Move to position |
| `REPEAT n [...]` | Repeat commands |
| `TO name :param` | Define procedure |
| `MAKE "var val` | Set variable |
| `:var` | Read variable |
| `BEGINFILL`/`ENDFILL` | Fill a shape |
| `DOT n` | Draw a dot |
| `ARC angle radius` | Draw an arc |
| `LABEL [text]` | Write text |
| `IF cond [...]` | Conditional |
| `IFELSE cond [...][...]` | If/else |
| `STOP` | Exit procedure |
| `PRINT [text]` | Output text |
