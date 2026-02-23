# Tutorial 04 — Pascal: Structured Programming

## Introduction

Pascal was designed by Niklaus Wirth in 1970 to teach good programming
habits. It enforces clear structure: every program has a defined beginning
and end, variables must be declared, and code is organised into procedures
and functions.

**What you'll learn:**
- Program structure
- Variables, types, and constants
- Input/output with `read`/`write`
- Conditionals (`if`/`then`/`else`)
- Loops (`for`, `while`, `repeat`)
- Procedures and functions
- Arrays and algorithms

---

## Lesson 1: Program Structure

Every Pascal program follows this pattern:

```pascal
program MyFirstProgram;
begin
  writeln('Hello, World!');
  writeln('This is Pascal!');
end.
```

**Key points:**
- `program Name;` declares the program name
- `begin`...`end.` wraps the main code (note the `.` after `end`)
- Statements end with semicolons `;`
- Strings use single quotes `'text'`

**Challenge:** Write a program that prints your name, age, and favourite food.

---

## Lesson 2: Variables and Types

Variables must be declared in a `var` section before `begin`:

```pascal
program Variables;
var
  name: string;
  age: integer;
  height: real;
  initial: char;
begin
  name := 'Alice';
  age := 14;
  height := 1.65;
  initial := 'A';

  writeln('Name: ', name);
  writeln('Age: ', age);
  writeln('Height: ', height:0:2);
  writeln('Initial: ', initial);
end.
```

### Types

| Type | What It Stores | Example |
|------|---------------|---------|
| `integer` | Whole numbers | `42`, `-7` |
| `real` | Decimal numbers | `3.14`, `-0.5` |
| `char` | Single character | `'A'`, `'5'` |
| `string` | Text | `'hello'` |
| `boolean` | True/false | `true`, `false` |

### Constants

```pascal
program UsingConstants;
const
  PI = 3.14159;
  MAX_SCORE = 100;
var
  radius: integer;
  area: real;
begin
  radius := 10;
  area := PI * radius * radius;
  writeln('Area: ', area:0:2);
  writeln('Max score: ', MAX_SCORE);
end.
```

**Challenge:** Declare variables for a rectangle's length and width. Calculate and print its area, perimeter, and diagonal.

---

## Lesson 3: Input and Output

### Output Formatting

```pascal
writeln('Text');           { Print with newline }
write('No newline');       { Print without newline }
writeln(42);               { Print integer }
writeln(3.14:0:2);         { Print real with 2 decimals }
writeln(name:20);          { Right-align in 20 chars }
```

### Reading Input

```pascal
program GetInput;
var
  name: string;
  age: integer;
begin
  write('Name: ');
  readln(name);
  write('Age: ');
  readln(age);
  writeln('Hello, ', name, '! You are ', age, '.');
end.
```

**Challenge:** Create a temperature converter that reads Celsius and prints Fahrenheit (`F = C × 9/5 + 32`).

---

## Lesson 4: Conditionals

### IF/THEN/ELSE

```pascal
program Grading;
var
  score: integer;
begin
  write('Enter score: ');
  readln(score);

  if score >= 90 then
    writeln('Grade: A')
  else if score >= 80 then
    writeln('Grade: B')
  else if score >= 70 then
    writeln('Grade: C')
  else
    writeln('Grade: F');
end.
```

### Compound Statements

Use `begin`...`end` for multiple statements in a branch:

```pascal
if score >= 90 then
begin
  writeln('Excellent!');
  writeln('Grade: A');
end
else
begin
  writeln('Keep trying!');
  writeln('Grade: below A');
end;
```

### Boolean Operators

```pascal
if (age >= 13) and (age <= 19) then
  writeln('Teenager');

if (day = 'Sat') or (day = 'Sun') then
  writeln('Weekend!');

if not isPassing then
  writeln('Need more study');
```

**Challenge:** Write a leap year checker. A year is a leap year if it's divisible by 4, except centuries must be divisible by 400.

---

## Lesson 5: Loops

### FOR Loop

```pascal
{ Count from 1 to 10 }
for i := 1 to 10 do
  writeln(i, ' squared = ', i * i);

{ Count backwards }
for i := 10 downto 1 do
  write(i, '... ');
writeln('Blast off!');
```

### WHILE Loop

```pascal
{ Powers of 2 }
n := 1;
while n <= 1000 do
begin
  writeln(n);
  n := n * 2;
end;
```

### REPEAT/UNTIL Loop

Executes at least once, then checks the condition:

```pascal
repeat
  write('Enter a positive number: ');
  readln(n);
until n > 0;
writeln('You entered: ', n);
```

### Nested Loops

```pascal
{ Multiplication table }
for i := 1 to 9 do
begin
  for j := 1 to 9 do
    write(i * j:4);
  writeln;
end;
```

**Challenge:** Print a triangle pattern of stars using nested loops.

---

## Lesson 6: Procedures and Functions

### Procedures

```pascal
procedure PrintBanner(title: string);
begin
  writeln('========================');
  writeln('  ', title);
  writeln('========================');
end;

{ In main: }
PrintBanner('Welcome!');
PrintBanner('Results');
```

### Functions

Functions return a value:

```pascal
function Factorial(n: integer): integer;
begin
  if n <= 1 then
    Factorial := 1
  else
    Factorial := n * Factorial(n - 1);
end;

{ In main: }
writeln('5! = ', Factorial(5));
writeln('10! = ', Factorial(10));
```

### More Examples

```pascal
function IsPrime(n: integer): boolean;
var
  k: integer;
begin
  if n < 2 then begin IsPrime := false; exit; end;
  IsPrime := true;
  for k := 2 to n div 2 do
    if n mod k = 0 then begin IsPrime := false; exit; end;
end;

function Max(a, b: integer): integer;
begin
  if a > b then Max := a else Max := b;
end;
```

**Challenge:** Write a function `GCD(a, b)` that computes the Greatest Common Divisor using Euclid's algorithm.

---

## Lesson 7: Arrays

```pascal
program ArrayDemo;
var
  grades: array[1..5] of integer;
  i, total: integer;
begin
  grades[1] := 85;
  grades[2] := 92;
  grades[3] := 78;
  grades[4] := 95;
  grades[5] := 88;

  total := 0;
  for i := 1 to 5 do
    total := total + grades[i];

  writeln('Average: ', total div 5);
end.
```

### Bubble Sort

```pascal
{ Sort array in ascending order }
for i := 1 to n - 1 do
  for j := 1 to n - i do
    if data[j] > data[j + 1] then
    begin
      temp := data[j];
      data[j] := data[j + 1];
      data[j + 1] := temp;
    end;
```

**Challenge:** Create an array of 10 numbers, find the minimum, maximum, and average.

---

## Projects

### Project 1: Prime Number Generator
Print all prime numbers from 2 to 1000 using a function and a loop.

### Project 2: Student Gradebook
Store 10 student scores in an array, sort them, calculate statistics,
and assign letter grades.

### Project 3: Fibonacci Calculator
Write a function to compute the Nth Fibonacci number. Print the first 25.

### Project 4: Number Guessing Game
The computer picks a number 1–100, the user guesses with hints.

---

## Quick Reference

| Feature | Syntax |
|---------|--------|
| Program | `program Name; begin ... end.` |
| Variable | `var x: integer;` |
| Constant | `const PI = 3.14;` |
| Assignment | `x := 5;` |
| Output | `writeln('text', value);` |
| Input | `readln(variable);` |
| If/Else | `if cond then ... else ...;` |
| For loop | `for i := 1 to 10 do ...;` |
| While | `while cond do begin ... end;` |
| Repeat | `repeat ... until cond;` |
| Procedure | `procedure Name(params);` |
| Function | `function Name(p): type;` |
| Array | `array[1..10] of integer` |
