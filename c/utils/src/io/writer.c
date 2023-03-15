#include "writer.h"

int len_writer_write(len_writer *w, const uint8_t *p, int plen, int *len) {
    uint8_t *h;

    if ( LEN_SIZE(w) + plen > buf_available(w->buf)) {
        *len = 0;
        return -1;
    }

    h = buf_write_ptr(w->buf);

    buf_write_inc(w->buf, LEN_SIZE(w));
    buf_write(w->buf, p, plen);
    
    if (LEN_SIZE(w) == 1) {
        *h = plen;
    } else if (LEN_SIZE(w) == 2) {
        *h = plen>>8;
        *(h+1) = plen;
    } else if (LEN_SIZE(w) == 4) {
        *h = plen>>24;
        *(h+1) = plen>>16;
        *(h+2) = plen>>8;
        *(h+3) = plen;
    }

    *len = plen;
    return 0;
}
