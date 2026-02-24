/* ============================================
 * 10 - Complete C Showcase
 * Demonstrates EVERY C feature in Time Warp
 * ============================================ */

#include <stdio.h>
#include <math.h>

/* --- Function definitions --- */

int factorial(int n) {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}

int is_prime(int n) {
    if (n < 2) return 0;
    for (int i = 2; i * i <= n; i++) {
        if (n % i == 0) return 0;
    }
    return 1;
}

int fibonacci(int n) {
    int a = 0, b = 1;
    for (int i = 0; i < n; i++) {
        int t = a + b;
        a = b;
        b = t;
    }
    return a;
}

int gcd(int a, int b) {
    while (b != 0) {
        int t = b;
        b = a % b;
        a = t;
    }
    return a;
}

/* --- Main program --- */

int main() {
    printf("========================================\n");
    printf("    Time Warp Rusted - C Showcase\n");
    printf("========================================\n\n");

    /* --- Variables & Types --- */
    printf("--- Variables & Types ---\n");
    int x = 42;
    float pi = 3.14159;
    char ch = 'C';
    const int MAX = 100;
    printf("int x = %d\n", x);
    printf("float pi = %f\n", pi);
    printf("char ch = %c (ASCII %d)\n", ch, ch);
    printf("const MAX = %d\n", MAX);
    printf("\n");

    /* --- Operators --- */
    printf("--- Operators ---\n");
    int a = 25, b = 7;
    printf("%d + %d = %d\n", a, b, a + b);
    printf("%d * %d = %d\n", a, b, a * b);
    printf("%d / %d = %d\n", a, b, a / b);
    printf("%d %% %d = %d\n", a, b, a % b);
    x = 10;
    x += 5;
    printf("x=10; x+=5; x=%d\n", x);
    x++;
    printf("x++; x=%d\n", x);
    x--;
    printf("x--; x=%d\n", x);
    printf("\n");

    /* --- Conditionals --- */
    printf("--- Conditionals ---\n");
    int score = 92;
    if (score >= 90) {
        printf("Score %d: Grade A\n", score);
    } else if (score >= 80) {
        printf("Score %d: Grade B\n", score);
    } else {
        printf("Score %d: Below B\n", score);
    }
    printf("\n");

    /* --- for loop --- */
    printf("--- for loop (squares) ---\n  ");
    for (int i = 1; i <= 10; i++) {
        printf("%d ", i * i);
    }
    printf("\n\n");

    /* --- while loop --- */
    printf("--- while loop (Fibonacci) ---\n  ");
    int fa = 0, fb = 1;
    while (fa < 100) {
        printf("%d ", fa);
        int t = fa + fb;
        fa = fb;
        fb = t;
    }
    printf("\n\n");

    /* --- Functions --- */
    printf("--- Functions ---\n");
    for (int i = 1; i <= 8; i++) {
        printf("  %d! = %d\n", i, factorial(i));
    }
    printf("\n");

    printf("--- Primes to 50 ---\n  ");
    for (int i = 2; i <= 50; i++) {
        if (is_prime(i)) printf("%d ", i);
    }
    printf("\n\n");

    printf("--- Fibonacci ---\n  ");
    for (int i = 0; i < 12; i++) {
        printf("%d ", fibonacci(i));
    }
    printf("\n\n");

    printf("--- GCD ---\n");
    printf("  GCD(48,18) = %d\n", gcd(48, 18));
    printf("  GCD(100,75) = %d\n", gcd(100, 75));
    printf("\n");

    /* --- Math functions --- */
    printf("--- Math Functions ---\n");
    printf("  sqrt(144) = %f\n", sqrt(144));
    printf("  pow(2,10) = %f\n", pow(2, 10));
    printf("  abs(-42) = %d\n", abs(-42));
    printf("  sin(45) = %f\n", sin(45));
    printf("  floor(7.8) = %f\n", floor(7.8));
    printf("  ceil(7.2) = %f\n", ceil(7.2));
    printf("\n");

    /* --- Nested loops (pattern) --- */
    printf("--- Pattern ---\n");
    for (int i = 1; i <= 5; i++) {
        for (int j = 0; j < i; j++) {
            printf("* ");
        }
        printf("\n");
    }
    for (int i = 4; i >= 1; i--) {
        for (int j = 0; j < i; j++) {
            printf("* ");
        }
        printf("\n");
    }
    printf("\n");

    /* --- puts and putchar --- */
    printf("--- I/O ---\n");
    puts("puts() prints a line");
    printf("putchar: ");
    putchar('H');
    putchar('i');
    putchar('!');
    printf("\n\n");

    printf("========================================\n");
    printf("    All C features demonstrated!\n");
    printf("========================================\n");

    return 0;
}
