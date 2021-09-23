#include <assert.h>
#include <stdint.h>

/*
The result of E1 >> E2 is E1 right-shifted E2 bit positions.
If E1 has an unsigned type or if E1 has a signed type and a nonnegative value, the value of the result is the integral part of the quotient of E12E2.
If E1 has a signed type and a negative value, the resulting value is implementation-defined.
*/

void test_shift() {
    uint16_t u16 = 0xffff;
    int16_t i16 = 0xffff;

    assert((u16 >> 1) == 0x7fff);
    assert((uint16_t)(i16 >> 1) == 0xffff);
    i16 = 0x7fff;
    assert((i16 >> 1) == 0x3fff);
}
