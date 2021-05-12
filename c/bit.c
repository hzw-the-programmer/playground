#include <stdio.h>
#include <string.h>

//error: width of ‘y’ exceeds its type
/*
struct test
{
   unsigned int x;
   unsigned int y: 33;
   unsigned int z;
};
*/

#if 0

struct test
{
   unsigned int x;
   long int y;
   unsigned int z;
};

int main(int argc, char *argv[]) {
    printf("%lu\n", sizeof(long int));
    printf("%lu\n", sizeof(struct test));

    struct test t;
    unsigned int *ptr1 = &t.x;
    unsigned int *ptr2 = &t.z;
    printf("%lu\n", ptr2 - ptr1);

    printf("%p\n", ptr1);
    printf("%p\n", ptr2);

    long int n[] = {0x0102030405060708, 0x1112131415161718, 0x2122232425262728};
    memcpy(&t, n, 24);
    printf("%016x\n", t.x);
    printf("%016lx\n", t.y);
    printf("%016x\n", t.z);

    return 0;
}

#endif

#if 0

struct test {
    int a : 4;
    int b : 4;
};

int main(int argc, char *argv[]) {
    printf("%lu\n", sizeof(struct test));
    struct test t;
    int c = 0xFFFFFF12;
    memcpy(&t, &c, 4);
    printf("%d\n", t.a);
    printf("%d\n", t.b);
}

#endif

#if 0

struct test {
    int a : 1;
    int b : 2;
    int c : 3;
    int d : 2;
};

int main(int argc, char *argv[]) {
    printf("%lu\n", sizeof(struct test));
    struct test t;
    int c = 0x7FFFFF1d;
    //int c = 0xFFFFFF1a;
    memcpy(&t, &c, 4);
    printf("%d\n", t.a);
    printf("%d\n", t.b);
    printf("%d\n", t.c);
    printf("%d\n", t.d);
}

#endif

struct test {
    unsigned int a : 1;
    unsigned int b : 2;
    unsigned int c : 3;
    unsigned int d : 2;
};

int main(int argc, char *argv[]) {
    printf("%lu\n", sizeof(struct test));
    struct test t;
    int c = 0x7FFFFF1d;
    //int c = 0xFFFFFF1a;
    memcpy(&t, &c, 4);
    printf("%d\n", t.a);
    printf("%d\n", t.b);
    printf("%d\n", t.c);
    printf("%d\n", t.d);
}
