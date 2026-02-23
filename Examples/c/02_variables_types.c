/* ============================================
 * 02 - Variables and Types
 * Learn: int, float, double, char, const
 * ============================================ */

#include <stdio.h>

int main() {
    printf("=== Variables & Types ===\n\n");

    // Integer types
    int age = 15;
    int count = 100;
    long big_number = 1000000;
    printf("int age = %d\n", age);
    printf("int count = %d\n", count);
    printf("long big = %d\n", big_number);
    printf("\n");

    // Floating point
    float pi = 3.14159;
    double precise = 3.141592653589793;
    printf("float pi = %f\n", pi);
    printf("double precise = %f\n", precise);
    printf("\n");

    // Character
    char letter = 'A';
    char digit = '7';
    printf("char letter = %c (ASCII %d)\n", letter, letter);
    printf("char digit = %c (ASCII %d)\n", digit, digit);
    printf("\n");

    // Constants
    const int MAX_SCORE = 100;
    printf("const MAX_SCORE = %d\n", MAX_SCORE);
    printf("\n");

    // Arithmetic
    int a = 25;
    int b = 7;
    printf("=== Arithmetic ===\n");
    printf("%d + %d = %d\n", a, b, a + b);
    printf("%d - %d = %d\n", a, b, a - b);
    printf("%d * %d = %d\n", a, b, a * b);
    printf("%d / %d = %d\n", a, b, a / b);

    // Compound assignment
    int x = 10;
    x += 5;
    printf("\nx = 10; x += 5; x = %d\n", x);
    x *= 2;
    printf("x *= 2; x = %d\n", x);
    x++;
    printf("x++; x = %d\n", x);

    return 0;
}
