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

void test_pointer() {
    int a = 10;
    int *b = &a;
    int **c = &b;
    a_free(&(*c));

    b = &a;
    a_m(c);

    b = &a;
    aa_m(b);
}