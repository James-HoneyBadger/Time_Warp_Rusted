# Tutorial 01 — Getting Started with Time Warp Rusted

## What is Time Warp Rusted?

Time Warp Rusted is a retro programming environment that lets you write and
run programs in **seven classic programming languages**:

| Language | Year | What It's Great For |
|----------|------|---------------------|
| **BASIC** | 1964 | General programming, maths, graphics |
| **Logo** | 1967 | Turtle graphics, art, geometry |
| **Pascal** | 1970 | Structured programming, algorithms |
| **C** | 1972 | Systems programming, low-level control |
| **Forth** | 1970 | Stack-based computing, efficiency |
| **PILOT** | 1962 | Interactive lessons, quizzes |
| **Prolog** | 1972 | Logic, reasoning, knowledge bases |

---

## Your First Program

### Step 1: Open Time Warp Rusted

Launch the application. You will see:
- A **code editor** on the left where you type programs
- An **output panel** on the right showing results
- A **language selector** to choose your programming language
- A **Run** button to execute your code

### Step 2: Choose a Language

Click the language selector and choose **BASIC** — it's the easiest to start with.

### Step 3: Type Your First Program

In the editor, type:

```basic
PRINT "Hello, World!"
PRINT "I am learning to code!"
PRINT "This is Time Warp Rusted!"
```

### Step 4: Run It

Click **Run** (or press the keyboard shortcut). You should see:

```
Hello, World!
I am learning to code!
This is Time Warp Rusted!
```

🎉 **Congratulations!** You just wrote your first program!

---

## Try Each Language

Here's "Hello, World!" in every Time Warp language:

### BASIC
```basic
PRINT "Hello, World!"
```

### Logo
```logo
PRINT [Hello, World!]
```

### Pascal
```pascal
program Hello;
begin
  writeln('Hello, World!');
end.
```

### C
```c
int main() {
    printf("Hello, World!\n");
    return 0;
}
```

### Forth
```forth
." Hello, World!" CR
```

### PILOT
```pilot
T: Hello, World!
```

### Prolog
```prolog
?- write(hello_world).
```

---

## Understanding the Interface

### The Editor
- Type your code here
- Line numbers appear on the left
- Syntax is highlighted in colour

### The Output Panel
- Text output appears here
- Error messages appear here too
- Use this to check your program's results

### The Graphics Canvas
- When you use turtle graphics (Logo, BASIC) or pixel graphics,
  drawings appear on the canvas
- The turtle starts in the centre facing up

### The Run Button
- Executes your entire program
- If your program uses INPUT, you'll be prompted to type responses

---

## Tips for Success

1. **Start simple** — get each small piece working before adding more
2. **Read error messages** — they tell you what went wrong and where
3. **Save often** — don't lose your work!
4. **Experiment** — change things and see what happens
5. **Use the examples** — study the Examples/ folder for inspiration

---

## What's Next?

Choose a language tutorial to dive deeper:

- **New to programming?** Start with [BASIC](02_basic_tutorial.md) or [Logo](03_logo_tutorial.md)
- **Want to make art?** Try [Logo](03_logo_tutorial.md)
- **Want to build quizzes?** Try [PILOT](07_pilot_tutorial.md)
- **Ready for a challenge?** Try [Pascal](04_pascal_tutorial.md) or [C](05_c_tutorial.md)
- **Think differently?** Try [Forth](06_forth_tutorial.md) or [Prolog](08_prolog_tutorial.md)
