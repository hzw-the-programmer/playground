#include <stdint.h> // uint32_t
#include <assert.h>
#include <string.h> // memcmp

size_t uint32_pack(uint32_t value, char *buf, size_t len) {
    size_t index = 0;

    while (value >= 0x80) {
        if (index < len) {
            buf[index++] = value | 0x80;
            value >>= 7;
        }
    }

    if (index < len) {
        buf[index++] = value;
    }

    return index;
}

uint32_t uint32_unpack(const char *buf, size_t len) {
    size_t index = 0;
    uint32_t value = 0;

    while (index < len && buf[index]  & 0x80) {
        value |= (buf[index++] & 0x7f) << (index * 7);
    }

    if (index < len) {
        value |= (buf[index++] & 0x7f) << (index * 7);
    }

    return value;
}

void test_varints() {
    char buf[32] = {0xff};
    assert(uint32_pack(300, buf, 32) == 2);
    assert(memcmp(buf, "\xac\x02", 2) == 0);
    assert(uint32_unpack(buf, 32) == 300);
}
