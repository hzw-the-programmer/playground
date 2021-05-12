#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <assert.h>

int primes[] = {
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
    73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173,
    179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
    283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409,
    419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541,
    547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
    661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809,
    811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941
};

#if 1

int main(int argc, char *argv[]) {
    if (argc != 2) {
        printf("Usage: %s num\n", argv[0]);
        exit(1);
    }

    int num = atoi(argv[1]);
    int bytes = num / 8;
    if (num % 8 != 0) {
        bytes++;
    }
    char *a = malloc(bytes);
    if (a == NULL) {
        exit(1);
    }
    memset(a, 0xFF, bytes);

    int i, j;

    /*
    for (i = 2; i < num; i++) {
        if (a[i / 8] & (1 << i % 8)) {
            printf("%4d ", i);
        }
    }
    printf("\n");
    */

    for (i = 2; i < sqrt(num); i++) {
        if (a[i / 8] & (1 << i % 8)) {
            for (j = i; i * j < num; j++) {
                a[i * j / 8] &= ~(1 << i * j % 8);
            }
        }
    }

    for (i = 2; i < num; i++) {
        if (a[i / 8] & (1 << i % 8)) {
            printf("%4d ", i);
        }
    }
    printf("\n");

    j = 0;
    for (i = 2; i < num; i++) {
        if (a[i / 8] & (1 << i % 8)) {
            assert(primes[j++] == i);
        }
    }

    free(a);
    a = NULL;
}

#else

int main(int argc, char *argv[]) {
    //printf("%d\n", EXIT_FAILURE);
    if (argc != 2) {
        printf("Usage: %s num\n", argv[0]);
        exit(1);
    }

    int num = atoi(argv[1]);
    int *a = malloc(num * sizeof(int));
    
    //memset(a, 1, num * sizeof(int));
    
    int i;
    for (i = 2; i < num; i++) {
        a[i] = 1;
    }
    
    /*
    for (i = 0; i < num; i++) {
        printf("%08x ", a[i]);
    }
    printf("\n");
    */
    
    int j;
    for (i = 2; i < sqrt(num); i++) {
        if (a[i]) {
            for (j = i; i * j < num; j++) {
                a[i * j] = 0;
            }
        }
    }

    for (i = 2; i < num; i++) {
        if (a[i]) {
            printf("%4d ", i);
        }
    }
    printf("\n");

    j = 0;
    for (i = 2; i < num; i++) {
        if (a[i]) {
            assert(primes[j++] == i);
        }
    }
}

#endif

//gcc sieve_of_eratosthenes.c -lm
