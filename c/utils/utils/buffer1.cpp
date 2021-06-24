#include <string.h> // memcpy
#include <assert.h> // assert

#include "memory.h"
#include "buffer1.h"

#define NULL 0

void buffer_free(buffer_t *buf) {
    if (!buf) return;
    if (buf->data) HzwFree(buf->data);
    HzwFree(buf);
}

buffer_t* buffer_new(int cap) {
    bool fail = true;
    buffer_t *buf = NULL;

    buf = (buffer_t*)HZW_MALLOC(sizeof(buffer_t));
    if (!buf) goto error;
    buf->data = (char*)HZW_MALLOC(cap + 1);
    if (!buf->data) goto error;
    buf->cap = cap;
    buf->len = 0;
    buf->data[buf->len] = 0;

    fail = false;

error:
    if (fail) {
        buffer_free(buf);
        buf = NULL;
    }

    return buf;
}

char* buffer_writable(buffer_t *buf) {
    return buf->data + buf->len;
}

int buffer_writable_len(buffer_t *buf) {
    return buf->cap - buf->len;
}

char* buffer_readable(buffer_t *buf) {
    return buf->data;
}

int buffer_readable_len(buffer_t *buf) {
    return buf->len;
}

void buffer_inc_len(buffer_t *buf, int len) {
    buf->len += len;
    buf->data[buf->len] = 0;
}

int buffer_copy(buffer_t *dst, buffer_t *src, int len) {
    int rlen, wlen;

    rlen = buffer_readable_len(src);
    wlen = buffer_writable_len(dst);

    if (!len || len > rlen) {
        len = rlen;
    }

    if (len > wlen) {
        len = wlen;
    }

    memcpy(buffer_writable(dst), buffer_readable(src), len);
    buffer_inc_len(dst, len);

    return len;
}

int buffer_copy_str(buffer_t *dst, char *str) {
    int len;

    len = strlen(str);
    if (len > buffer_writable_len(dst)) {
        len = buffer_writable_len(dst);
    }

    memcpy(buffer_writable(dst), str, len);
    
    buffer_inc_len(dst, len);

    return len;
}

int buffer_clear(buffer_t *buf, int len) {
    if (len < buf->len) {
        memmove(buf->data , buf->data + len, buf->len - len);
    } else {
        len = buf->len;
    }

    buffer_inc_len(buf, -len);

    return len;
}

bool buffer_full(buffer_t *buf) {
    return buf->len == buf->cap;
}

void test_buffer_copy_str() {
    buffer_t *buf;

    buf = buffer_new(10);

    assert(5 == buffer_copy_str(buf, "hello"));
    assert(!strcmp(buffer_readable(buf), "hello"));
    assert(5 == buffer_copy_str(buf, " world!"));
    assert(!strcmp(buffer_readable(buf), "hello worl"));

    buffer_free(buf);
}

void test_buffer_copy() {
    buffer_t *src;
    buffer_t *dst;

    src = buffer_new(12);
    dst = buffer_new(10);

    assert(5 == buffer_copy_str(src, "hello"));
    assert(!strcmp(buffer_readable(src), "hello"));
    assert(5 == buffer_copy(dst, src, 0));
    assert(!strcmp(buffer_readable(dst), "hello"));

    assert(7 == buffer_copy_str(src, " world!"));
    assert(!strcmp(buffer_readable(src), "hello world!"));
    assert(5 == buffer_copy(dst, src, 0));
    assert(!strcmp(buffer_readable(dst), "hellohello"));

    buffer_free(src);
    buffer_free(dst);
}

void test_buffer1() {
    test_buffer_copy_str();
    test_buffer_copy();
}
