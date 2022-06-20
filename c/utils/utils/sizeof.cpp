#include <stdio.h> // for printf
#include <assert.h>

void f(int a[], int *b) {
    assert(sizeof(a) == 4);
    assert(sizeof(b) == 4);
}

void test_sizeof() {
    int a[100];
    int b;
    
    assert(sizeof(a) == 4 * 100);
    f(a, &b);

    assert(sizeof(char) == 1);
    assert(sizeof(short) == 2);
    assert(sizeof(int) == 4);
    assert(sizeof(long) == 4);
    assert(sizeof(long long) == 8);
    assert(sizeof(float) == 4);
    assert(sizeof(double) == 8);
}