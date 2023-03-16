#include "len_reader_writer.h"

int len_writer_write(void *arg, const slice_t *slice) {
    len_writer_t *w = arg;
    uint8_t *header;

    if ( LEN_SIZE(w) + slice->len > buf_available(w->buf)) {
        return -1;
    }

    header = buf_write_ptr(w->buf);

    buf_write_inc(w->buf, LEN_SIZE(w));
    buf_write_slice(w->buf, slice);

    if (LEN_SIZE(w) == 1) {
        *header = slice->len;
    } else if (LEN_SIZE(w) == 2) {
        *header = slice->len>>8;
        *(header+1) = slice->len;
    } else if (LEN_SIZE(w) == 4) {
        *header = slice->len>>24;
        *(header+1) = slice->len>>16;
        *(header+2) = slice->len>>8;
        *(header+3) = slice->len;
    }

    return slice->len;
}

int len_reader_read(void *arg, slice_t *slice) {
    len_reader_t *r = arg;

    if (!(r->flags & LEN_READED)) {
        uint8_t *header;
        if (buf_buffered(r->buf) < LEN_SIZE(r)) {
            return -1;
        }
        header = buf_read_ptr(r->buf);
        if (LEN_SIZE(r) == 1) {
            r->len = *header;
        } else if (LEN_SIZE(r) == 2) {
            r->len = (*header<<8)|*(header+1);
        } else if (LEN_SIZE(r) == 4) {
            r->len = (*header<<24)|(*(header+1)<<16)|(*(header+2)<<8)|*(header+3);
        }
        r->flags |= LEN_READED;
        buf_read_inc(r->buf, LEN_SIZE(r));
    }

    if (buf_buffered(r->buf) < r->len) {
        return -1;
    }
    slice->data = buf_read_ptr(r->buf);
    slice->len = r->len;
    buf_read_inc(r->buf, r->len);

    return r->len;
}
