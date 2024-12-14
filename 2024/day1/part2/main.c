#include <stdio.h>
#include <stdlib.h>

#define COLUMNS 2

int compare(const void *a, const void *b) {
    return (*(int *)a - *(int *)b);
}

int main() {
    FILE *file;
    char buffer[256];
    int rows = 0;
    int *first;
    int *second;

    file = fopen("input.txt", "r");
    if (file == NULL) {
        perror("Error opening file");
        return EXIT_FAILURE;
    }

    while (fgets(buffer, sizeof(buffer), file) != NULL) {
        rows++;
    }
    rewind(file);

    first = malloc(rows * sizeof(int *));
    second = malloc(rows * sizeof(int *));

    for (int i = 0; i < rows; i++) {
        fscanf(file, "%d", &first[i]);
        fscanf(file, "%d", &second[i]);
    }

    fclose(file);

    qsort(first, rows, sizeof(int), compare);
    qsort(second, rows, sizeof(int), compare);


    int sum = 0;
    for (int i = 0; i < rows; i++) {
        int count = 0;
        for (int j = 0; j < rows; j++) {
            if (second[j] == first[i]) {
                count++;
            }
        }
        sum += first[i] * count;
    }

    printf("sum: %d\n", sum);

    return EXIT_SUCCESS;
}
