#include <assert.h>
#include "buffer/buffer.h"

typedef struct {
    int len;
    buf_t *buf;
} len_writer;

int len_writer_write(len_writer *w, const uint8_t *p, int plen, int *len) {
    uint8_t *b = buf_write_ptr(w->buf);
    buf_write_inc(w->buf, w->len);
    buf_write(w->buf, p, plen);
    *b = plen;
    *len = plen;
    return 0;
}

static void writer_test_1() {
    int pool[128];
    len_writer w = {0};
    uint8_t want[] = {'\3', 'h', 'z', 'w'};
    int ret, len;

    w.len = 1;
    w.buf = buf_static((uint8_t*)pool, sizeof(pool));
    ret = len_writer_write(&w, "hzw", 3, &len);
    assert(ret == 0);
    assert(len == 3);
    assert(memcmp(want, buf_read_ptr(w.buf), 3 + w.len) == 0);
}

void writer_test() {
    writer_test_1();
}