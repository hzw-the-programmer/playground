#include <string.h>
#include <assert.h>
#include "sep_reader_writer.h"
#include "reader_writer.h"
#include "utils.h"

typedef struct {
    slice_t sep;
    slice_t in;
} fixture_t;

static sep_writer_test() {
    fixture_t fixtures[] = {
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
    int pool[128];
    sep_writer_t w = {0};
    int i;
    write_t func = sep_writer_write;

    w.buf = buf_static((uint8_t*)pool, sizeof(pool));

    for (i = 0; i < ARRAY_SIZE(fixtures); i++) {
        fixture_t *fixture = &fixtures[i];
        w.sep = fixture->sep;
        assert(func(&w, &fixture->in) == fixture->in.len);
        assert(buf_buffered(w.buf) == fixture->in.len + fixture->sep.len);
        assert(memcmp(buf_read_ptr(w.buf), fixture->in.data, fixture->in.len) == 0);
        buf_read_inc(w.buf, fixture->in.len);
        assert(memcmp(buf_read_ptr(w.buf), fixture->sep.data, fixture->sep.len) == 0);
        buf_read_inc(w.buf, fixture->sep.len);
    }
}

void sep_reader_writer_test() {
    sep_writer_test();
}