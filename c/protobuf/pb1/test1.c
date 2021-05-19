#include "test1.pb-c.h"

void test1_process(size_t len, const uint8_t *buf) {
    Test1 *t1 = test1__unpack(NULL, len, buf);
    test1__free_unpacked(t1, NULL);
}
