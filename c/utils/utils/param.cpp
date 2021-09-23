#include <assert.h>
#include <stdint.h>

void u16(uint16_t *i) {
    *i = 0x0102;
}

void u32(uint32_t *i) {
    *i = 0x01020304;
}

void test_param() {
    uint32_t i32 = 0x03040506;
    uint16_t i16 = 0x0102;
    uint32_t *p = &i32;
    u16((uint16_t*)&i32);
    assert(i32 == 0x03040102);

    /*
    Run-Time Check Failure #2 - Stack around the variable 'i16' was corrupted.
    */
    //u32((uint32_t*)&i16);
}
