#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "buffer.h"

buf_t* buf_new(int cap) {
    buf_t *buf;

    buf = malloc(sizeof(*buf) + cap);
    if (!buf) {
        return NULL;
    }

    buf->cap = cap;
    buf->w = buf->r = 0;
    buf->ptr = (uint8_t*)buf + sizeof(*buf);

    return buf;
}

int buf_available(const buf_t *buf) {
    return buf->cap - buf->w;
}

uint8_t* buf_write_ptr(const buf_t *buf) {
    return buf->ptr + buf->w;
}

void buf_write_inc(buf_t *buf, int len) {
    buf->w += len;
}

int buf_buffered(const buf_t *buf) {
    return buf->w - buf->r;
}

uint8_t* buf_read_ptr(const buf_t *buf) {
    return buf->ptr + buf->r;
}

void buf_read_inc(buf_t *buf, int len) {
    buf->r += len;
}

void buf_tidy(buf_t *buf) {
    if (!buf->r) {
        return;
    }

    if (!buf_buffered(buf)) {
        buf->w = buf->r = 0;
        return;
    }

    memmove(buf->ptr, buf->ptr + buf->r, buf_buffered(buf));
    buf->w -= buf->r;
    buf->r = 0;
}

void buf_write(buf_t *buf, const uint8_t *ptr, int len) {
    assert(len <= buf_available(buf));
    memmove(buf_write_ptr(buf), ptr, len);
    buf_write_inc(buf, len);
}
