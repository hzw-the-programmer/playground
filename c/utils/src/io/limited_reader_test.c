#include <string.h>
#include <assert.h>

#include "buffer/buffer.h"
#include "mem/mem.h"
#include "io/limited_reader.h"

void limited_reader_test() {
    char *data = "0123456789abcdefghijklmnopqrstuvwxyz";
    int len;
    buf_t *buf = buf_new(1024);
    limited_reader_t r = {buf, 23};
    slice_t slice;

    data = "0123";
    len = 4;
    buf_write(buf, slice_new(data, 4));
    limited_reader_read(&r, &slice);
    assert(slice.len == len);
    assert(!strncmp(slice.ptr, data, len));
    assert(r.len == 19);

    data = "456789";
    len = 6;
    buf_write(buf, slice_new(data, 6));
    limited_reader_read(&r, &slice);
    assert(slice.len == len);
    assert(!strncmp(slice.ptr, data, len));
    assert(r.len == 13);

    data = "abcdefghijklmnopqrstuvwxyz";
    len = 26;
    buf_write(buf, slice_new(data, 26));
    limited_reader_read(&r, &slice);
    assert(slice.len == 13);
    assert(!strncmp(slice.ptr, data, slice.len));
    assert(r.len == 0);
    assert(!strncmp(buf_read_ptr(buf), data + slice.len, len - slice.len));

    free(buf);
}