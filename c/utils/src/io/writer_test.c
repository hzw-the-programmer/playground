#include <string.h>
#include <assert.h>
#include "buffer/buffer.h"

typedef struct {
    int len;
    buf_t *buf;
} len_writer;

int len_writer_write(len_writer *w, const uint8_t *p, int plen, int *len) {
    uint8_t *h;

    if (w->len + plen > buf_available(w->buf)) {
        *len = 0;
        return -1;
    }

    h = buf_write_ptr(w->buf);

    buf_write_inc(w->buf, w->len);
    buf_write(w->buf, p, plen);
    
    if (w->len == 1) {
        *h = plen;
    } else if (w->len == 2) {
        *h = plen>>8;
        *(h+1) = plen;
    } else if (w->len == 4) {
        *h = plen>>24;
        *(h+1) = plen>>16;
        *(h+2) = plen>>8;
        *(h+3) = plen;
    }

    *len = plen;
    return 0;
}

static void writer_test_1() {
    int pool[128];
    len_writer w = {0};
    uint8_t want[] = {'\3', 'h', 'z', 'w'};
    uint8_t want1[] = {'\0', '\5', 'b', 'j', '&', 't', 'j'};
    uint8_t want2[] = {'\0', '\0', '\0', '\5', 'b', 'j', '&', 't', 'j'};
    int ret, len;

    w.buf = buf_static((uint8_t*)pool, sizeof(pool));

    w.len = 1;
    ret = len_writer_write(&w, "hzw", 3, &len);
    assert(ret == 0);
    assert(len == 3);
    assert(memcmp(want, buf_read_ptr(w.buf), 3 + w.len) == 0);
    buf_read_inc(w.buf, 3 + w.len);

    w.len = 2;
    ret = len_writer_write(&w, "bj&tj", 5, &len);
    assert(ret == 0);
    assert(len == 5);
    assert(memcmp(want1, buf_read_ptr(w.buf), 5 + w.len) == 0);
    buf_read_inc(w.buf, 5 + w.len);

    w.len = 4;
    ret = len_writer_write(&w, "bj&tj", 5, &len);
    assert(ret == 0);
    assert(len == 5);
    assert(memcmp(want2, buf_read_ptr(w.buf), 5 + w.len) == 0);
    buf_read_inc(w.buf, 5 + w.len);
}

void writer_test() {
    writer_test_1();
}