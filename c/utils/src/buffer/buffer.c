#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "mem/mem.h"
#include "buffer.h"
#include "slice/split.h"

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

int buf_write(buf_t *buf, slice_t slice) {
    int len = slice.len;
    if (len > buf_available(buf)) {
        len = buf_available(buf);
    }
    if (!len) {
        return 0;
    }
    if (slice.ptr) {
        memmove(buf_write_ptr(buf), slice.ptr, len);    
    }
    buf_write_inc(buf, len);
    return len;
}

void buf_split(buf_t *buf, slice_t sep, int (*cb)(void*, slice_t*), void *arg) {
    split_t split = split_new(buf_buffered_slice(buf), sep);
    while (1) {
        slice_t slice = split_next(&split);
        if (!cb(arg, &slice)) {
            break;
        }
    }
    buf_read_inc(buf, buf_buffered(buf) - split.slice.len);
    buf_tidy(buf);
}

buf_t* buf_static(uint8_t *ptr, int len) {
    buf_t *buf = NULL;

    buf = (buf_t*)ptr;
    buf->cap = len - sizeof(*buf);
    buf->w = buf->r = 0;
    buf->ptr = ptr + sizeof(*buf);

    return buf;
}

slice_t buf_buffered_slice(const buf_t *buf) {
    slice_t slice;

    slice.ptr = buf_read_ptr(buf);
    slice.len = buf_buffered(buf);

    return slice;
}

int buf_read(buf_t *buf, slice_t *slice) {
    slice->ptr = buf_read_ptr(buf);
    if (slice->len > buf_buffered(buf)) {
        slice->len = buf_buffered(buf);
    }
    buf_read_inc(buf, slice->len);
    return slice->len;
}

int buf_read_out(buf_t *buf, slice_t *out) {
    slice_t slice;

    slice.len = out->len;
    if (buf_read(buf, &slice)) {
        memmove(out->ptr, slice.ptr, slice.len);
    }    
    return slice.len;
}