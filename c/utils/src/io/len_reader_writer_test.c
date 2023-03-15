#include <string.h>
#include <assert.h>
#include "len_reader_writer.h"
#include "utils.h"

static void len_writer_test_1() {
    char *in_0 = "hzw";
    struct {
        uint32_t flags;
        slice_t in;
        slice_t out;
    } tests[] = {
        {
            LEN_SIZE_1,
            {"hzw", 3},
            {"\3hzw", 4},
        },
        {
            LEN_SIZE_2,
            {"bj&tj", 5},
            {"\0\5bj&tj", 7},
        },
        {
            LEN_SIZE_4,
            {"bj&tj", 5},
            {"\0\0\0\5bj&tj", 9},
        },
    };
    
    int pool[128];
    len_writer_t w = {0};

    int i, ret;

    w.buf = buf_static((uint8_t*)pool, sizeof(pool));

    for (i = 0; i < ARRAY_SIZE(tests); i++) {
        w.flags = tests[i].flags;
        ret = len_writer_write_slice(&w, &tests[i].in);
        assert(ret == tests[i].in.len);
        assert(buf_buffered(w.buf) == tests[i].out.len);
        assert(memcmp(tests[i].out.data, buf_read_ptr(w.buf), tests[i].out.len) == 0);
        buf_read_inc(w.buf, tests[i].out.len);
    }
}

void len_reader_writer_test() {
    len_writer_test_1();
}