#include <stdint.h> // uint32_t
#include <assert.h>
#include <string.h> // memcmp

#define output(buf, len, index, value) if (index < len) buf[index] = value

size_t uint32_pack(uint32_t value, char *buf, size_t len) {
    size_t index = 0;

    while (value & ~0x7f) {
        output(buf, len, index, value | 0x80);
        index++;
        value >>= 7;
    }

    output(buf, len, index, value);
    index++;

    return index;
}

size_t uint32_unpack(uint32_t *value, const char *buf, size_t len) {
    size_t index = 0;
    char b;
    
    *value = 0;
    while (index < len) {
        b = buf[index];
        *value |= (b & 0x7f) << (index * 7);
        index++;
        if (!(b  & 0x80)) {
            break;
        }
    }

    return index;
}

void test_varints() {
    char buf[32] = {0xff};
    uint32_t value;

    assert(uint32_pack(127, NULL, 0) == 1);
    assert(uint32_pack(127, buf, 32) == 1);
    assert(memcmp(buf, "\x7f", 1) == 0);

    assert(uint32_unpack(&value, buf, 32) == 1);
    assert(value == 127);
    
    assert(uint32_pack(300, NULL, 0) == 2);
    assert(uint32_pack(300, buf, 32) == 2);
    assert(memcmp(buf, "\xac\x02", 2) == 0);

    assert(uint32_unpack(&value, buf, 32) == 2);
    assert(value == 300);

    // 1000 0000 1000 0000 0111 1111
    // 0001 1111 1100 0000 0000 0000
    assert(uint32_pack(0x1fc000, NULL, 0) == 3);
    assert(uint32_pack(0x1fc000, buf, 32) == 3);
    assert(memcmp(buf, "\x80\x80\x7f", 3) == 0);

    assert(uint32_unpack(&value, buf, 32) == 3);
    assert(value == 0x1fc000);
}
