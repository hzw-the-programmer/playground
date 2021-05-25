#include "test1.pb-c.h"

void test1_process(size_t len, const uint8_t *buf) {
    Test1 *t1, t;
    
    t1 = test1__unpack(NULL, len, buf);
    test1__free_unpacked(t1, NULL);

    t.f1 = "hj1";
    t.f2 = "hj2";
    t.f3 = "bj1";
    t.f4 = "bj2";
    t.f5 = "bj3";
    t.f6 = 789;
    t.f7 = 321;
    t.f8 = "tj1";
    t.f9 = "tj2";
    len = test1__get_packed_size(&t);
    buf = malloc(len);
    test1__pack(&t, buf);

    t1 = test1__unpack(NULL, len, buf);
    test1__free_unpacked(t1, NULL);
}
