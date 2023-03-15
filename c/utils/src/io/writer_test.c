#include <string.h>
#include <assert.h>
#include "buffer/buffer.h"
#include "writer.h"

static void writer_test_1() {
    int pool[128];
    len_writer w = {0};
    uint8_t want_0[] = {'\3', 'h', 'z', 'w'};
    uint8_t want_1[] = {'\0', '\5', 'b', 'j', '&', 't', 'j'};
    uint8_t want_2[] = {'\0', '\0', '\0', '\5', 'b', 'j', '&', 't', 'j'};
    int ret, len;

    w.buf = buf_static((uint8_t*)pool, sizeof(pool));

    w.flags = LEN_SIZE_1;
    ret = len_writer_write(&w, "hzw", 3, &len);
    assert(ret == 0);
    assert(len == 3);
    assert(memcmp(want_0, buf_read_ptr(w.buf), 3 + LEN_SIZE(&w)) == 0);
    buf_read_inc(w.buf, 3 + LEN_SIZE(&w));

    w.flags = LEN_SIZE_2;
    ret = len_writer_write(&w, "bj&tj", 5, &len);
    assert(ret == 0);
    assert(len == 5);
    assert(memcmp(want_1, buf_read_ptr(w.buf), 5 + LEN_SIZE(&w)) == 0);
    buf_read_inc(w.buf, 5 + LEN_SIZE(&w));

    w.flags = LEN_SIZE_4;
    ret = len_writer_write(&w, "bj&tj", 5, &len);
    assert(ret == 0);
    assert(len == 5);
    assert(memcmp(want_2, buf_read_ptr(w.buf), 5 + LEN_SIZE(&w)) == 0);
    buf_read_inc(w.buf, 5 + LEN_SIZE(&w));
}

void writer_test() {
    writer_test_1();
}