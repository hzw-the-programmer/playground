#include <assert.h>

void a_free(void **p) {
    *p = 0;
}

#define AA_FREE(p) a_free(&(p))

void a_m(void *p) {
    int **a = p;
    //a_free(&(*a));
    //a_free(&p);
    //a_free(p);
    AA_FREE(*a);
}

void aa_m(void *p) {
    AA_FREE(p);
}

void test_pointer_1() {
    short a[100] = {0};
    short *b, *c;
    char *d, *e;

    b = a;
    c = &a[10];
    assert(c - b == 10);

    d = a;
    e = &a[10];
    assert(e - d == 20);
}

void test_pointer() {
    int a = 10;
    int *b = &a;
    int **c = &b;
    a_free(&(*c));

    b = &a;
    a_m(c);

    b = &a;
    aa_m(b);

    test_pointer_1();
}