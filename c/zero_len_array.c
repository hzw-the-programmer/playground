#include <stdio.h>
#include <stdlib.h>

#if 1
struct test1 {
    long a;
    short b;
};

struct test2 {
    short a;
    long b;
};

struct test3 {
    long a;
    short b;
    char c[];
};

struct test4 {
    short a;
    long b;
    char c[];
};

struct test5 {
    long a;
    char b;
    char c[];
};
#else
struct test1 {
    int a;
    short b;
};

struct test2 {
    short a;
    int b;
};

struct test3 {
    int a;
    short b;
    char c[];
};

struct test4 {
    short a;
    int b;
    char c[];
};

struct test5 {
    int a;
    char b;
    char c[];
};
#endif

int main(int argc, char *argv[]) {
    printf("%lu\n", sizeof(struct test1));
    struct test1 *t1 = malloc(sizeof(struct test1) + 10);
    printf("%p\n", t1);
    printf("%p\n", t1 + 1);

    printf("%lu\n", sizeof(struct test2));
    struct test2 *t2 = malloc(sizeof(struct test2) + 10);
    printf("%p\n", t2);
    printf("%p\n", t2 + 1);

    printf("%lu\n", sizeof(struct test3));
    struct test3 *t3 = malloc(sizeof(struct test3) + 10);
    printf("%p\n", t3);
    printf("%p\n", t3 + 1);
    printf("%p\n", t3->c);

    printf("%lu\n", sizeof(struct test4));
    struct test4 *t4 = malloc(sizeof(struct test4) + 10);
    printf("%p\n", t4);
    printf("%p\n", t4 + 1);
    printf("%p\n", t4->c);

    printf("%lu\n", sizeof(struct test5));
    struct test5 *t5 = malloc(sizeof(struct test5) + 10);
    printf("%p\n", t5);
    printf("%p\n", t5 + 1);
    printf("%p\n", t5->c);
}
