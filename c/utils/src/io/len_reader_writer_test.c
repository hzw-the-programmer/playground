#include <string.h>
#include <assert.h>
#include "len_reader_writer.h"

static void len_writer_test_1() {
    slice_t in_0 = {"hzw", 3};
    slice_t out_0 = {"\3hzw", 4};
    slice_t in_1 = {"bj&tj", 5};
    slice_t out_1 = {"\0\5bj&tj", 7};
    slice_t in_2 = {"bj&tj", 5};
    slice_t out_2 = {"\0\0\0\5bj&tj", 9};
    
    int pool[128];
    len_writer_t w = {0};
    
    slice_t in;
    slice_t out;
    int ret;

    w.buf = buf_static((uint8_t*)pool, sizeof(pool));

    in = in_0;
    out = out_0;
    w.flags = LEN_SIZE_1;
    ret = len_writer_write_slice(&w, &in);
    assert(ret == in.len);
    assert(memcmp(out.data, buf_read_ptr(w.buf), out.len) == 0);
    buf_read_inc(w.buf, out.len);

    in = in_1;
    out = out_1;
    w.flags = LEN_SIZE_2;
    ret = len_writer_write_slice(&w, &in);
    assert(ret == in.len);
    assert(memcmp(out.data, buf_read_ptr(w.buf), out.len) == 0);
    buf_read_inc(w.buf, out.len);

    in = in_2;
    out = out_2;
    w.flags = LEN_SIZE_4;
    ret = len_writer_write_slice(&w, &in);
    assert(ret == in.len);
    assert(memcmp(out.data, buf_read_ptr(w.buf), out.len) == 0);
    buf_read_inc(w.buf, out.len);
}

void len_reader_writer_test() {
    len_writer_test_1();
}