/* ============================================
 * 03 - Printf Format Specifiers
 * Learn: %d, %f, %e, %g, %c, %s, %%, escapes
 * ============================================ */

#include <stdio.h>

int main() {
    printf("=== Printf Format Specifiers ===\n\n");

    // Integer formatting
    int n = 42;
    printf("%%d  (decimal):    %d\n", n);
    printf("%%i  (integer):    %i\n", n);
    printf("\n");

    // Float formatting
    double pi = 3.14159265;
    printf("%%f  (float):      %f\n", pi);
    printf("%%e  (scientific): %e\n", pi);
    printf("%%g  (auto):       %g\n", pi);
    printf("\n");

    // Character and string
    char ch = 'X';
    printf("%%c  (char):       %c\n", ch);
    printf("%%s  (string):     %s\n", "Hello");
    printf("\n");

    // Literal percent
    printf("100%%  (literal percent sign)\n");
    printf("\n");

    // Escape sequences
    printf("=== Escape Sequences ===\n");
    printf("Newline: Line1\nLine2\n");
    printf("Tab:\tColumn1\tColumn2\n");
    printf("Backslash: \\\n");
    printf("Quote: \"\n");
    printf("\n");

    // Multiple values
    int x = 10, y = 20;
    printf("Coordinates: (%d, %d)\n", x, y);
    printf("Sum: %d + %d = %d\n", x, y, x + y);
    printf("\n");

    // puts and putchar
    puts("puts() adds a newline automatically");
    printf("Characters: ");
    putchar('H');
    putchar('i');
    putchar('!');
    printf("\n");

    return 0;
}
