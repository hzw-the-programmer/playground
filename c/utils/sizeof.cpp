#include <stdio.h> // for printf

void f(int a[], int *b) {
    printf("%d\n", sizeof(a));
    printf("%d\n", sizeof(b));
}

void test_sizeof() {
    int a[100];
    int b;
    
    f(a, &b);
    printf("%d\n", sizeof(int));
}