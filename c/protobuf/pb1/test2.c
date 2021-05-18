#include "stdint.h"
#include "pb.h"
#include "memory.h"

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

Test2* test2_unpack(size_t len, const uint8_t *buf) {
    scanned_member_t sm;
    Test2 *t2;

    t2 = HZW_MALLOC(sizeof(Test2));
    if (!t2) {
        return NULL;
    }

    while (pb_scanned_member(len, buf, &sm)) {
        switch (sm.tag) {
            case 1:
                t2->f1 = pb_string(sm.len - sm.length_prefix_len, sm.data + sm.length_prefix_len);
                break;
            case 2:
                t2->f2 = pb_string(sm.len - sm.length_prefix_len, sm.data + sm.length_prefix_len);
                break;
            case 3:
                t2->f3 = pb_string(sm.len - sm.length_prefix_len, sm.data + sm.length_prefix_len);
                break;
            case 4:
                t2->f4 = pb_string(sm.len - sm.length_prefix_len, sm.data + sm.length_prefix_len);
                break;
            case 5:
                t2->f5 = pb_string(sm.len - sm.length_prefix_len, sm.data + sm.length_prefix_len);
                break;
            case 6:
                t2->f6 = pb_uint64(sm.len, sm.data);
                break;
            case 7:
                t2->f7 = pb_uint64(sm.len, sm.data);
                break;
            case 8:
                t2->f8 = pb_string(sm.len - sm.length_prefix_len, sm.data + sm.length_prefix_len);
                break;
            case 9:
                t2->f9 = pb_string(sm.len - sm.length_prefix_len, sm.data + sm.length_prefix_len);
                break;
        }

        len -= sm.data + sm.len - buf;
        buf = sm.data + sm.len;
    }

    return t2;
}

void test2_free(Test2 *t2) {
    if (t2->f1) {
        HzwFree(t2->f1);
    }
    if (t2->f2) {
        HzwFree(t2->f2);
    }
    if (t2->f3) {
        HzwFree(t2->f3);
    }
    if (t2->f4) {
        HzwFree(t2->f4);
    }
    if (t2->f5) {
        HzwFree(t2->f5);
    }
    if (t2->f8) {
        HzwFree(t2->f8);
    }
    if (t2->f9) {
        HzwFree(t2->f9);
    }
    HzwFree(t2);
}

void test2_process(uint8_t *buf, size_t len) {
    Test2 *t2 = test2_unpack(len, buf);
    test2_free(t2);
}
