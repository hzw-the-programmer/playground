#include <stdlib.h>
#include <stddef.h>
#include <string.h>
#include <assert.h>

#define bool char
#define true 1
#define false 0

typedef struct {
    char *ptr;
    size_t cap;
    size_t len;
} buffer2_t;

bool reserve(buffer2_t *buf, size_t by) {
    if (buf->len + by > buf->cap) {
        char *ptr = NULL;
        while (buf->len + by > buf->cap) {
            buf->cap = buf->cap ? 2 * buf->cap : 8;
        }
        ptr = (char*)realloc(buf->ptr, buf->cap);
        if (ptr == NULL) {
            return false;
        }
        buf->ptr = ptr;
    }
    return true;
}

bool reserve_str(buffer2_t *buf, size_t by) {
    if (buf->len == 0) {
        by += 1;
    }

    if (!reserve(buf, by)) {
        return false;
    }

    if (buf->len == 0) {
        buf->len = 1;
    }
    buf->ptr[buf->len - 1] = '\0';

    return true;
}

bool append_header(buffer2_t *buf, const char *k, const char *v) {
    size_t kl, vl, tl;

    kl = strlen(k);
    vl = strlen(v);

    tl = kl + 2 + vl + 2;

    if (!reserve_str(buf, tl)) {
        return false;
    }

    strcat(buf->ptr, k);
    strcat(buf->ptr, ": ");
    strcat(buf->ptr, v);
    strcat(buf->ptr, "\r\n");

    buf->len += tl;

    return true;
}

bool delete_header(buffer2_t *buf, const char *k) {
    size_t kl;
    char *s, *e;

    if (buf->len == 0) {
        return true;
    }

    kl = strlen(k);

    s = buf->ptr;
    e = NULL;
    for (;;) {
        s = strstr(s, k);
        if (s == NULL) {
            break;
        }

        if (*(s+kl) == ':' && *(s+kl+1) == ' ') {
            e = strstr(s + kl + 2, "\r\n");
            break;
        }

        s += kl;
    }

    if (e != NULL) {
        memmove(s, e+2, buf->ptr+buf->len-e-2);
        buf->len -= e+2-s;
    }

    return true;
}

bool replace_header(buffer2_t *buf, const char *k, const char *v) {
    delete_header(buf, k);
    return append_header(buf, k, v);
}

void test_delete_header() {
    buffer2_t buf = {0};
    delete_header(&buf, "k1");
}

void test_replace_header() {
    buffer2_t buf = {0};
    replace_header(&buf, "k1", "v1");
    assert(strcmp("k1: v1\r\n", buf.ptr) == 0);
}

void test_op_header() {
    buffer2_t buf = {0};

    append_header(&buf, "k1", "v1");
    assert(strcmp("k1: v1\r\n", buf.ptr) == 0);
    append_header(&buf, "k2", "v2");
    assert(strcmp("k1: v1\r\nk2: v2\r\n", buf.ptr) == 0);
    append_header(&buf, "k3", "v3");
    assert(strcmp("k1: v1\r\nk2: v2\r\nk3: v3\r\n", buf.ptr) == 0);
    append_header(&buf, "k4", "v4");
    assert(strcmp("k1: v1\r\nk2: v2\r\nk3: v3\r\nk4: v4\r\n", buf.ptr) == 0);

    replace_header(&buf, "k5", "v5");
    assert(strcmp("k1: v1\r\nk2: v2\r\nk3: v3\r\nk4: v4\r\nk5: v5\r\n", buf.ptr) == 0);

    replace_header(&buf, "k2", "rk42");
    assert(strcmp("k1: v1\r\nk3: v3\r\nk4: v4\r\nk5: v5\r\nk2: rk42\r\n", buf.ptr) == 0);

    replace_header(&buf, "k1", "kk21");
    assert(strcmp("k3: v3\r\nk4: v4\r\nk5: v5\r\nk2: rk42\r\nk1: kk21\r\n", buf.ptr) == 0);

    replace_header(&buf, "k2", "hzw");
    assert(strcmp("k3: v3\r\nk4: v4\r\nk5: v5\r\nk1: kk21\r\nk2: hzw\r\n", buf.ptr) == 0);

    replace_header(&buf, "k2", "ml");
    assert(strcmp("k3: v3\r\nk4: v4\r\nk5: v5\r\nk1: kk21\r\nk2: ml\r\n", buf.ptr) == 0);
}

void test_buffer2() {
    test_delete_header();
    test_replace_header();
    test_op_header();
}
