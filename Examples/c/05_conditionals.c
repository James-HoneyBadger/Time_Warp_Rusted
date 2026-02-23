/* ============================================
 * 05 - Conditionals
 * Learn: if, else if, else, comparisons
 * ============================================ */

#include <stdio.h>

int main() {
    printf("=== Conditionals ===\n\n");

    // Simple if/else
    int score = 85;
    printf("Score: %d\n", score);
    if (score >= 90) {
        printf("Grade: A - Excellent!\n");
    } else if (score >= 80) {
        printf("Grade: B - Great work!\n");
    } else if (score >= 70) {
        printf("Grade: C - Good job\n");
    } else if (score >= 60) {
        printf("Grade: D - Needs improvement\n");
    } else {
        printf("Grade: F - See teacher\n");
    }
    printf("\n");

    // Nested conditions
    int age = 16;
    int has_permit = 1;
    printf("Age: %d, Has permit: %d\n", age, has_permit);
    if (age >= 16) {
        printf("Old enough to drive.\n");
        if (has_permit) {
            printf("And you have a permit - great!\n");
        } else {
            printf("But get your permit first!\n");
        }
    } else {
        printf("Too young to drive.\n");
    }
    printf("\n");

    // Comparison operators
    int x = 15;
    printf("x = %d\n", x);
    if (x > 10 && x < 20) {
        printf("x is between 10 and 20\n");
    }
    if (x == 15 || x == 30) {
        printf("x is 15 or 30\n");
    }
    if (!(x > 20)) {
        printf("x is NOT greater than 20\n");
    }

    return 0;
}
