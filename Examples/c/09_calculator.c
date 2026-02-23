/* ============================================
 * 09 - Interactive Calculator
 * Learn: Combining input, functions, control
 * ============================================ */

#include <stdio.h>
#include <math.h>

int factorial(int n) {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}

int main() {
    printf("=========================\n");
    printf("   C Language Calculator\n");
    printf("=========================\n\n");
    printf("Operations:\n");
    printf("  1. Addition\n");
    printf("  2. Subtraction\n");
    printf("  3. Multiplication\n");
    printf("  4. Division\n");
    printf("  5. Power\n");
    printf("  6. Square Root\n");
    printf("  7. Factorial\n\n");

    int choice;
    printf("Choose (1-7): ");
    scanf("%d", &choice);

    if (choice >= 1 && choice <= 5) {
        int a, b;
        printf("First number: ");
        scanf("%d", &a);
        printf("Second number: ");
        scanf("%d", &b);
        printf("\n");

        if (choice == 1) {
            printf("%d + %d = %d\n", a, b, a + b);
        } else if (choice == 2) {
            printf("%d - %d = %d\n", a, b, a - b);
        } else if (choice == 3) {
            printf("%d * %d = %d\n", a, b, a * b);
        } else if (choice == 4) {
            if (b != 0) {
                printf("%d / %d = %f\n", a, b, a / b);
            } else {
                printf("Error: Division by zero!\n");
            }
        } else if (choice == 5) {
            printf("%d ^ %d = %f\n", a, b, pow(a, b));
        }
    } else if (choice == 6) {
        int n;
        printf("Number: ");
        scanf("%d", &n);
        printf("sqrt(%d) = %f\n", n, sqrt(n));
    } else if (choice == 7) {
        int n;
        printf("Number: ");
        scanf("%d", &n);
        printf("%d! = %d\n", n, factorial(n));
    } else {
        printf("Invalid choice!\n");
    }

    return 0;
}
