#include <stdio.h>
#include "stdint.h"
#include "test1.pb-c.h"

void test1_process(uint8_t *buf, size_t len) {
    Test1 *t1 = test1__unpack(NULL, len, buf);
    printf("f1=%s\n", t1->f1);
    printf("f2=%s\n", t1->f2);
    printf("f3=%s\n", t1->f3);
    printf("f4=%s\n", t1->f4);
    printf("f5=%s\n", t1->f5);
    printf("f6=%d\n", t1->f6);
    printf("f7=%d\n", t1->f7);
    printf("f8=%s\n", t1->f8);
    printf("f9=%s\n", t1->f9);
}
