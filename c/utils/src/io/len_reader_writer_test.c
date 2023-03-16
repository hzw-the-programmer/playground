#include <string.h>
#include <assert.h>
#include "len_reader_writer.h"
#include "utils.h"

typedef struct {
    uint32_t flags;
    slice_t in;
    slice_t out;
} fixture_t;

fixture_t fixtures_1[] = {
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
    {
        LEN_SIZE_2,
        {
            "abcdefghijklmnopqrstuvwxyz"
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
            "abcdefghijklmnopqrstuvwxyz"
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
            "abcdefghijklmnopqrstuvwxyz"
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
            "abcdefghijklmnopqrstuvwxyz"
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
            "abcdefghijklmnopqrstuvwxyz"
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            260
        },
        {
            "\1\4"
            "abcdefghijklmnopqrstuvwxyz"
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
            "abcdefghijklmnopqrstuvwxyz"
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
            "abcdefghijklmnopqrstuvwxyz"
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
            "abcdefghijklmnopqrstuvwxyz"
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
            "abcdefghijklmnopqrstuvwxyz"
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            262
        },
    },
};

static void fixtures_1_len_writer_test(len_writer_t *w) {
    int i;

    for (i = 0; i < ARRAY_SIZE(fixtures_1); i++) {
        fixture_t *fixture = &fixtures_1[i];
        w->flags = fixture->flags;
        assert(len_writer_write(w, &fixture->in) == fixture->in.len);
        assert(buf_buffered(w->buf) == fixture->out.len);
        assert(memcmp(fixture->out.data, buf_read_ptr(w->buf), fixture->out.len) == 0);
        buf_read_inc(w->buf, fixture->out.len);
    }
    assert(buf_buffered(w->buf) == 0);
}

static void len_writer_test_1() {
    int pool[128];
    len_writer_t w = {0};

    w.buf = buf_static((uint8_t*)pool, sizeof(pool));
    fixtures_1_len_writer_test(&w);
}

static void len_reader_test_1() {
    int pool[128];
    len_writer_t w = {0};
    len_reader_t r = {0};
    int i;
    slice_t slice;

    w.buf = buf_static((uint8_t*)pool, sizeof(pool));
    fixtures_1_len_writer_test(&w);

    r.buf = w.buf;
    r.buf->r = 0;
    for (i = 0; i < ARRAY_SIZE(fixtures_1); i++) {
        fixture_t *fixture = &fixtures_1[i];
        r.flags = fixture->flags;
        assert(len_reader_read(&r, &slice) == fixture->in.len);
    }
}

void len_reader_writer_test() {
    len_writer_test_1();
    len_reader_test_1();
}