#include <string.h>
#include <assert.h>
#include "sep_reader_writer.h"
#include "reader_writer.h"
#include "utils.h"

typedef struct {
    slice_t sep;
    slice_t in;
} fixture_t;

static fixture_t fixtures_1[] = {
    {
        {"\r\n", 2},
        {"the first line", 14},
    },
    {
        {"\r", 1},
        {"the second line", 15},
    },
    {
        {"\n", 1},
        {"the third line", 14},
    },
};

static void sep_writer_fixtures_1_test(sep_writer_t *w, write_t func) {
    int i;
    for (i = 0; i < ARRAY_SIZE(fixtures_1); i++) {
        fixture_t *fixture = &fixtures_1[i];
        w->sep = fixture->sep;
        assert(func(w, &fixture->in) == fixture->in.len);
        assert(buf_buffered(w->buf) == fixture->in.len + fixture->sep.len);
        assert(memcmp(buf_read_ptr(w->buf), fixture->in.data, fixture->in.len) == 0);
        buf_read_inc(w->buf, fixture->in.len);
        assert(memcmp(buf_read_ptr(w->buf), fixture->sep.data, fixture->sep.len) == 0);
        buf_read_inc(w->buf, fixture->sep.len);
    }
    assert(buf_buffered(w->buf) == 0);
}

static void sep_writer_test() {
    int pool[128];
    sep_writer_t w = {0};
    
    w.buf = buf_static((uint8_t*)pool, sizeof(pool));
    sep_writer_fixtures_1_test(&w, sep_writer_write);
}

static void sep_reader_test() {
    int pool[128];
    sep_writer_t w = {0};
    sep_reader_t r = {0};
    int i;
    read_t func = sep_reader_read;
    slice_t slice;

    w.buf = buf_static((uint8_t*)pool, sizeof(pool));
    sep_writer_fixtures_1_test(&w, sep_writer_write);
    
    r.buf = w.buf;
    r.buf->r = 0;
    for (i = 0; i < ARRAY_SIZE(fixtures_1); i++) {
        fixture_t *fixture = &fixtures_1[i];
        r.sep = fixture->sep;
        assert(func(&r, &slice) == fixture->in.len);
        assert(slice.len = fixture->in.len);
        assert(memcmp(slice.data, fixture->in.data, fixture->in.len) == 0);
    }
    assert(buf_buffered(r.buf) == 0);
}

void sep_reader_writer_test() {
    sep_writer_test();
    sep_reader_test();
}