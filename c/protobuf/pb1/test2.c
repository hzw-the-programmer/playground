#include "stdint.h"
#include "pb.h"

typedef struct {
    char *f1;
    char *f2;
    char *f3;
    char *f4;
    char *f5;
    int32_t f6;
    int64_t f7;
    char *f8;
    char *f9;
} Test2;

Test2* test2_unpack(size_t len, uint8_t *data) {
    ScannedMember sm;
    Test2 *t2;

    t2 = malloc(sizeof(Test2));
    if (!t2) {
        return NULL;
    }

    while (pb_scanned_member(len, data, &sm)) {
        switch (sm.tag) {
            case 1:
                t2->f1 = pb_string(&sm);
            case 2:
                t2->f2 = pb_string(&sm);
            case 3:
                t2->f3 = pb_string(&sm);
            case 4:
                t2->f4 = pb_string(&sm);
            case 5:
                t2->f5 = pb_string(&sm);
        }

        len -= sm.data + sm.len - data;
        data = sm.data + sm.len;
    }

    return t2;
}

void test2_process(uint8_t *buf, size_t len) {
    Test2 *t2 = test2_unpack(len, buf);
}
