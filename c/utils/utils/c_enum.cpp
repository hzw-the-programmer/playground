#include <stdio.h> // printf

typedef enum {
    EINT8_1 = -1,
    EINT8_2 = 0x7f,
} EINT8;

typedef enum {
    EUINT8_1 = 0xff,
} EUINT8;

typedef enum {
    EINT16_1 = -1,
    EINT16_2 = 0x7fff,
} EINT16;

typedef enum {
    EUINT16_1 = 0xffff,
} EUINT16;

typedef enum {
    EINT24_1 = -1,
    EINT24_2 = 0x7fffff,
} EINT24;

typedef enum {
    EUINT24_1 = 0xffffff,
} EUINT24;

typedef enum {
    EINT32_1 = -1,
    EINT32_2 = 0x7fffffff,
} EINT32;

typedef enum {
    EUINT32_1 = 0xffffffff,
} EUINT32;

void test_c_enum() {
    printf("EINT8=%d\n", sizeof(EINT8));
    printf("EUINT8=%d\n", sizeof(EUINT8));
    printf("EINT16=%d\n", sizeof(EINT16));
    printf("EUINT16=%d\n", sizeof(EUINT16));
    printf("EINT24=%d\n", sizeof(EINT24));
    printf("EUINT24=%d\n", sizeof(EUINT24));
    printf("EINT32=%d\n", sizeof(EINT32));
    printf("EUINT32=%d\n", sizeof(EUINT32));
}
