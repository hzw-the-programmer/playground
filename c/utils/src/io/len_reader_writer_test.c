#include <string.h>
#include <assert.h>
#include "len_reader_writer.h"
#include "utils.h"

typedef struct {
    uint32_t flags;
    slice_t in;
    slice_t out;
} fixture_t;

static void fixtures_1_len_writer_test(len_writer_t *w) {
    fixture_t fixtures[] = {
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

    int i;

    for (i = 0; i < ARRAY_SIZE(fixtures); i++) {
        fixture_t *fixture = &fixtures[i];
        w->flags = fixture->flags;
        assert(len_writer_write_slice(w, &fixture->in) == fixture->in.len);
        assert(buf_buffered(w->buf) == fixture->out.len);
        assert(memcmp(fixture->out.data, buf_read_ptr(w->buf), fixture->out.len) == 0);
        buf_read_inc(w->buf, fixture->out.len);
    }
}

static void len_writer_test_1() {
    int pool[128];
    len_writer_t w = {0};

    w.buf = buf_static((uint8_t*)pool, sizeof(pool));
    fixtures_1_len_writer_test(&w);
}

static void len_reader_test_1() {
}

void len_reader_writer_test() {
    len_writer_test_1();
    len_reader_test_1();
}