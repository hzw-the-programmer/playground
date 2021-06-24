#include <stdio.h>

typedef struct {
    char c;
    unsigned int d;
    unsigned int m;
    unsigned int y;
} Date1;

typedef struct {
    char c;
    unsigned int d : 5;
    unsigned int m : 4;
    unsigned int y;
} Date2;

typedef struct {
    char c;
    unsigned short d : 5;
    unsigned short m : 4;
    unsigned int y;
} Date3;

typedef struct {
    unsigned short d : 5;
    unsigned short m : 4;
    unsigned int y;
    char c;
} Date4;

typedef struct {
    unsigned int y;
    unsigned short d : 5;
    unsigned short m : 4;
    char c;
} Date5;

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

void test_date1() {
    printf("%d, %d\n", sizeof(unsigned int), sizeof(Date1));
}

void test_date2() {
    Date2 d = {0};

    d.c = 1;
    d.d = 31;
    d.m = 8;
    d.y = 0x01020304;
    printf("%d, %d\n", sizeof(unsigned int), sizeof(Date2));
    print_bit((unsigned char*)&d, sizeof(Date2));
}

void test_date3() {
    Date3 d = {0};

    d.c = 1;
    d.d = 31;
    d.m = 8;
    d.y = 0x01020304;
    printf("%d, %d\n", sizeof(unsigned int), sizeof(Date3));
    print_bit((unsigned char*)&d, sizeof(Date3));
}

void test_date4() {
    Date4 d = {0};

    d.c = 1;
    d.d = 31;
    d.m = 8;
    d.y = 0x01020304;
    printf("%d, %d\n", sizeof(unsigned int), sizeof(Date4));
    print_bit((unsigned char*)&d, sizeof(Date4));
}

void test_date5() {
    Date5 d = {0};

    d.c = 1;
    d.d = 31;
    d.m = 8;
    d.y = 0x01020304;
    printf("%d, %d\n", sizeof(unsigned int), sizeof(Date5));
    print_bit((unsigned char*)&d, sizeof(Date5));
}

void test_bitfield() {
    test_date1();
    test_date2();
    test_date3();
    test_date4();
    test_date5();
}
