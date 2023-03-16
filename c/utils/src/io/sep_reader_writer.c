#include "sep_reader_writer.h"

int sep_writer_write(void *arg, const slice_t *slice) {
    sep_writer_t *w = arg;

    if (slice->len + w->sep.len > buf_available(w->buf)) {
        return -1;
    }

    buf_write(w->buf, slice);
    buf_write(w->buf, &w->sep);

    return slice->len;
}