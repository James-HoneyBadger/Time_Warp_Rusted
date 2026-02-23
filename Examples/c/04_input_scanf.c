/* ============================================
 * 04 - Input and Scanf
 * Learn: scanf, gets, user interaction
 * ============================================ */

#include <stdio.h>

int main() {
    printf("=== Interactive Input ===\n\n");

    // Integer input
    int num;
    printf("Enter a number: ");
    scanf("%d", &num);
    printf("You entered: %d\n", num);
    printf("Doubled: %d\n", num * 2);
    printf("Squared: %d\n", num * num);
    printf("\n");

    // Second number for calculation
    int num2;
    printf("Enter another number: ");
    scanf("%d", &num2);
    printf("\n");
    printf("=== Results ===\n");
    printf("%d + %d = %d\n", num, num2, num + num2);
    printf("%d - %d = %d\n", num, num2, num - num2);
    printf("%d * %d = %d\n", num, num2, num * num2);
    if (num2 != 0) {
        printf("%d / %d = %d\n", num, num2, num / num2);
    } else {
        printf("Cannot divide by zero!\n");
    }

    printf("\nThank you!\n");
    return 0;
}
