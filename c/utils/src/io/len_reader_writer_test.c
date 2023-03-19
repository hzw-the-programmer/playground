#include <string.h>
#include <assert.h>
#include "len_reader_writer.h"
#include "utils.h"

#define MAX_BUF 0x010203FF
int pool[MAX_BUF];

typedef struct {
    uint32_t flags;
    slice_t in;
    uint8_t *header;
} fixture_t;

static fixture_t fixtures_1[] = {
    {
        LEN_SIZE_1,
        {"hzw", 3},
        "\3",
    },
    {
        LEN_SIZE_2,
        {"bj&tj", 5},
        "\0\5",
    },
    {
        LEN_SIZE_4,
        {"bj&tj", 5},
        "\0\0\0\5",
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
        "\1\4",
    },
    {
        LEN_SIZE_4,
        {NULL, 0x01020314},
        "\1\2\3\x14",
    },
};

static void fixtures_1_len_writer_test(len_writer_t *w) {
    int i;

    for (i = 0; i < ARRAY_SIZE(fixtures_1); i++) {
        fixture_t *fixture = &fixtures_1[i];
        w->flags = fixture->flags;
        assert(len_writer_write(w, &fixture->in) == fixture->in.len);
        assert(buf_buffered(w->buf) == LEN_SIZE(w) + fixture->in.len);
        assert(memcmp(buf_read_ptr(w->buf), fixture->header, LEN_SIZE(w)) == 0);
        buf_read_inc(w->buf, LEN_SIZE(w));
        if (fixture->in.ptr) {
            assert(memcmp(buf_read_ptr(w->buf), fixture->in.ptr, fixture->in.len) == 0);
        }
        buf_read_inc(w->buf, fixture->in.len);
    }
    assert(buf_buffered(w->buf) == 0);
}

static void len_writer_test_1() {
    len_writer_t w = {0};

    w.buf = buf_static((uint8_t*)pool, sizeof(pool));
    fixtures_1_len_writer_test(&w);
}

static void len_reader_test_1() {
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