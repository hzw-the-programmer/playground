#include <assert.h>
#include <stdint.h>

__declspec(align(4)) uint8_t b[] = {0x01, 0x02, 0x03, 0x04, 0x05};

void test_align() {
    uint32_t *u32 = (uint32_t*)b;
    assert(*u32 == 0x04030201);
    u32 = (uint32_t*)(b + 1);
    assert(*u32 == 0x05040302);
}