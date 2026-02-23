/* ============================================
 * 06 - Loops
 * Learn: for, while, do-while, nested loops
 * ============================================ */

#include <stdio.h>

int main() {
    printf("=== Loop Structures ===\n\n");

    // For loop
    printf("--- for loop (1 to 10) ---\n");
    for (int i = 1; i <= 10; i++) {
        printf("%d ", i);
    }
    printf("\n\n");

    // For loop with step
    printf("--- Counting by 3s ---\n");
    for (int i = 0; i <= 30; i += 3) {
        printf("%d ", i);
    }
    printf("\n\n");

    // Countdown
    printf("--- Countdown ---\n");
    for (int i = 10; i >= 1; i--) {
        printf("%d...", i);
    }
    printf("Liftoff!\n\n");

    // While loop
    printf("--- while loop (Fibonacci) ---\n");
    int a = 0, b = 1;
    while (a < 100) {
        printf("%d ", a);
        int temp = a + b;
        a = b;
        b = temp;
    }
    printf("\n\n");

    // Nested for loops - pattern
    printf("--- Nested loops (triangle) ---\n");
    for (int i = 1; i <= 5; i++) {
        for (int j = 1; j <= i; j++) {
            printf("* ");
        }
        printf("\n");
    }
    printf("\n");

    // Multiplication table
    printf("--- Multiplication Table ---\n");
    printf("     ");
    for (int j = 1; j <= 5; j++) {
        printf("%3d ", j);
    }
    printf("\n    ----------------\n");
    for (int i = 1; i <= 5; i++) {
        printf("%d | ", i);
        for (int j = 1; j <= 5; j++) {
            printf("%3d ", i * j);
        }
        printf("\n");
    }
    printf("\n");

    // Sum calculation
    int sum = 0;
    for (int i = 1; i <= 100; i++) {
        sum += i;
    }
    printf("Sum of 1 to 100 = %d\n", sum);

    return 0;
}
