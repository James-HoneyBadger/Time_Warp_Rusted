/* ============================================
 * 08 - Arrays
 * Learn: Array declaration, initialization,
 *        algorithms, sorting
 * ============================================ */

#include <stdio.h>

int main() {
    printf("=== Arrays ===\n\n");

    // Array initialization
    int data[10];
    data[0] = 64; data[1] = 34; data[2] = 25;
    data[3] = 12; data[4] = 22; data[5] = 11;
    data[6] = 90; data[7] = 45; data[8] = 78;
    data[9] = 56;

    // Print array
    printf("Original: ");
    for (int i = 0; i < 10; i++) {
        printf("%d ", data[i]);
    }
    printf("\n\n");

    // Statistics
    int sum = 0;
    int min = data[0];
    int max = data[0];
    for (int i = 0; i < 10; i++) {
        sum += data[i];
        if (data[i] < min) min = data[i];
        if (data[i] > max) max = data[i];
    }
    printf("Sum: %d\n", sum);
    printf("Average: %d\n", sum / 10);
    printf("Min: %d\n", min);
    printf("Max: %d\n", max);
    printf("\n");

    // Bubble sort
    for (int i = 0; i < 9; i++) {
        for (int j = 0; j < 9 - i; j++) {
            if (data[j] > data[j + 1]) {
                int temp = data[j];
                data[j] = data[j + 1];
                data[j + 1] = temp;
            }
        }
    }

    printf("Sorted: ");
    for (int i = 0; i < 10; i++) {
        printf("%d ", data[i]);
    }
    printf("\n\n");

    // Linear search
    int target = 45;
    printf("Searching for %d...\n", target);
    for (int i = 0; i < 10; i++) {
        if (data[i] == target) {
            printf("Found at index %d\n", i);
        }
    }

    return 0;
}
