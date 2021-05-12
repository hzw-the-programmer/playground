#include <stdio.h>
#include <math.h>

int isPrime(int num) {
    if (num < 2) return 0;
    if (num % 2 == 0) return num == 2;
    if (num % 3 == 0) return num == 3;
    for (int i = 5; i <= sqrt(num); i++) {
        if (num % i == 0) return 0;
    }
    return 1;
}

int main(int argc, char* argv[]) {
    int m, n;

    scanf("%d %d", &m, &n);

    int num = 0, c = 0;
    while (1) {
        if (isPrime(num)) {
            c++;
            if (c >= m) {
                printf("%d", num);
                if ((c - m + 1) % 10 != 0) {
                    if (c == n) {
                        break;
                    } else {
                        printf(" ");    
                    }
                } else {
                    printf("\n");
                }
            }
        }
        num++;
    }
}
