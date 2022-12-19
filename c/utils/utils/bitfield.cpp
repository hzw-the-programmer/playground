#include <stdio.h>
#include <assert.h>

void print_bit(unsigned char *buf, int len) {
    int i, j;

    for (i = 0; i < len; i++) {
        j = 7;
        while (j >= 0) {
            printf("%d", buf[i] & (1 << j--) ? 1 : 0);
        }
        printf("\n");
    }
}

typedef struct {
    char c;
    unsigned int d;
    unsigned int m;
    unsigned int y;
} Date1;

void test_date1() {
    printf("test_date1\n");
    printf("%d, %d\n", sizeof(unsigned int), sizeof(Date1));
    
    assert(sizeof(Date1) == 16);
}

typedef struct {
    char c;
    unsigned int d : 5;
    unsigned int m : 4;
    unsigned int y;
} Date2;

void test_date2() {
    Date2 d = {0};

    printf("test_date2\n");

    d.c = 1;
    d.d = 31;
    d.m = 8;
    d.y = 0x01020304;
    printf("%d, %d\n", sizeof(unsigned int), sizeof(Date2));
    print_bit((unsigned char*)&d, sizeof(Date2));
}

typedef struct {
    char c;
    unsigned short d : 5;
    unsigned short m : 4;
    unsigned int y;
} Date3;

void test_date3() {
    Date3 d = {0};

    printf("test_date3\n");

    d.c = 1;
    d.d = 31;
    d.m = 8;
    d.y = 0x01020304;
    printf("%d, %d\n", sizeof(unsigned int), sizeof(Date3));
    print_bit((unsigned char*)&d, sizeof(Date3));
}

typedef struct {
    unsigned short d : 5;
    unsigned short m : 4;
    unsigned int y;
    char c;
} Date4;

void test_date4() {
    Date4 d = {0};

    printf("test_date4\n");

    d.c = 1;
    d.d = 31;
    d.m = 8;
    d.y = 0x01020304;
    printf("%d, %d\n", sizeof(unsigned int), sizeof(Date4));
    print_bit((unsigned char*)&d, sizeof(Date4));
}

typedef struct {
    unsigned int y;
    unsigned short d : 5;
    unsigned short m : 4;
    char c;
} Date5;

void test_date5() {
    Date5 d = {0};

    printf("test_date5\n");

    d.c = 1;
    d.d = 31;
    d.m = 8;
    d.y = 0x01020304;
    printf("%d, %d\n", sizeof(unsigned int), sizeof(Date5));
    print_bit((unsigned char*)&d, sizeof(Date5));
}

typedef struct {
    int f1 : 1;
    int f2 : 31;
} Test6;

void test6() {
    Test6 t = {0};

    printf("test6\n");

    t.f1 = 1;
    t.f2 = 0xcf020304;
    
    printf("%d, %d\n", sizeof(unsigned int), sizeof(Test6));
    print_bit((unsigned char*)&t, sizeof(Test6));
}

typedef struct {
    int f1 : 16;
    int f2 : 17;
} Test7;

void test7() {
    Test7 t = {0};

    printf("test7\n");

    t.f1 = 1;
    t.f2 = 0xffff0304;
    
    printf("%d, %d\n", sizeof(unsigned int), sizeof(Test7));
    print_bit((unsigned char*)&t, sizeof(Test7));
}

typedef struct {
    int f1 : 1;
    char f2 : 2;
} Test8;

void test8() {
    Test8 t = {0};

    printf("test8\n");

    t.f1 = 1;
    t.f2 = 5;
    
    printf("%d, %d\n", sizeof(unsigned int), sizeof(Test8));
    print_bit((unsigned char*)&t, sizeof(Test8));
}

typedef struct {
    int f1 : 1;
    int f2 : 2;
} Test9;

void test9() {
    Test9 t = {0};

    printf("test9\n");

    t.f1 = 1;
    t.f2 = 5;
    
    printf("%d, %d\n", sizeof(unsigned int), sizeof(Test9));
    print_bit((unsigned char*)&t, sizeof(Test9));
}

typedef struct {
    int f1 : 1;
    int f2 : 1;
    int f3 : 2;
    int f4 : 1;
    int f5 : 3;
    int f6 : 1;
    int f7 : 4;
    int f8 : 1;
    int f9 : 5;
} Test10;

void test10() {
    Test10 t = {0};

    printf("test10\n");

    t.f1 = 1;
    t.f3 = 3;
    t.f5 = 7;
    t.f7 = 15;
    t.f9 = 31;
    
    printf("%d, %d\n", sizeof(unsigned int), sizeof(Test9));
    print_bit((unsigned char*)&t, sizeof(Test9));
}

void test_bitfield() {
    printf("test_bitfield\n");
    test_date1();
    test_date2();
    test_date3();
    test_date4();
    test_date5();
    test6();
    test7();
    test8();
    test9();
    test10();
}
