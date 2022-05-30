#include "buffer.h"
#include "../utils.h"

int h_buf_reserve(h_buf *buf, int len) {
    int cap;
    char *data;

    cap = buf->len + len;
    if (cap <= buf->cap) {
        return 1;
    }

    cap = MAX(cap, buf->cap << 1);

    data = realloc(buf->data, cap);
    if (data == 0) {
        return 0;
    }

    buf->data = data;
    buf->cap = cap;
    return 1;
}

int h_buf_write(h_buf *buf, char *data, int len) {
    if (!h_buf_reserve(buf, len)) return 0;
    
    memcpy(buf->data + buf->len, data, len);
    buf->len += len;
    
    return len;
}

int h_buf_drain(h_buf *buf, int len) {
    if (len >= buf->len) {
        len = buf->len;
        buf->len = 0;
        return len;
    }

    buf->len -= len;
    memmove(buf->data, buf->data + len, buf->len);
    return len;    
}