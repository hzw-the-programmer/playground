#include <stdio.h>
#include <string.h>

typedef struct _s1 {
    char a;
} s1;

typedef struct _s2 {
    short a;
} s2;

typedef struct _s3 {
    short a;
    char b;
} s3;

typedef struct _s4 {
    short a;
    char b;
    char c;
} s4;

typedef struct _s5 {
    short a;
    short b;
    char c;
} s5;

typedef struct _s6 {
    short a;
    int b;
} s6;

typedef struct _s7 {
    s3 a;
    char b;
} s7;

typedef struct _s8 {
    short a;
    int b;
    short c;
} s8;

typedef struct _s9 {
    short a;
    short c;
    int b;
} s9;

typedef struct _s10 {
    short a;
    short c;
    int b;
    short d;
} s10;

typedef struct _s11 {
    short a;
    short c;
    short d;
} s11;

typedef struct _s12 {
    char a;
    short c;
} s12;

typedef struct _s13 {
    char a;
    short c;
    char d;
} s13;

typedef struct _s14 {
    char a;
    int c;
    char d;
} s14;

typedef struct _s15 {
    char a;
    long c;
    char d;
} s15;

typedef struct _s16 {
    char a;
    s14 b;
    char d;
} s16;

typedef struct _s17 {
    char a;
    s15 b;
    char d;
} s17;

int main(int argc, char *argv[]) {
    printf("short: %lu\n", sizeof(short));
    printf("s1: %lu\n", sizeof(s1));
    printf("s2: %lu\n", sizeof(s2));
    printf("s3: %lu\n", sizeof(s3));
    printf("s4: %lu\n", sizeof(s4));
    printf("s5: %lu\n", sizeof(s5));
    printf("s6: %lu\n", sizeof(s6));
    printf("s7: %lu\n", sizeof(s7));
    printf("s8: %lu\n", sizeof(s8));
    printf("s9: %lu\n", sizeof(s9));
    printf("s10: %lu\n", sizeof(s10));
    printf("s11: %lu\n", sizeof(s11));
    printf("s12: %lu\n", sizeof(s12));
    printf("s13: %lu\n", sizeof(s13));
    printf("s14: %lu\n", sizeof(s14));
    printf("s15: %lu\n", sizeof(s15));
    printf("s16: %lu\n", sizeof(s16));
    printf("s17: %lu\n", sizeof(s17));

    s3 a;
    //memset(&a, 0x0F, sizeof(s3));
    *(char*)&a = 0x01;
    *((char*)&a + 1) = 0x02;
    *((char*)&a + 2) = 0x03;
    *((char*)&a + 3) = 0x04;
    printf("%04x\n", a.a);
    printf("%02x\n", a.b);

    s7 b;
    *(char*)&b = 0x00;
    *((char*)&b + 1) = 0x01;
    *((char*)&b + 2) = 0x02;
    *((char*)&b + 3) = 0x03;
    *((char*)&b + 4) = 0x04;
    *((char*)&b + 5) = 0x05;
    printf("%04x\n", b.a.a);
    printf("%02x\n", b.a.b);
    printf("%02x\n", b.b);

    printf("\n");
    printf("s6:\n");
    s6 c;
    *(char*)&c = 0x00;
    *((char*)&c + 1) = 0x01;
    *((char*)&c + 2) = 0x02;
    *((char*)&c + 3) = 0x03;
    *((char*)&c + 4) = 0x04;
    *((char*)&c + 5) = 0x05;
    *((char*)&c + 6) = 0x06;
    *((char*)&c + 7) = 0x07;
    printf("%04x\n", c.a);
    printf("%08x\n", c.b);

    printf("\n");
    printf("s17:\n");
    s17 d;
    char *p = (char*)&d;
    char v = 0x00;
    for (int i = 0; i < sizeof(s17); i++) {
        *p++ = v++;
    }
    printf("%02x\n", d.a);
    printf("%02x\n", d.b.a);
    printf("%016lx\n", d.b.c);
    printf("%02x\n", d.b.d);
    printf("%02x\n", d.d);
}
