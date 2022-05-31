#include <stdlib.h> // realloc
#include <string.h> // memcpy
#include "buffer.h"
#include "slice.h"
#include "../utils.h"

int h_buf_reserve(h_buf *buf, int len) {
    int cap;
    char *data;

    cap = buf->len + len;
    if (cap <= buf->cap) {
        return 1;
    }

    cap = MAX(cap, buf->cap << 1);

    data = realloc(buf->data, cap + 1);
    if (data == 0) {
        return 0;
    }

    buf->data = data;
    buf->cap = cap;
    return 1;
}

int h_buf_write(h_buf *buf, const char *data, int len) {
    if (!h_buf_reserve(buf, len)) return 0;
    
    memcpy(buf->data + buf->len, data, len);
    buf->len += len;
    buf->data[buf->len] = 0;
    
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

int h_buf_append_header(h_buf *buf, const char *key, const char *value) {
    int kl, vl, l;

    kl = strlen(key); vl = strlen(value);
    l = kl + 1 + vl + 2;
    if (!h_buf_reserve(buf, l)) return 0;

    h_buf_write(buf, key, kl);
    h_buf_write(buf, ":", 1);
    h_buf_write(buf, value, vl);
    h_buf_write(buf, "\r\n", 2);

    return l;
}

int h_buf_delete_header(h_buf *buf, const char *key) {
    h_slice line, k;
    h_slice_split lines, header;
    int l;

    l = strlen(key);

    lines = h_slice_split_new(h_slice_new(buf->data, buf->len), '\n');
    while (1) {
        line = h_slice_split_next(&lines);
        if (line.len == 0) {
            return 0;
        }
        header = h_slice_split_new(line, ':');
        k = h_slice_trim_space(h_slice_split_next(&header));
        if (k.len == l && strncmp(k.data, key, k.len) == 0) {
            if (lines.s.len != 0) {
                memmove(line.data, lines.s.data, lines.s.len);
                line.len = lines.s.data - line.data;
                lines.s.data -= line.len;
            } else if (lines.s.data != 0) {
                line.len++;
            }
            buf->len -= line.len;
            buf->data[buf->len] = 0;
        }
    }
}