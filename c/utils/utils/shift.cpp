#include <assert.h>
#include <stdint.h>
#include <stdio.h>

/*
The result of E1 >> E2 is E1 right-shifted E2 bit positions.
If E1 has an unsigned type or if E1 has a signed type and a nonnegative value, the value of the result is the integral part of the quotient of E12E2.
If E1 has a signed type and a negative value, the resulting value is implementation-defined.
*/

#define BYTE_TO_BINARY_PATTERN "%c%c%c%c%c%c%c%c"
#define BYTE_TO_BINARY(byte)  \
  (byte & 0x80 ? '1' : '0'), \
  (byte & 0x40 ? '1' : '0'), \
  (byte & 0x20 ? '1' : '0'), \
  (byte & 0x10 ? '1' : '0'), \
  (byte & 0x08 ? '1' : '0'), \
  (byte & 0x04 ? '1' : '0'), \
  (byte & 0x02 ? '1' : '0'), \
  (byte & 0x01 ? '1' : '0') 

void print_bits(uint8_t *buf, uint32_t len) {
    int i;

    for (i = 0; i < len; i++) {
        printf(BYTE_TO_BINARY_PATTERN, BYTE_TO_BINARY(buf[i]));
    }

    printf("\n");
}

void test_shift() {
    uint16_t u16 = 0xffff;
    int16_t i16 = 0xffff;

    assert((u16 >> 1) == 0x7fff);
    assert((uint16_t)(i16 >> 1) == 0xffff);
    i16 = 0x7fff;
    assert((i16 >> 1) == 0x3fff);

    {
        uint8_t a = 1;
        uint8_t b = -a;
        print_bits(&b, 1);
        assert(b == 0xff);
        b = -2;
        print_bits(&b, 1);
        assert(b == 0xfe);
    }
}
