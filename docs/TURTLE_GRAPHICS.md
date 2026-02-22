# Turtle Graphics

All seven languages share the same turtle graphics system.  The turtle lives at the centre of the canvas (0, 0) with its head pointing up (north = 0°).

---

## Coordinate System

- Origin (0, 0) at centre of canvas
- X increases rightward
- Y increases upward (mathematical convention)
- Heading: 0 = north, 90 = east, 180 = south, 270 = west

---

## Basic Movement

| Logo | BASIC | Forth | Description |
|------|-------|-------|-------------|
| `FORWARD n` | `FORWARD n` | `n FD` | Move forward n steps |
| `BACK n` | `BACKWARD n` | `n BK` | Move backward |
| `RIGHT deg` | `RIGHT deg` | `deg RT` | Turn right |
| `LEFT deg` | `LEFT deg` | `deg LT` | Turn left |

---

## Pen Control

| Logo | BASIC | Forth | Description |
|------|-------|-------|-------------|
| `PENUP` / `PU` | `PENUP` | `PU` | Lift pen (move without drawing) |
| `PENDOWN` / `PD` | `PENDOWN` | `PD` | Lower pen |
| `SETPENCOLOR color` | `COLOR r,g,b` | `n PEN` | Set pen colour |
| `SETPENWIDTH n` | — | — | Set pen width |

Colours can be:
- Named: `red`, `blue`, `green`, `white`, `black`, `yellow`, `cyan`, `magenta`, `orange`, `purple`, `brown`, `gray` / `grey`
- Hex: `#FF8800`
- CGA palette index 0–15 (for BASIC/Forth)

---

## Position

| Logo | BASIC | Description |
|------|-------|-------------|
| `SETXY x y` | — | Jump to (x, y) |
| `SETX x` | — | Set X only |
| `SETY y` | — | Set Y only |
| `HOME` | `HOME` | Return to (0,0) facing north |
| `CLEARSCREEN` / `CS` | `CLEARSCREEN` | Clear canvas, home turtle |

---

## Shapes

### Arc
```logo
ARC radius angle
```
Draws an arc of the given radius sweeping through `angle` degrees starting from the current heading.

### Dot
```logo
DOT radius
```
Draws a filled circle centred at the current turtle position.

### Filled Polygon (Logo)
```logo
SETPENCOLOR red
REPEAT 6 [FORWARD 80 RIGHT 60]
```
Logo automatically closes and fills polygons when you trace a closed path.

### Label
```logo
LABEL "Hello"
```
Draws text at the current turtle position.

---

## Canvas Controls

- **Scroll wheel** — zoom in/out
- **Click + drag** — pan the canvas
- **🔁 Reset view** (toolbar) — return to 1× zoom, centred

A grid overlay appears automatically when zoom > 3×.

---

## Background Colour

| Logo | BASIC |
|------|-------|
| `SETBGCOLOR color` | `COLOR r,g,b` (full canvas) |

---

## Example: Spirograph

```logo
REPEAT 360 [
  FORWARD REPCOUNT / 5
  RIGHT 1
]
```

## Example: Rainbow Star (BASIC)

```basic
10 FOR I = 1 TO 36
20   COLOR I * 7, 200 - I * 5, 100
30   FORWARD I * 3
40   RIGHT 170
50 NEXT I
60 END
```
