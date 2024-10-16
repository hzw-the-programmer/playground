#include "sep_reader_writer.h"

int sep_writer_write(void *arg, slice_t slice) {
    sep_writer_t *w = arg;

    if (slice.len + w->sep.len > buf_available(w->buf)) {
        return -1;
    }

    buf_write(w->buf, slice);
    buf_write(w->buf, w->sep);

    return slice.len;
}

int sep_reader_read(void *arg, slice_t *slice) {
    sep_reader_t *r = arg;
    slice_t m;

    m.ptr = buf_read_ptr(r->buf);
    m.len = buf_buffered(r->buf);
    *slice = slice_slice(m, r->sep);
    if (!slice->ptr) {
        return -1;
    }
    slice->ptr = m.ptr;
    slice->len = m.len - slice->len;
    buf_read_inc(r->buf, slice->len + r->sep.len);

    return slice->len;
}
