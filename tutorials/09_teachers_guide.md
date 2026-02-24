# Tutorial 09 — Teacher's Guide

## Overview

This guide helps teachers use Time Warp Rusted to deliver computing lessons.
It covers lesson planning, age-appropriate language choices, cross-language
activities, assessment strategies, and curriculum alignment.

---

## Recommended Language Progression by Age

### Ages 7–10 (Key Stage 2 / Primary)
1. **Logo** — Visual, immediate feedback, teaches geometry concepts
2. **PILOT** — Interactive stories and simple quizzes
3. **BASIC** — Simple PRINT and INPUT programs

### Ages 11–14 (Key Stage 3 / Lower Secondary)
1. **BASIC** — Full feature set: variables, loops, conditionals, functions
2. **Logo** — Procedures, recursion, computational geometry
3. **Pascal** — Structured programming, formal syntax
4. **PILOT** — Building educational tools for younger students

### Ages 14–16 (Key Stage 4 / GCSE)
1. **Pascal** — Algorithms, data structures, formal methods
2. **C** — Systems concepts, low-level understanding
3. **Prolog** — Logic, reasoning, AI foundations
4. **Forth** — Alternative computational model, stack machines

### Ages 16–18 (A-Level / College)
1. **C** — Memory model, efficiency, computer architecture
2. **Prolog** — Declarative programming, AI, NLP
3. **Forth** — Stack machines, embedded systems concepts
4. All languages for comparative study

---

## Lesson Plan Template

```
Lesson: [Title]
Language: [Language]
Duration: [45-60 minutes]
Prior Knowledge: [What students should already know]

Learning Objectives:
  By the end of this lesson, students will be able to:
  1. [Objective 1]
  2. [Objective 2]
  3. [Objective 3]

Starter (5-10 min):
  [Warm-up activity or recap]

Main Activity (25-35 min):
  [Step-by-step instructions]

Extension:
  [For students who finish early]

Plenary (5-10 min):
  [Summary, questions, preview of next lesson]

Assessment:
  [How to check understanding]

Resources:
  [Example files, reference sheets]
```

---

## Sample Lesson Plans

### Lesson: Introduction to Programming (BASIC)
**Duration:** 45 minutes | **Ages:** 11+

**Objectives:**
1. Write and run a simple program
2. Use PRINT to display text
3. Use variables to store data

**Starter (10 min):**
Discuss: What is a program? Where do we encounter programs in daily life?

**Main Activity (25 min):**
1. Open Time Warp Rusted, select BASIC
2. Type: `PRINT "Hello, World!"` — Run it
3. Add more PRINT statements with their name, age, school
4. Introduce variables: `name$ = "Alice"` and `PRINT name$`
5. Introduce numeric variables: `age = 14` and `PRINT "I am "; age`
6. Challenge: Create a "digital business card" program

**Extension:**
Use INPUT to make the program interactive.

**Plenary (10 min):**
Students share their programs. Discuss: What did the computer do with our instructions?

**Assessment:** Can the student write a 5-line BASIC program that uses PRINT and variables?

---

### Lesson: Turtle Graphics Introduction (Logo)
**Duration:** 45 minutes | **Ages:** 8+

**Objectives:**
1. Control the turtle with FD, BK, RT, LT
2. Draw a square using REPEAT
3. Draw at least two different shapes

**Starter (10 min):**
Activity: Give a blindfolded student directions to walk across the room.
Discuss: How is this like controlling a turtle?

**Main Activity (25 min):**
1. Type `FD 100` — observe the turtle moves forward
2. Type `RT 90` then `FD 100` — observe the turn
3. Draw a square manually: FD 100 RT 90 FD 100 RT 90 FD 100 RT 90 FD 100 RT 90
4. Introduce REPEAT: `REPEAT 4 [FD 100 RT 90]`
5. Challenge: Draw a triangle (3 sides, 120-degree turns)
6. Explore: pentagon (5/72), hexagon (6/60), octagon (8/45)

**Extension:**
Add colours with `SETPENCOLOR "red"`. Draw a house using multiple shapes.

**Plenary (10 min):**
"What's the connection between the number of sides and the turn angle?"
(360 ÷ sides = turn angle)

**Assessment:** Can the student draw a named polygon using REPEAT?

---

### Lesson: Pattern Matching and Quizzes (PILOT)
**Duration:** 45 minutes | **Ages:** 10+

**Objectives:**
1. Get user input with A:/ACCEPT
2. Use M: and TY:/TN: for conditional responses
3. Create a 3-question quiz with scoring

**Starter (10 min):**
Play a quick verbal quiz game. Discuss: What makes a good quiz question?

**Main Activity (25 min):**
1. Start with output: `T: Welcome to the quiz!`
2. Add input: `T: What is 2 + 2?` then `A: ANS`
3. Add matching: `M: 4,FOUR` then `TY: Correct!` and `TN: Try again!`
4. Add scoring: `C: SCORE = 0` at the start, `C: SCORE = SCORE + 1` for correct
5. Students create their own 3-question quiz on any topic

**Extension:**
Add `MATCH/CASE/END` blocks for the final score feedback.

**Plenary (10 min):**
Students test each other's quizzes. Vote for the best one.

**Assessment:** Can the student create a working quiz with at least 3 questions and a score tracker?

---

### Lesson: Structured Programming (Pascal)
**Duration:** 60 minutes | **Ages:** 13+

**Objectives:**
1. Understand the structure of a Pascal program
2. Declare variables with appropriate types
3. Write a program using a procedure

**Starter (10 min):**
Compare a messy room to a tidy room. Discuss: Why does organisation matter
in programming?

**Main Activity (35 min):**
1. Explain program structure: `program`, `var`, `begin`/`end.`
2. Write a program that calculates rectangle area
3. Introduce procedures: extract the calculation into a procedure
4. Compare with the BASIC equivalent — what's different?
5. Challenge: Write a program with a `PrintBanner` procedure

**Extension:**
Add a function that returns a value (e.g., `Factorial`).

**Plenary (15 min):**
Discussion: What are the benefits of declaring types and using structure?

---

### Lesson: Logic Programming (Prolog)
**Duration:** 60 minutes | **Ages:** 14+

**Objectives:**
1. Create facts to represent knowledge
2. Write queries with variables
3. Define a simple rule

**Starter (10 min):**
Activity: "20 Questions" game. Discuss: How do we reason about things
based on what we know?

**Main Activity (35 min):**
1. Create facts: `cat(tom). dog(rex). bird(tweety).`
2. Query: `?- cat(tom).` → true, `?- cat(rex).` → false
3. Query with variable: `?- cat(X).` → finds all cats
4. Add properties: `has_legs(tom, 4). has_legs(tweety, 2).`
5. Write a rule: `is_pet(X) :- cat(X).` plus `is_pet(X) :- dog(X).`
6. Challenge: Build a small animal database with rules

**Extension:**
Add a recursive `ancestor` rule using a family tree.

**Plenary (15 min):**
"How is Prolog different from BASIC? What kind of problems would Prolog
be better at solving?"

---

## Cross-Language Comparison Activities

### Activity 1: "Hello World" in Seven Languages
Students write Hello World in all 7 languages. Create a poster showing
each version. Discuss: What's the same? What's different?

### Activity 2: Loop Comparison
Write a program that prints 1 to 10 in BASIC, Pascal, C, Logo, and Forth.
Complete a comparison table:

| Feature | BASIC | Pascal | C | Logo | Forth |
|---------|-------|--------|---|------|-------|
| Loop keyword | FOR/NEXT | for/do | for | REPEAT | DO/LOOP |
| Counter variable | Yes | Yes | Yes | REPCOUNT | I |
| End structure | NEXT | (auto) | } | ] | LOOP |

### Activity 3: The Same Problem in Different Paradigms
Solve "Is this number prime?" in:
- **BASIC** (imperative — step by step)
- **Pascal** (structured — with a function)
- **Prolog** (declarative — define what "prime" means)

Discuss which approach feels most natural and why.

### Activity 4: Quiz Building Competition
Teams of 2-3. Each team builds a quiz in PILOT, then other teams take it.
Score by: number of questions, variety of pattern matching, quality of feedback.

### Activity 5: Art Competition
Students create a picture using either Logo or BASIC turtle graphics.
Categories: Most geometric, most creative, most colourful, best use of procedures.

---

## Assessment Ideas

### Formative Assessment
- **Code prediction:** Show code, ask "What will this output?"
- **Bug fixing:** Give code with intentional errors to fix
- **Code completion:** Provide partial programs to complete
- **Peer review:** Students explain each other's code

### Summative Assessment

#### Beginner (BASIC/Logo/PILOT)
- Write a program that uses variables, a loop, and conditional logic
- Create a turtle drawing using at least 3 procedures
- Build a quiz with 5+ questions and scoring

#### Intermediate (Pascal/C)
- Implement a sorting algorithm
- Write functions for mathematical operations
- Create a complete calculator program

#### Advanced (Prolog/Forth)
- Build a knowledge base with recursive rules
- Implement a stack-based algorithm
- Compare solutions across multiple languages

### Project Ideas by Difficulty

| Difficulty | Project | Suggested Language |
|-----------|---------|-------------------|
| ⭐ | Digital business card | BASIC |
| ⭐ | Square and triangle art | Logo |
| ⭐ | Simple quiz (3 questions) | PILOT |
| ⭐⭐ | Times table tester | BASIC |
| ⭐⭐ | Recursive tree drawing | Logo |
| ⭐⭐ | Temperature converter | Pascal |
| ⭐⭐⭐ | Number guessing game | BASIC or C |
| ⭐⭐⭐ | Fractal generator | Logo |
| ⭐⭐⭐ | Sorting visualiser | Pascal |
| ⭐⭐⭐⭐ | Family tree reasoner | Prolog |
| ⭐⭐⭐⭐ | Stack calculator | Forth |
| ⭐⭐⭐⭐ | Multi-language comparison | All |

---

## Curriculum Alignment

### Computer Science Concepts Mapped to Languages

| Concept | Best Language(s) | Example |
|---------|-----------------|---------|
| Sequence | BASIC, PILOT | Step-by-step instructions |
| Selection | BASIC, Pascal, PILOT | IF/THEN, CASE |
| Iteration | BASIC, Pascal, C | FOR, WHILE, REPEAT |
| Variables | All | Assignment, types |
| Procedures | Pascal, Logo, PILOT | Reusable code blocks |
| Functions | BASIC, Pascal, C | Return values |
| Recursion | Logo, Prolog | Trees, fractals, ancestors |
| Arrays | BASIC, Pascal, C | Data collections |
| Pattern matching | PILOT, Prolog | M:, unification |
| Logic | Prolog | Facts, rules, inference |
| Data structures | Forth | Stack operations |
| Code organisation | Pascal | Program structure |
| Abstraction | Logo | Procedures with parameters |
| Decomposition | All | Breaking into sub-problems |
| Algorithm design | Pascal, C | Sorting, searching |

---

## Classroom Tips

1. **Pair programming** works well — one types, one navigates
2. **Start with the output** — show what the program should produce first
3. **Build incrementally** — get a small piece working, then add more
4. **Celebrate errors** — they're learning opportunities
5. **Use the examples** — the Examples/ folder has graduated difficulty
6. **Cross-reference** — compare the same concept across languages
7. **Student choice** — let advanced students pick their language
8. **Gallery walks** — display programs on screens for peer feedback

---

## Resources

- **Example Programs:** `Examples/` directory (7 language folders + demos)
- **Student Tutorials:** `tutorials/02-08` (one per language)
- **Documentation:** `docs/` directory (User Guide, Language Guide, etc.)
- **Quick References:** Each tutorial ends with a command reference table
