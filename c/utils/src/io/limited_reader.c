#include "limited_reader.h"

int limited_reader_read(void *arg, slice_t *slice) {
    limited_reader_t *r = (limited_reader_t*)arg;

    slice->len = r->len;
    r->len -= buf_read(r->buf, slice);
    return slice->len;
}