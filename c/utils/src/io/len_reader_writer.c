#include "len_reader_writer.h"

int len_reader_read(void *arg, slice_t *slice) {
    len_reader_t *r = arg;
    return 0;
}

int len_writer_write(void *arg, const uint8_t *data, int len) {
    len_writer_t *w = arg;
    uint8_t *header;

    if ( LEN_SIZE(w) + len > buf_available(w->buf)) {
        return -1;
    }

    header = buf_write_ptr(w->buf);

    buf_write_inc(w->buf, LEN_SIZE(w));
    buf_write(w->buf, data, len);

    if (LEN_SIZE(w) == 1) {
        *header = len;
    } else if (LEN_SIZE(w) == 2) {
        *header = len>>8;
        *(header+1) = len;
    } else if (LEN_SIZE(w) == 4) {
        *header = len>>24;
        *(header+1) = len>>16;
        *(header+2) = len>>8;
        *(header+3) = len;
    }

    return len;
}

int len_writer_write_slice(void *arg, const slice_t *slice) {
    return len_writer_write(arg, slice->data, slice->len);
}
