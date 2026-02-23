/* ============================================
 * 07 - Functions
 * Learn: function definitions, parameters,
 *        return values, math functions
 * ============================================ */

#include <stdio.h>
#include <math.h>

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

int power(int base, int exp) {
    int result = 1;
    for (int i = 0; i < exp; i++) {
        result *= base;
    }
    return result;
}

int fibonacci(int n) {
    if (n <= 0) return 0;
    if (n == 1) return 1;
    int a = 0, b = 1;
    for (int i = 2; i <= n; i++) {
        int temp = a + b;
        a = b;
        b = temp;
    }
    return b;
}

int main() {
    printf("=== Functions ===\n\n");

    // Factorials
    printf("--- Factorials ---\n");
    for (int i = 1; i <= 10; i++) {
        printf("  %d! = %d\n", i, factorial(i));
    }
    printf("\n");

    // Prime numbers
    printf("--- Primes up to 50 ---\n  ");
    for (int i = 2; i <= 50; i++) {
        if (is_prime(i)) {
            printf("%d ", i);
        }
    }
    printf("\n\n");

    // Powers
    printf("--- Powers of 2 ---\n");
    for (int i = 0; i <= 10; i++) {
        printf("  2^%d = %d\n", i, power(2, i));
    }
    printf("\n");

    // Fibonacci
    printf("--- Fibonacci ---\n  ");
    for (int i = 0; i <= 15; i++) {
        printf("%d ", fibonacci(i));
    }
    printf("\n\n");

    // Built-in math functions
    printf("--- Math Functions ---\n");
    printf("  sqrt(144) = %f\n", sqrt(144));
    printf("  abs(-42) = %d\n", abs(-42));
    printf("  pow(2, 8) = %f\n", pow(2, 8));
    printf("  sin(45) = %f\n", sin(45));
    printf("  cos(45) = %f\n", cos(45));
    printf("  floor(7.8) = %f\n", floor(7.8));
    printf("  ceil(7.2) = %f\n", ceil(7.2));

    return 0;
}
