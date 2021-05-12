#include <stdlib.h>
#include <stdio.h>

int min(int a, int b) {
    return a > b ? b : a;
}

int minpath(int *matrix, int m, int n) {
    int *dp = malloc(sizeof(int) * m * n);

    int i = 0, j = 0;
    dp[i * n + j] = matrix[i * n + j];
    
    j = 0;
    for (i = 1; i < m; i++) {
        dp[i * n + j] = dp[(i - 1) * n + j] + matrix[i * n + j];
    }

    i = 0;
    for (j = 1; j < n; j++) {
        dp[i * n + j] = dp[i * n + (j - 1)] + matrix[i * n + j];
    }

    for (int i = 1; i < m; i++) {
        for (int j = 1; j < n; j++) {
            dp[i * n + j] = min(dp[i * n + (j - 1)], dp[(i - 1) * n + j]) + matrix[i * n + j];
        }
    }

    int mp = dp[(m - 1) * n + (n - 1)];
    free(dp);
    return mp;
}

int main(int argc, char** argv) {
    int matrix[] = {
        1, 3, 5, 9,
        8, 1, 3, 4,
        5, 0, 6, 1,
        8, 8, 4, 0,
    };
    
    printf("%d\n", minpath(matrix, 4, 4));
}
