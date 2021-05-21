#include "stdint.h"
#include "test2.pb-c.h"

void test2_process(size_t len, const uint8_t *buf) {
    A a = {"I am a string"};
    B b = {1123};
    C c = {&a, &b}, *c1;

    len = c__get_packed_size(&c);
    buf = malloc(len);
    c__pack(&c, buf);
    c1 = c__unpack(NULL, len, buf);
    c__free_unpacked(c1, NULL);
}
