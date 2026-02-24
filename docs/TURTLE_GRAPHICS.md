# Turtle Graphics Guide

Time Warp Rusted includes a full turtle graphics engine, accessible from
BASIC, Logo, and Forth. The turtle draws on the **Canvas** tab — a zoomable,
pannable drawing surface.

---

## Quick Start

### Logo

```logo
; Draw a square
REPEAT 4 [FD 100 RT 90]
```

### BASIC

```basic
' Draw a triangle
FOR I = 1 TO 3
    FORWARD 100
    RIGHT 120
NEXT I
```

### Forth

```forth
\ Draw a pentagon
: PENTAGON  5 0 DO 72 RT 80 FD LOOP ;
PENTAGON
```

Press **F5** (or click ▶ Run), then switch to the **Canvas** tab to see your
drawing.

---

## The Canvas

The canvas is an infinite drawing surface. The turtle starts at the center
(0, 0), facing north (up).

### Navigation

| Action | Mouse | Keyboard |
|--------|-------|----------|
| Zoom in / out | Scroll wheel | — |
| Pan | Click and drag | — |
| Reset view | — | Click "Reset View" button |

### Coordinate System

- **Origin** (0, 0) is at the center of the canvas
- **X** increases to the right
- **Y** increases upward
- **Heading 0°** points north (up); angles increase clockwise

---

## Movement Commands

| Command (Logo) | BASIC Equivalent | Forth | Description |
|----------------|------------------|-------|-------------|
| `FD n` / `FORWARD n` | `FORWARD n` | `n FD` | Move forward n pixels |
| `BK n` / `BACK n` | `BACKWARD n` | `n BK` | Move backward n pixels |
| `RT n` / `RIGHT n` | `RIGHT n` | `n RT` | Turn clockwise n degrees |
| `LT n` / `LEFT n` | `LEFT n` | `n LT` | Turn counter-clockwise n degrees |
| `HOME` | `HOME` | `HOME` | Return to (0,0) facing north |
| `SETXY x y` | — | — | Move to coordinates (x, y) |
| `SETX x` | — | — | Set X, keep Y |
| `SETY y` | — | — | Set Y, keep X |
| `SETHEADING n` | — | — | Set absolute heading in degrees |

---

## Pen Commands

| Command (Logo) | BASIC Equivalent | Forth | Description |
|----------------|------------------|-------|-------------|
| `PU` / `PENUP` | `PENUP` | `PU` | Lift pen — movement won't draw |
| `PD` / `PENDOWN` | `PENDOWN` | `PD` | Lower pen — movement draws |
| `SETPENCOLOR c` | `COLOR r, g, b` | `n COLOR` | Set drawing color |
| `SETPENWIDTH n` | — | — | Set line width in pixels |
| `CS` / `CLEARSCREEN` | `CLEARSCREEN` | — | Clear canvas & reset turtle |

---

## Colors

### Logo Colors

Logo accepts colors as **names**, **hex codes**, or **CGA indices** (0–15):

| Index | Name | Color |
|-------|------|-------|
| 0 | black | Black |
| 1 | blue | Blue |
| 2 | green | Green |
| 3 | cyan | Cyan |
| 4 | red | Red |
| 5 | magenta | Magenta |
| 6 | brown | Brown |
| 7 | lightgray | Light gray |
| 8 | darkgray | Dark gray |
| 9 | lightblue | Light blue |
| 10 | lightgreen | Light green |
| 11 | lightcyan | Light cyan |
| 12 | lightred | Light red |
| 13 | lightmagenta | Bright magenta |
| 14 | yellow | Yellow |
| 15 | white | White |

Plus 60+ named colors: `orange`, `pink`, `purple`, `gold`, `coral`,
`salmon`, `violet`, `indigo`, `lime`, `teal`, `navy`, `maroon`,
`olive`, `silver`, `crimson`, `turquoise`, `lavender`, `chocolate`,
`tan`, `wheat`, `khaki`, `plum`, `orchid`, `sienna`, `peru`, and many
more. Hex codes like `#FF6600` are also supported.

```logo
SETPENCOLOR "red
SETPENCOLOR "#FF6600
SETPENCOLOR 14
SETBGCOLOR "navy
```

### BASIC Colors

BASIC uses RGB values (0–255 each):

```basic
COLOR 255, 0, 0       ' Red
COLOR 0, 128, 255     ' Blue
```

### Forth Colors

Forth uses CGA color indices (0–15):

```forth
4 COLOR    \ Set to red
```

---

## Drawing Shapes

### Squares

```logo
TO SQUARE :size
    REPEAT 4 [FD :size RT 90]
END

SQUARE 100
```

### Regular Polygons

```logo
TO POLYGON :sides :size
    REPEAT :sides [FD :size RT 360 / :sides]
END

POLYGON 6 60    ; hexagon
```

### Circles

```logo
TO CIRCLE :radius
    REPEAT 360 [FD :radius * 3.14159 / 180 RT 1]
END

CIRCLE 80
```

Or use the `ARC` command:

```logo
ARC 80 360    ; full circle, radius 80
ARC 80 180    ; semicircle
```

### Stars

```logo
TO STAR :size
    REPEAT 5 [FD :size RT 144]
END

SETPENCOLOR "gold
STAR 120
```

### Filled Shapes

```logo
SETPENCOLOR "blue
BEGINFILL
REPEAT 4 [FD 80 RT 90]
ENDFILL
```

---

## Patterns and Art

### Spirograph

```logo
TO SPIROGRAPH :n :size
    REPEAT :n [
        REPEAT 4 [FD :size RT 90]
        RT 360 / :n
    ]
END

SETPENCOLOR "cyan
SPIROGRAPH 36 60
```

### Spiral

```logo
TO SPIRAL :size :angle :growth
    IF :size > 200 [STOP]
    FD :size
    RT :angle
    SPIRAL :size + :growth :angle :growth
END

SETPENCOLOR "magenta
SPIRAL 5 91 2
```

### Recursive Tree

```logo
TO TREE :size
    IF :size < 5 [STOP]
    FD :size
    LT 30
    TREE :size * 0.7
    RT 60
    TREE :size * 0.7
    LT 30
    BK :size
END

PU SETY -100 PD
SETPENCOLOR "green
TREE 80
```

### Color Wheel

```logo
TO COLORWHEEL
    REPEAT 15 [
        SETPENCOLOR REPCOUNT
        REPEAT 4 [FD 40 RT 90]
        RT 24
    ]
END

COLORWHEEL
```

---

## Advanced Features

### BASIC Drawing Commands

BASIC includes pixel-level drawing commands in addition to turtle graphics:

| Command | Description | Example |
|---------|-------------|---------|
| `LINE x1, y1, x2, y2` | Draw a line between two points | `LINE 0, 0, 100, 100` |
| `CIRCLE x, y, r` | Draw a circle | `CIRCLE 200, 200, 50` |
| `PSET x, y` | Plot a single pixel | `PSET 100, 100` |
| `DRAW "cmds"` | Draw using a command string | `DRAW "U10 R10 D10 L10"` |

### Logo Text

```logo
SETPENCOLOR "white
LABEL "Hello
```

### Logo Dots

```logo
SETPENCOLOR "red
DOT 10        ; filled circle, radius 10
```

---

## Tips and Tricks

1. **Use procedures** to avoid repetition — break complex drawings into
   small reusable shapes.

2. **Pen up/down** lets you move without drawing — useful for positioning.

3. **Zoom out** on the canvas to see large drawings — scroll wheel or
   pinch to zoom.

4. **Angles**: Remember that Logo angles are clockwise from north. A full
   turn is 360°. For a regular polygon with N sides, turn `360 / N`
   degrees.

5. **Colors**: Use named colors for readability, hex codes for precision.

6. **Fractals**: Use recursion with a size cutoff (`IF :size < 5 [STOP]`)
   to create beautiful fractal patterns.

7. **Animation-style effects**: Use `CS` (clear screen) and loops with
   small delays to create simple animations.

---

## Example Programs

See the `Examples/logo/` directory for ready-to-run turtle graphics
programs:

| File | Description |
|------|-------------|
| `01_hello_world.logo` | Basic drawing introduction |
| `02_squares.logo` | Square variations |
| `03_polygons.logo` | Regular polygons |
| `04_spirals.logo` | Spiral patterns |
| `05_trees.logo` | Recursive trees |
| `06_patterns.logo` | Geometric patterns |
| `07_geometric.logo` | Advanced geometry |
| `08_artistic.logo` | Artistic designs |
| `09_showcase.logo` | Comprehensive demo |
| `10_graphics_demo.logo` | Full graphics showcase |

Load them from the **Examples** panel (left sidebar) or via
**File → Open**.
