#include <stdio.h>

void print_byte_bits(unsigned char b) {
    int i;

    for (i = 7; i >= 0; i--) {
        printf("%d", (b >> i) & 0x01);
    }
}

void print_buf_bits(unsigned char *buf, int len) {
    int i;

    for (i = 0; i < len; i++) {
        print_byte_bits(buf[i]);
        printf("\n");
    }
}

typedef struct {
    unsigned char f1;
    unsigned long long f2;
    unsigned long long f3;
    unsigned long long f4;
    unsigned short f5;
    unsigned long f6;
    unsigned short f7;
} S1;

typedef struct {
    unsigned long long f2;
    unsigned long long f3;
    unsigned long long f4;
    unsigned short f5;
    unsigned long f6;
    unsigned short f7;
    unsigned char f1;
} S2;

typedef struct {
    unsigned long long f2;
    unsigned long long f3;
    unsigned long long f4;
    unsigned short f5;
    unsigned short f7;
    unsigned long f6;
    unsigned char f1;
} S3;

int main() {
    {
        unsigned char b = 0xaa;
        print_byte_bits(b);
        printf("\n");
    }
    
    {
        printf("sizeof(short) = %d\n", sizeof(short));
        printf("sizeof(int) = %d\n", sizeof(int));
        printf("sizeof(long) = %d\n", sizeof(long));
        printf("sizeof(long long) = %d\n", sizeof(long long));
    }

    {
        S1 s = {0xff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffff, 0xffffffff, 0xffff};
        printf("sizeof(S1) = %d\n", sizeof(S1));
        print_buf_bits((unsigned char*)&s, sizeof(s));
    }

    {
        S2 s = {0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffff, 0xffffffff, 0xffff, 0xff};
        printf("sizeof(S2) = %d\n", sizeof(S2));
        print_buf_bits((unsigned char*)&s, sizeof(s));
    }

    {
        S3 s = {0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffff, 0xffff, 0xffffffff, 0xff};
        printf("sizeof(S3) = %d\n", sizeof(S3));
        print_buf_bits((unsigned char*)&s, sizeof(s));
    }
}
