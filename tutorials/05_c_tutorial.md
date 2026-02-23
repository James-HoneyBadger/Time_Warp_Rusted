# Tutorial 05 — C: Systems Programming Introduction

## Introduction

C was created by Dennis Ritchie in 1972 at Bell Labs. It became one of the
most influential languages ever, forming the foundation for Unix, Linux,
Windows, and countless other systems. Learning C teaches you how computers
really work at a low level.

**What you'll learn:**
- Program structure and `main()`
- Variables and data types
- `printf` formatting
- `scanf` for input
- Operators and expressions
- Conditionals and loops
- Functions
- Arrays

---

## Lesson 1: Hello World

```c
int main() {
    printf("Hello, World!\n");
    return 0;
}
```

**Key points:**
- Every C program starts with `int main()`
- Code goes inside curly braces `{ }`
- `printf()` prints text
- `\n` creates a new line
- Statements end with semicolons `;`
- `return 0;` means "program finished successfully"

**Challenge:** Print three lines: your name, your school, and your favourite subject.

---

## Lesson 2: Variables and Types

```c
int main() {
    int age = 14;
    float height = 1.72;
    char grade = 'A';

    printf("Age: %d\n", age);
    printf("Height: %.2f\n", height);
    printf("Grade: %c\n", grade);

    return 0;
}
```

### Types

| Type | What It Stores | Format |
|------|---------------|--------|
| `int` | Whole numbers | `%d` |
| `float` | Decimal numbers | `%f` |
| `char` | Single character | `%c` |

**Challenge:** Declare variables for a rectangle's dimensions and print its area.

---

## Lesson 3: Printf Formatting

`printf` uses format specifiers starting with `%`:

```c
int main() {
    int n = 42;
    float pi = 3.14159;

    printf("Integer:    %d\n", n);
    printf("Float:      %f\n", pi);
    printf("2 decimals: %.2f\n", pi);
    printf("Scientific: %e\n", pi);
    printf("Compact:    %g\n", pi);
    printf("Character:  %c\n", 'A');
    printf("String:     %s\n", "hello");
    printf("Hex:        %x\n", 255);
    printf("Padded:     %10d\n", n);
    printf("Left-align: %-10d!\n", n);
    printf("Literal %%:  100%%\n");

    return 0;
}
```

### Escape Sequences

| Escape | Meaning |
|--------|---------|
| `\n` | New line |
| `\t` | Tab |
| `\\` | Backslash |
| `\"` | Double quote |

**Challenge:** Print a formatted receipt with item names, quantities, and prices aligned in columns.

---

## Lesson 4: Input with scanf

```c
int main() {
    int age;
    float height;

    printf("Enter your age: ");
    scanf("%d", &age);

    printf("Enter your height: ");
    scanf("%f", &height);

    printf("You are %d years old and %.2f metres tall.\n", age, height);

    return 0;
}
```

**Important:** `scanf` uses `&` before the variable name (address-of operator).

**Challenge:** Write a calculator that reads two numbers and an operator, then prints the result.

---

## Lesson 5: Operators

### Arithmetic
```c
int a = 17, b = 5;
printf("a + b = %d\n", a + b);    // 22
printf("a - b = %d\n", a - b);    // 12
printf("a * b = %d\n", a * b);    // 85
printf("a / b = %d\n", a / b);    // 3 (integer division!)
printf("a %% b = %d\n", a % b);   // 2 (modulo/remainder)
```

### Compound Assignment
```c
int x = 10;
x += 5;    // x = x + 5  → 15
x -= 3;    // x = x - 3  → 12
x *= 2;    // x = x * 2  → 24
x /= 4;    // x = x / 4  → 6
```

### Increment/Decrement
```c
int count = 0;
count++;   // count = 1
count++;   // count = 2
count--;   // count = 1
```

### Comparison
```c
==   // equal to
!=   // not equal to
<    // less than
>    // greater than
<=   // less than or equal
>=   // greater than or equal
```

### Logical
```c
&&   // AND
||   // OR
!    // NOT
```

---

## Lesson 6: Conditionals

### if/else

```c
int score = 85;

if (score >= 90) {
    printf("Grade: A\n");
} else if (score >= 80) {
    printf("Grade: B\n");
} else if (score >= 70) {
    printf("Grade: C\n");
} else {
    printf("Grade: F\n");
}
```

### FizzBuzz Example

```c
int i;
for (i = 1; i <= 20; i++) {
    if (i % 15 == 0) {
        printf("FizzBuzz\n");
    } else if (i % 3 == 0) {
        printf("Fizz\n");
    } else if (i % 5 == 0) {
        printf("Buzz\n");
    } else {
        printf("%d\n", i);
    }
}
```

**Challenge:** Write an even/odd checker using the `%` operator.

---

## Lesson 7: Loops

### for Loop

```c
// Count 1 to 10
int i;
for (i = 1; i <= 10; i++) {
    printf("%d squared = %d\n", i, i * i);
}
```

Structure: `for (init; condition; update)`

### while Loop

```c
int n = 1;
while (n <= 1000) {
    printf("%d ", n);
    n = n * 2;
}
```

### Nested Loops

```c
// Multiplication table
int i, j;
for (i = 1; i <= 5; i++) {
    for (j = 1; j <= 5; j++) {
        printf("%4d", i * j);
    }
    printf("\n");
}
```

**Challenge:** Print a triangle pattern:
```
*
**
***
****
*****
```

---

## Lesson 8: Functions

```c
int factorial(int n) {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}

int isPrime(int n) {
    if (n < 2) return 0;
    int i;
    for (i = 2; i * i <= n; i++) {
        if (n % i == 0) return 0;
    }
    return 1;
}

int main() {
    printf("5! = %d\n", factorial(5));
    printf("7 is prime? %d\n", isPrime(7));
    printf("8 is prime? %d\n", isPrime(8));
    return 0;
}
```

**Challenge:** Write a `power(base, exp)` function that computes `base^exp` using a loop.

---

## Lesson 9: Arrays

```c
int main() {
    int scores[5];
    int i, sum;

    scores[0] = 85;
    scores[1] = 92;
    scores[2] = 78;
    scores[3] = 95;
    scores[4] = 88;

    sum = 0;
    for (i = 0; i < 5; i++) {
        sum = sum + scores[i];
    }

    printf("Average: %d\n", sum / 5);
    return 0;
}
```

**Important:** C arrays start at index 0, not 1!

### Bubble Sort

```c
int temp;
for (i = 0; i < n - 1; i++) {
    for (j = 0; j < n - 1 - i; j++) {
        if (arr[j] > arr[j + 1]) {
            temp = arr[j];
            arr[j] = arr[j + 1];
            arr[j + 1] = temp;
        }
    }
}
```

**Challenge:** Create an array of 10 numbers, sort them, and print the sorted list.

---

## Projects

### Project 1: Calculator
Read two numbers and an operator (+, -, *, /). Print the result.
Handle division by zero.

### Project 2: Prime Finder
Print all prime numbers from 2 to 1000.

### Project 3: Pattern Generator
Read a size N and print a diamond pattern of height 2N-1.

### Project 4: Statistics Calculator
Read 10 numbers into an array. Calculate and display the mean,
minimum, maximum, and range.

---

## Quick Reference

| Feature | Syntax |
|---------|--------|
| Main function | `int main() { ... return 0; }` |
| Print | `printf("format", args);` |
| Input | `scanf("%d", &var);` |
| Integer | `int x = 5;` |
| Float | `float y = 3.14;` |
| Character | `char c = 'A';` |
| If/Else | `if (cond) { } else { }` |
| For loop | `for (i = 0; i < n; i++) { }` |
| While | `while (cond) { }` |
| Function | `int name(int param) { return val; }` |
| Array | `int arr[10];` |
| Increment | `i++` or `++i` |
| Modulo | `a % b` |
