# Tutorial 07 — PILOT: Interactive Lessons & Quizzes

## Introduction

PILOT (Programmed Inquiry, Learning Or Teaching) was created by John Carbonell
in 1962 specifically for educators to create interactive lessons and quizzes.
It's the original "quiz-making" language!

PILOT has two syntax styles:
- **Classic:** Single-letter commands (`T:`, `A:`, `M:`)
- **Verbose:** Keyword commands (`PRINT`, `ACCEPT`, `MATCH`)

**What you'll learn:**
- Displaying text (T: / PRINT)
- Getting input (A: / ACCEPT)
- Pattern matching (M: / MATCH)
- Conditional output (TY: / TN:)
- Variables and computation (C: / COMPUTE)
- Labels and jumps (J: / JUMP)
- Subroutines (TU / RULE / RETURN)
- MATCH/CASE/DEFAULT/END blocks

---

## Lesson 1: Displaying Text

### Classic Syntax

```pilot
R: This is a comment (Remark)
T: Hello, World!
T: Welcome to PILOT!
D: Display also works for output.
```

- `R:` — Remark (comment, ignored by computer)
- `T:` — Type (print text)
- `D:` — Display (same as T:)

### Verbose Syntax

```pilot
REMARK This is a comment
PRINT Hello, World!
PRINT Welcome to PILOT!
PRINT
PRINT An empty PRINT gives a blank line.
```

**Challenge:** Write a program that displays your name, your school, and
your favourite subject using both classic and verbose syntax.

---

## Lesson 2: Getting Input

### Classic Syntax
```pilot
T: What is your name?
A: NAME
T: Hello, #NAME!
```

- `A:` — Accept (read input into variable)
- `#NAME` — Interpolate variable value in classic mode

### Verbose Syntax
```pilot
ACCEPT USERNAME What is your name
PRINT Hello, $USERNAME!
```

- `ACCEPT variable prompt` — Read input with a prompt
- `$VAR` — Interpolate variable value in verbose mode

**Important difference:**
- Classic uses `#VAR` for interpolation
- Verbose uses `$VAR` for interpolation

**Challenge:** Ask the user for three things about themselves and display
a summary.

---

## Lesson 3: Pattern Matching

This is what makes PILOT special! The `M:` command compares the last
input against patterns.

```pilot
T: Do you like pizza? (yes/no)
A: ANSWER
M: YES,Y,YEP,SURE,YEAH
TY: Great! Pizza is delicious!
TN: What?! You don't like pizza?
```

**How it works:**
1. `A:` reads user input
2. `M:` checks if the input matches any of the patterns (comma-separated)
3. `TY:` — Type if Yes (only runs if match succeeded)
4. `TN:` — Type if No (only runs if match failed)

### Wildcards

Use `*` as a wildcard that matches anything:

```pilot
T: Name a programming language:
A: LANG
M: *BASIC*
TY: BASIC — a classic!
M: *LOGO*
TY: Logo — great for graphics!
M: *PILOT*
TY: You chose PILOT!
```

### Conditional Jumps

```pilot
T: Are you ready? (yes/no)
A: READY
M: YES,Y
Y: START_LESSON
N: NOT_READY

*START_LESSON
T: Let's begin!
J: END

*NOT_READY
T: Come back when you're ready!

*END
E:
```

- `Y: LABEL` — Jump to label if last match succeeded
- `N: LABEL` — Jump to label if last match failed

**Challenge:** Write a program that asks the user's favourite colour and
responds differently for red, blue, green, and anything else.

---

## Lesson 4: Variables and Computation

### Classic Syntax (C:)

```pilot
C: X = 10
C: Y = 25
C: SUM = X + Y
C: PRODUCT = X * Y
T: X = #X, Y = #Y
T: Sum = #SUM
T: Product = #PRODUCT
```

### Verbose Syntax (COMPUTE)

```pilot
COMPUTE PRICE 19
COMPUTE QUANTITY 5
COMPUTE TOTAL $PRICE * $QUANTITY
PRINT $QUANTITY items at $PRICE each = $TOTAL total
```

COMPUTE can do arithmetic: `+`, `-`, `*`, `/`

**Challenge:** Create a simple tip calculator: ask for the bill amount and
tip percentage, then compute the total.

---

## Lesson 5: MATCH/CASE/DEFAULT/END Blocks

The verbose syntax supports structured decision blocks:

```pilot
ACCEPT SCORE Enter your test score (0-100)

MATCH $SCORE
  CASE [90-100]:
    PRINT Grade: A - Excellent!
  CASE [80-89]:
    PRINT Grade: B - Very Good!
  CASE [70-79]:
    PRINT Grade: C - Good
  CASE [60-69]:
    PRINT Grade: D - Needs Work
  DEFAULT:
    PRINT Grade: F - Study Harder!
END
```

### How CASE works:

- **Exact number:** `CASE 42:` — matches if value equals 42
- **Range:** `CASE [10-20]:` — matches if value is between 10 and 20
- **String:** `CASE Monday:` — matches string (case-insensitive)
- **DEFAULT:** — matches if nothing else did

### Nested MATCH blocks

You can put MATCH blocks inside other MATCH blocks for complex decisions.

**Challenge:** Build a "choose your own adventure" with MATCH/CASE for
each decision point.

---

## Lesson 6: Labels and Jumps

Labels let you create loops and branches:

```pilot
COMPUTE COUNT 10

*COUNTDOWN
PRINT $COUNT...
COMPUTE COUNT $COUNT - 1
MATCH $COUNT
  CASE 0:
    PRINT Blast off!
    JUMP DONE
  DEFAULT:
    JUMP COUNTDOWN
END

*DONE
PRINT Mission complete!
```

**Key points:**
- `*LABEL` — Defines a label (position in the program)
- `J: LABEL` or `JUMP LABEL` — Unconditionally jump to a label
- `Y: LABEL` — Jump if last match was true
- `N: LABEL` — Jump if last match was false

**Challenge:** Create a counting loop that prints 1 to 20 using labels and JUMP.

---

## Lesson 7: Subroutines

### Verbose Subroutines

```pilot
PRINT Main program starting...
TU GREET
PRINT Back in main program.
TU GREET
PRINT Done!

JUMP END

*GREET
RULE GREET
PRINT +---------------------+
PRINT |      Hello!         |
PRINT +---------------------+
RETURN

*END
E:
```

- `TU name` — Transfer to Use (call subroutine)
- `*label` + `RULE name` — Marks the subroutine start
- `RETURN` — Return to the caller

### Subroutines with Shared Variables

Since PILOT variables are global, subroutines can read and modify them:

```pilot
COMPUTE NUM 6
TU SQUARE_IT
PRINT $NUM squared = $RESULT

COMPUTE NUM 9
TU SQUARE_IT
PRINT $NUM squared = $RESULT

JUMP END

*SQUARE_IT
RULE SQUARE_IT
COMPUTE RESULT $NUM * $NUM
RETURN

*END
E:
```

**Challenge:** Create subroutines for: draw a line, draw a box header,
and draw a box footer. Use them to create a formatted display.

---

## Lesson 8: Building a Quiz

PILOT was made for this! Here's a complete quiz structure:

```pilot
T: ===========================
T:  Geography Quiz
T: ===========================

C: SCORE = 0

T: Q1: What is the capital of France?
A: ANS
M: PARIS
TY: Correct!
C: SCORE = SCORE + 1
TN: Sorry, it's Paris.

T: Q2: What is the largest continent?
A: ANS
M: ASIA,*ASIA*
TY: Correct!
C: SCORE = SCORE + 1
TN: It's Asia.

T: Q3: What ocean is the deepest?
A: ANS
M: PACIFIC,PACIFIC OCEAN,*PACIFIC*
TY: Correct!
C: SCORE = SCORE + 1
TN: It's the Pacific Ocean.

T:
T: You scored #SCORE out of 3!

MATCH $SCORE
  CASE 3:
    PRINT Perfect! Geography expert!
  CASE 2:
    PRINT Well done!
  CASE 1:
    PRINT Keep studying!
  CASE 0:
    PRINT Try again!
END
```

**Tips for good quizzes:**
1. Accept multiple correct formats (`M: YES,Y,YEP`)
2. Use wildcards for flexible matching (`M: *PARIS*`)
3. Give feedback after each question (`TY:` / `TN:`)
4. Keep a running score with `C:` / `COMPUTE`
5. Give overall feedback at the end with `MATCH/CASE`

**Challenge:** Create a 5-question quiz on a topic you're studying. Include
score tracking and grade feedback.

---

## Lesson 9: Interactive Story

PILOT is perfect for interactive fiction:

```pilot
PRINT ===========================
PRINT   The Enchanted Forest
PRINT ===========================
PRINT
PRINT You stand at the edge of a dark forest.
PRINT Two paths lead into the trees.
PRINT
ACCEPT CHOICE Go LEFT or RIGHT

M: LEFT,L
Y: LEFT_PATH
N: RIGHT_PATH

*LEFT_PATH
PRINT
PRINT You take the left path and find a stream.
PRINT A bridge crosses it. Do you cross?
ACCEPT CROSS Cross the bridge? (yes/no)
M: YES,Y
TY: You cross safely and find a treasure chest!
TN: You follow the stream and find a village.
JUMP ENDING

*RIGHT_PATH
PRINT
PRINT You take the right path into thick woods.
PRINT You hear a sound. Investigate?
ACCEPT INVESTIGATE Investigate the sound? (yes/no)
M: YES,Y
TY: It is a friendly owl! It guides you to safety.
TN: You keep walking and emerge in a clearing.
JUMP ENDING

*ENDING
PRINT
PRINT ===========================
PRINT   THE END
PRINT ===========================

E:
```

**Challenge:** Expand this story to have at least 5 decision points and
multiple endings.

---

## Projects

### Project 1: Quiz Maker
Create a 10-question quiz on any school subject with scoring,
per-question feedback, and a final grade.

### Project 2: Survey
Build a survey that asks 5 questions, categorises responses,
and gives a personality-type result.

### Project 3: Interactive Tutorial
Create a PILOT program that teaches someone about a topic you know well,
with questions to check understanding.

### Project 4: Choose Your Own Adventure
Create a branching story with at least 3 different endings.

---

## Quick Reference

### Classic Commands
| Command | What It Does |
|---------|-------------|
| `R:` | Remark (comment) |
| `T:` | Type (print text) |
| `D:` | Display (same as T:) |
| `A: VAR` | Accept input into variable |
| `M: patterns` | Match last input against patterns |
| `C: VAR = expr` | Compute/assign variable |
| `J: LABEL` | Jump to label |
| `Y: LABEL` | Jump if last match true |
| `N: LABEL` | Jump if last match false |
| `TY:` | Type if Yes |
| `TN:` | Type if No |
| `E:` | End program |

### Verbose Commands
| Command | What It Does |
|---------|-------------|
| `REMARK` | Comment |
| `PRINT text` | Print text |
| `ACCEPT var prompt` | Read input |
| `COMPUTE var expr` | Calculate/assign |
| `MATCH val` | Begin MATCH block |
| `CASE value:` | Match case |
| `CASE [low-high]:` | Range match |
| `DEFAULT:` | Default case |
| `END` | End MATCH block |
| `JUMP label` | Jump to label |
| `TU name` | Call subroutine |
| `RULE name` | Define subroutine |
| `RETURN` | Return from subroutine |
| `STOP` | End program |

### Variable Interpolation
| Context | Syntax |
|---------|--------|
| Classic (T:) | `#VARNAME` |
| Verbose (PRINT) | `$VARNAME` |
