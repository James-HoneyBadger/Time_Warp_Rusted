# Examples Guide

Time Warp Rusted ships with **74+ example programs** across seven languages
and nine directories. Examples progress from simple "Hello World" to
complete showcases demonstrating every language feature.

---

## Directory Structure

```
Examples/
├── basic/          11 tutorials + 7 quick examples
├── logo/           10 tutorials + 6 quick examples
├── c/              8 tutorials + 4 quick examples
├── pascal/         8 tutorials + 2 quick examples
├── forth/          4 tutorials + 1 showcase
├── pilot/          9 tutorials + 4 quick examples
├── prolog/         Logical programming examples
└── demo/           Multi-language showcases (8 files)
```

---

## Loading Examples

### From the Examples Panel

1. Click **Examples** in the left sidebar
2. Browse by language category
3. Click any example to load it into the editor
4. Press **F5** to run

### From the File Menu

1. **File → Open** (Ctrl + O)
2. Navigate to the `Examples/` directory
3. Select a file

### Auto-detection

When you open a file, Time Warp Rusted automatically detects the language
from the file extension and switches the interpreter.

---

## BASIC Examples

| File | Topic | Key Concepts |
|------|-------|-------------|
| `01_hello_world.bas` | Hello World | `PRINT`, basic output |
| `02_variables_and_types.bas` | Variables | `LET`, numeric/string types |
| `03_arithmetic.bas` | Arithmetic | Operators, math functions |
| `04_input_output.bas` | Input/Output | `INPUT`, `PRINT`, formatting |
| `05_conditionals.bas` | Conditionals | `IF`/`THEN`/`ELSE`, `SELECT CASE` |
| `06_loops.bas` | Loops | `FOR`/`NEXT`, `WHILE`/`WEND`, `DO`/`LOOP` |
| `07_subroutines.bas` | Subroutines | `SUB`, `FUNCTION`, `GOSUB` |
| `08_strings.bas` | Strings | `LEFT$`, `MID$`, `INSTR`, etc. |
| `09_arrays.bas` | Arrays | `DIM`, indexing, iteration |
| `10_guessing_game.bas` | Complete game | All concepts combined |
| `11_showcase.bas` | Showcase | Every BASIC feature |

Quick-start files: `hello_world.bas`, `input.bas`, `loops.bas`,
`math.bas`, `subroutines.bas`, `guessing_game.bas`, `showcase.bas`

---

## Logo Examples

| File | Topic | Key Concepts |
|------|-------|-------------|
| `01_hello_world.logo` | First drawing | `FD`, `RT`, basic shapes |
| `02_squares.logo` | Squares | `REPEAT`, procedures |
| `03_polygons.logo` | Polygons | Parameters, `360 / :sides` |
| `04_spirals.logo` | Spirals | Growing patterns, color |
| `05_trees.logo` | Recursive trees | Recursion, branching |
| `06_patterns.logo` | Patterns | Nested repeats, symmetry |
| `07_geometric.logo` | Geometry | Complex geometric art |
| `08_artistic.logo` | Art | Creative color compositions |
| `09_showcase.logo` | Showcase | Every Logo feature |
| `10_graphics_demo.logo` | Graphics demo | Advanced drawing techniques |

Quick-start files: `square.logo`, `spiral.logo`, `colors.logo`,
`procedures.logo`, `recursion.logo`, `showcase.logo`

---

## C Examples

| File | Topic | Key Concepts |
|------|-------|-------------|
| `01_hello_world.c` | Hello World | `printf`, `main()` |
| `02_variables_types.c` | Variables | `int`, `float`, `char` |
| `03_input_output.c` | I/O | `scanf`, `printf` format specifiers |
| `04_conditionals.c` | Conditionals | `if`/`else`, `switch` |
| `05_loops.c` | Loops | `for`, `while`, `do-while` |
| `06_functions.c` | Functions | Parameters, return, recursion |
| `07_arrays.c` | Arrays | Declaration, indexing |
| `08_calculator.c` | Calculator | Complete interactive project |

Quick-start files: `demo.c`, `showcase.c`, `mini_test.c`, `for_test.c`

---

## Pascal Examples

| File | Topic | Key Concepts |
|------|-------|-------------|
| `01_hello_world.pas` | Hello World | `WriteLn`, `program` |
| `02_variables_types.pas` | Variables | `var`, `const`, types |
| `03_input_output.pas` | I/O | `ReadLn`, `WriteLn` |
| `04_conditionals.pas` | Conditionals | `if`/`then`/`else` |
| `05_loops.pas` | Loops | `for`, `while`, `repeat` |
| `06_procedures_functions.pas` | Procedures | `procedure`, `function` |
| `07_arrays.pas` | Arrays | `array[1..n] of Type` |
| `08_calculator.pas` | Calculator | Complete project |

Quick-start files: `demo.pas`, `showcase.pas`

---

## Forth Examples

| File | Topic | Key Concepts |
|------|-------|-------------|
| `01_hello_world.f` | Hello World | `.`, `."`, stack basics |
| `02_arithmetic.f` | Arithmetic | Stack operations, math |
| `03_loops.f` | Loops | `DO`/`LOOP`, `BEGIN`/`UNTIL` |
| `04_graphics.f` | Graphics | Turtle graphics in Forth |
| `showcase.f` | Showcase | Every Forth feature |

---

## PILOT Examples

| File | Topic | Key Concepts |
|------|-------|-------------|
| `01_hello_world.pilot` | Hello World | `T:`, basic output |
| `02_variables.pilot` | Variables | `C:`, `#` interpolation |
| `03_arithmetic.pilot` | Arithmetic | Expressions, `C:` |
| `04_conditionals.pilot` | Conditionals | `M:`, `Y:`, `N:` |
| `05_loops.pilot` | Loops | `J:`, labels |
| `06_subroutines.pilot` | Subroutines | `U:`, `E:` |
| `07_strings.pilot` | Strings | String matching, `M:` |
| `08_guessing_game.pilot` | Game | Complete interactive quiz |
| `09_showcase.pilot` | Showcase | Every PILOT feature |

Quick-start files: `hello.pilot`, `quiz.pilot`, `match.pilot`,
`compute.pilot`

---

## Prolog Examples

The `Examples/prolog/` directory contains logic programming examples
covering facts, rules, queries, list operations, and arithmetic.

---

## Demo Showcases

The `Examples/demo/` directory contains one showcase per language plus a
special graphics test:

| File | Language | Description |
|------|----------|-------------|
| `demo_basic.bas` | BASIC | Full BASIC showcase |
| `demo_c.c` | C | Full C showcase |
| `demo_forth.f` | Forth | Full Forth showcase |
| `demo_logo.logo` | Logo | Full Logo showcase |
| `demo_pascal.pas` | Pascal | Full Pascal showcase |
| `demo_pilot.pilot` | PILOT | Full PILOT showcase |
| `demo_prolog.pro` | Prolog | Full Prolog showcase |
| `test_turbo_basic_graphics.bas` | BASIC | Turbo BASIC graphics test |

---

## Progression Path

For learners, the recommended order within each language is:

1. **Hello World** — basic output
2. **Variables** — data types and assignment
3. **Arithmetic / I/O** — expressions and input
4. **Conditionals** — decisions and branching
5. **Loops** — iteration and repetition
6. **Functions / Procedures** — code organization
7. **Arrays / Strings** — data structures
8. **Calculator / Game** — complete project
9. **Showcase** — comprehensive review

For cross-language learning, try the same tutorial number across
languages to see how each one approaches the same concept:

```
01_hello_world.bas  →  01_hello_world.c  →  01_hello_world.pas  →  …
```
