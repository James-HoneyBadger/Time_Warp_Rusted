/* =============================================
 * Time Warp C — Grand Demo
 * A comprehensive C language showcase
 * ============================================= */

int main() {
    int i, j, n, temp;
    int data[20];
    int fib1, fib2, fib3;
    int sum, count;
    float average;

    printf("============================================\n");
    printf("      TIME WARP C - GRAND DEMO\n");
    printf("============================================\n\n");

    /* Section 1: Data Types & Printf Formats */
    printf("--- Section 1: Data Types & Formats ---\n");
    int age = 42;
    float pi = 3.14159;
    char grade = 'A';

    printf("  Integer:   %d\n", age);
    printf("  Float:     %f\n", pi);
    printf("  Sci:       %e\n", pi);
    printf("  Compact:   %g\n", pi);
    printf("  Character: %c\n", grade);
    printf("  String:    %s\n", "Time Warp Studio");
    printf("  Hex:       0x%x\n", 255);
    printf("  Percent:   100%%\n");
    printf("  Padded:    [%10d]\n", age);
    printf("  Left:      [%-10d]\n", age);
    printf("  Precision: %.2f\n", pi);
    printf("\n");

    /* Section 2: Arithmetic & Operators */
    printf("--- Section 2: Arithmetic ---\n");
    int a = 17;
    int b = 5;
    printf("  a = %d, b = %d\n", a, b);
    printf("  a + b = %d\n", a + b);
    printf("  a - b = %d\n", a - b);
    printf("  a * b = %d\n", a * b);
    printf("  a / b = %d\n", a / b);
    printf("  a %% b = %d\n", a % b);
    printf("\n");

    /* Compound assignment */
    int x = 10;
    printf("  x = %d\n", x);
    x += 5;
    printf("  x += 5 => %d\n", x);
    x -= 3;
    printf("  x -= 3 => %d\n", x);
    x *= 2;
    printf("  x *= 2 => %d\n", x);
    x /= 4;
    printf("  x /= 4 => %d\n", x);
    x++;
    printf("  x++    => %d\n", x);
    x--;
    printf("  x--    => %d\n", x);
    printf("\n");

    /* Section 3: Control Flow */
    printf("--- Section 3: Conditionals ---\n");
    for (i = 1; i <= 10; i++) {
        if (i % 15 == 0) {
            printf("  %2d: FizzBuzz\n", i);
        } else if (i % 3 == 0) {
            printf("  %2d: Fizz\n", i);
        } else if (i % 5 == 0) {
            printf("  %2d: Buzz\n", i);
        } else {
            printf("  %2d: %d\n", i, i);
        }
    }
    printf("\n");

    /* Section 4: Loops */
    printf("--- Section 4: Loop Varieties ---\n");

    /* For loop — squares */
    printf("  Squares: ");
    for (i = 1; i <= 10; i++) {
        printf("%d ", i * i);
    }
    printf("\n");

    /* While loop — powers of 3 */
    printf("  Powers of 3: ");
    n = 1;
    while (n < 10000) {
        printf("%d ", n);
        n = n * 3;
    }
    printf("\n");

    /* Nested loops — triangle */
    printf("  Triangle:\n");
    for (i = 1; i <= 5; i++) {
        printf("    ");
        for (j = 1; j <= i; j++) {
            printf("* ");
        }
        printf("\n");
    }
    printf("\n");

    /* Section 5: Functions */
    printf("--- Section 5: Functions ---\n");

    printf("  Factorials:\n");
    for (i = 1; i <= 10; i++) {
        printf("    %2d! = %d\n", i, factorial(i));
    }
    printf("\n");

    printf("  Prime check:\n");
    for (i = 2; i <= 30; i++) {
        if (isPrime(i)) {
            printf("    %d is prime\n", i);
        }
    }
    printf("\n");

    printf("  Fibonacci:\n    ");
    for (i = 1; i <= 15; i++) {
        printf("%d ", fibonacci(i));
    }
    printf("\n\n");

    /* Section 6: Arrays */
    printf("--- Section 6: Arrays & Sorting ---\n");
    int arr[10];
    arr[0] = 64;
    arr[1] = 25;
    arr[2] = 12;
    arr[3] = 22;
    arr[4] = 11;
    arr[5] = 90;
    arr[6] = 45;
    arr[7] = 37;
    arr[8] = 78;
    arr[9] = 53;

    printf("  Before sort: ");
    for (i = 0; i < 10; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");

    /* Bubble sort */
    for (i = 0; i < 9; i++) {
        for (j = 0; j < 9 - i; j++) {
            if (arr[j] > arr[j + 1]) {
                temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }

    printf("  After sort:  ");
    for (i = 0; i < 10; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");

    /* Statistics */
    sum = 0;
    for (i = 0; i < 10; i++) {
        sum = sum + arr[i];
    }
    average = sum / 10.0;
    printf("  Sum:     %d\n", sum);
    printf("  Average: %.1f\n", average);
    printf("  Min:     %d\n", arr[0]);
    printf("  Max:     %d\n", arr[9]);
    printf("\n");

    /* Section 7: Multiplication Table */
    printf("--- Section 7: Multiplication Table ---\n");
    printf("    x |");
    for (j = 1; j <= 9; j++) {
        printf("%4d", j);
    }
    printf("\n   ---+------------------------------------\n");
    for (i = 1; i <= 9; i++) {
        printf("   %2d |", i);
        for (j = 1; j <= 9; j++) {
            printf("%4d", i * j);
        }
        printf("\n");
    }
    printf("\n");

    /* Section 8: ASCII Art Pattern */
    printf("--- Section 8: Diamond Pattern ---\n");
    n = 5;
    for (i = 1; i <= n; i++) {
        for (j = 1; j <= n - i; j++) printf(" ");
        for (j = 1; j <= 2 * i - 1; j++) printf("*");
        printf("\n");
    }
    for (i = n - 1; i >= 1; i--) {
        for (j = 1; j <= n - i; j++) printf(" ");
        for (j = 1; j <= 2 * i - 1; j++) printf("*");
        printf("\n");
    }
    printf("\n");

    /* Finale */
    printf("============================================\n");
    printf("      GRAND DEMO COMPLETE!\n");
    printf("============================================\n");
    printf("  Features demonstrated:\n");
    printf("    - Variables (int, float, char)\n");
    printf("    - Printf formats (%%d %%f %%e %%g %%c %%s %%x)\n");
    printf("    - Arithmetic & compound assignment\n");
    printf("    - Increment/decrement (++/--)\n");
    printf("    - if/else if/else\n");
    printf("    - for, while loops\n");
    printf("    - Functions (factorial, isPrime, fibonacci)\n");
    printf("    - Arrays and bubble sort\n");
    printf("    - Nested loops and patterns\n");
    printf("============================================\n");

    return 0;
}

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

int fibonacci(int n) {
    if (n <= 1) return n;
    int a = 0;
    int b = 1;
    int c;
    int i;
    for (i = 2; i <= n; i++) {
        c = a + b;
        a = b;
        b = c;
    }
    return b;
}
