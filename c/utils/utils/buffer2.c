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
    by += 1;

    if (!reserve(buf, by)) {
        return false;
    }

    buf->ptr[buf->len] = '\0';

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

    if (!reserve_str(buf, 0)) {
        return false;
    }

    kl = strlen(k);

    s = buf->ptr;
    e = NULL;
    while ((s = strstr(s, k)) != NULL) {
        size_t len = s - buf->ptr;

        if ((len == 0 || (len >1 && *(s-2) == '\r' && *(s-1) == '\n')) && *(s+kl) == ':' && *(s+kl+1) == ' ') {
            e = strstr(s + kl + 2, "\r\n");
            break;
        }

        s += kl;
    }

    if (e != NULL) {
        e += 2;
        memmove(s, e, buf->ptr + buf->len + 1 - e);
        buf->len -= e - s;
    }

    return true;
}

bool replace_header(buffer2_t *buf, const char *k, const char *v) {
    delete_header(buf, k);
    return append_header(buf, k, v);
}

bool append_str(buffer2_t *buf, const char *str) {
    size_t len;

    len = strlen(str);
    if (!reserve_str(buf, len)) {
        return false;
    }

    strcat(buf->ptr, str);
    buf->len += len;
    return true;
}

bool prepend_str(buffer2_t *buf, const char *str) {
    size_t len;

    len = strlen(str);
    if (!reserve_str(buf, len)) {
        return false;
    }

    memmove(buf->ptr + len, buf->ptr, buf->len + 1);
    memcpy(buf->ptr, str, len);
    buf->len += len;
    return true;
}

bool append_bytes(buffer2_t *buf, const char *bytes, size_t len) {
    if (!reserve(buf, len)) {
        return false;
    }

    memcpy(buf->ptr + buf->len, bytes, len);
    buf->len += len;
    return true;
}

void buffer_free(buffer2_t *buf) {
    assert((buf->ptr == NULL && buf->cap == 0) || (buf->ptr != NULL && buf->cap != 0) );
    free(buf->ptr);
    buf->ptr = NULL;
    buf->cap = 0;
    buf->len = 0;
}

bool append_buffer(buffer2_t *dist, buffer2_t *src) {
    if (!reserve(dist, src->len)) {
        return false;
    }

    memcpy(dist->ptr + dist->len, src->ptr, src->len);
    dist->len += src->len;
    return true;
}

static void test_delete_header() {
    buffer2_t buf = {0};

    delete_header(&buf, "k1");

    buffer_free(&buf);
}

static void test_replace_header() {
    buffer2_t buf = {0};
    
    replace_header(&buf, "k1", "v1");
    assert(strcmp("k1: v1\r\n", buf.ptr) == 0);
    
    buffer_free(&buf);
}

static void test_append_header_helper(buffer2_t *buf) {
    append_header(buf, "k1", "v1");
    assert(strcmp("k1: v1\r\n", buf->ptr) == 0);
    
    append_header(buf, "k2", "v2");
    assert(strcmp("k1: v1\r\nk2: v2\r\n", buf->ptr) == 0);
   
    append_header(buf, "k3", "v3");
    assert(strcmp("k1: v1\r\nk2: v2\r\nk3: v3\r\n", buf->ptr) == 0);
    
    append_header(buf, "k4", "v4");
    assert(strcmp("k1: v1\r\nk2: v2\r\nk3: v3\r\nk4: v4\r\n", buf->ptr) == 0);
}

static void test_replace_header_helper(buffer2_t *buf) {
    replace_header(buf, "k5", "v5");
    assert(strcmp("k1: v1\r\nk2: v2\r\nk3: v3\r\nk4: v4\r\nk5: v5\r\n", buf->ptr) == 0);

    replace_header(buf, "k2", "rk42");
    assert(strcmp("k1: v1\r\nk3: v3\r\nk4: v4\r\nk5: v5\r\nk2: rk42\r\n", buf->ptr) == 0);

    replace_header(buf, "k1", "kk21");
    assert(strcmp("k3: v3\r\nk4: v4\r\nk5: v5\r\nk2: rk42\r\nk1: kk21\r\n", buf->ptr) == 0);

    replace_header(buf, "k2", "hzw");
    assert(strcmp("k3: v3\r\nk4: v4\r\nk5: v5\r\nk1: kk21\r\nk2: hzw\r\n", buf->ptr) == 0);

    replace_header(buf, "k2", "ml");
    assert(strcmp("k3: v3\r\nk4: v4\r\nk5: v5\r\nk1: kk21\r\nk2: ml\r\n", buf->ptr) == 0);
}

static void test_delete_header_helper(buffer2_t *buf) {
    delete_header(buf, "k2");
    assert(strcmp("k3: v3\r\nk4: v4\r\nk5: v5\r\nk1: kk21\r\n", buf->ptr) == 0);

    delete_header(buf, "k2");
    assert(strcmp("k3: v3\r\nk4: v4\r\nk5: v5\r\nk1: kk21\r\n", buf->ptr) == 0);

    delete_header(buf, "k3");
    assert(strcmp("k4: v4\r\nk5: v5\r\nk1: kk21\r\n", buf->ptr) == 0);

    delete_header(buf, "k5");
    assert(strcmp("k4: v4\r\nk1: kk21\r\n", buf->ptr) == 0);

    delete_header(buf, "k4");
    assert(strcmp("k1: kk21\r\n", buf->ptr) == 0);

    delete_header(buf, "k1");
    assert(strcmp("", buf->ptr) == 0);

    delete_header(buf, "k1");
    assert(strcmp("", buf->ptr) == 0);
}

static void test_op_header() {
    buffer2_t buf = {0};

    test_append_header_helper(&buf);
    test_replace_header_helper(&buf);
    test_delete_header_helper(&buf);

    test_append_header_helper(&buf);
    test_replace_header_helper(&buf);
    test_delete_header_helper(&buf);

    buffer_free(&buf);
}

static void test_edge_case() {
    buffer2_t buf = {0};

    append_header(&buf, "k1", "k2: v2");
    assert(strcmp("k1: k2: v2\r\n", buf.ptr) == 0);
    
    append_header(&buf, "k2", "value2");
    assert(strcmp("k1: k2: v2\r\nk2: value2\r\n", buf.ptr) == 0);
    
    replace_header(&buf, "k2", "replace2");
    assert(strcmp("k1: k2: v2\r\nk2: replace2\r\n", buf.ptr) == 0);
    
    replace_header(&buf, "k1", "r1");
    assert(strcmp("k2: replace2\r\nk1: r1\r\n", buf.ptr) == 0);

    buffer_free(&buf);
}

static void test_append_str() {
    buffer2_t buf = {0};

    append_str(&buf, "first line\r\n");
    assert(strcmp("first line\r\n", buf.ptr) == 0);
    append_header(&buf, "k1", "k2: v2");
    assert(strcmp("first line\r\nk1: k2: v2\r\n", buf.ptr) == 0);
    append_header(&buf, "k2", "value2");
    assert(strcmp("first line\r\nk1: k2: v2\r\nk2: value2\r\n", buf.ptr) == 0);
    replace_header(&buf, "k2", "replace2");
    assert(strcmp("first line\r\nk1: k2: v2\r\nk2: replace2\r\n", buf.ptr) == 0);
    replace_header(&buf, "k1", "r1");
    assert(strcmp("first line\r\nk2: replace2\r\nk1: r1\r\n", buf.ptr) == 0);

    buffer_free(&buf);
}

static void test_edge_case1() {
    buffer2_t buf = {0};

    append_str(&buf, "first line k1: v1 k2: v2, k3: v3\r\n");
    assert(strcmp("first line k1: v1 k2: v2, k3: v3\r\n", buf.ptr) == 0);
    
    replace_header(&buf, "k1", "v1");
    assert(strcmp("first line k1: v1 k2: v2, k3: v3\r\nk1: v1\r\n", buf.ptr) == 0);
    replace_header(&buf, "k2", "v2");
    assert(strcmp("first line k1: v1 k2: v2, k3: v3\r\nk1: v1\r\nk2: v2\r\n", buf.ptr) == 0);
    replace_header(&buf, "k3", "v3");
    assert(strcmp("first line k1: v1 k2: v2, k3: v3\r\nk1: v1\r\nk2: v2\r\nk3: v3\r\n", buf.ptr) == 0);
    
    replace_header(&buf, "k1", "r1");
    assert(strcmp("first line k1: v1 k2: v2, k3: v3\r\nk2: v2\r\nk3: v3\r\nk1: r1\r\n", buf.ptr) == 0);
    replace_header(&buf, "k2", "r2");
    assert(strcmp("first line k1: v1 k2: v2, k3: v3\r\nk3: v3\r\nk1: r1\r\nk2: r2\r\n", buf.ptr) == 0);
    replace_header(&buf, "k3", "r3");
    assert(strcmp("first line k1: v1 k2: v2, k3: v3\r\nk1: r1\r\nk2: r2\r\nk3: r3\r\n", buf.ptr) == 0);

    delete_header(&buf, "k1");
    assert(strcmp("first line k1: v1 k2: v2, k3: v3\r\nk2: r2\r\nk3: r3\r\n", buf.ptr) == 0);
    delete_header(&buf, "k2");
    assert(strcmp("first line k1: v1 k2: v2, k3: v3\r\nk3: r3\r\n", buf.ptr) == 0);
    delete_header(&buf, "k3");
    assert(strcmp("first line k1: v1 k2: v2, k3: v3\r\n", buf.ptr) == 0);

    buffer_free(&buf);
}

static void test_prepend_str() {
    buffer2_t buf = {0};

    replace_header(&buf, "key1", "value1");
    assert(strcmp("key1: value1\r\n", buf.ptr) == 0);
    prepend_str(&buf, "first line \r\n");
    assert(strcmp("first line \r\nkey1: value1\r\n", buf.ptr) == 0);

    buffer_free(&buf);
}

static void test_prepend_str1() {
    buffer2_t buf = {0};

    prepend_str(&buf, "first line \r\n");
    assert(strcmp("first line \r\n", buf.ptr) == 0);

    buffer_free(&buf);
}

static void test_append_bytes() {
    buffer2_t buf = {0};

    append_bytes(&buf, "hello, world!", 13);
    assert(buf.len == 13);
    assert(memcmp("hello, world!", buf.ptr, buf.len) == 0);

    append_bytes(&buf, "h", 1);
    assert(buf.len == 14);
    assert(memcmp("hello, world!h", buf.ptr, buf.len) == 0);

    append_bytes(&buf, "zw", 2);
    assert(buf.len == 16);
    assert(memcmp("hello, world!hzw", buf.ptr, buf.len) == 0);

    append_bytes(&buf, "\0\0\0", 3);
    assert(buf.len == 19);
    assert(memcmp("hello, world!hzw\0\0\0", buf.ptr, buf.len) == 0);

    append_bytes(&buf, "amazing", 7);
    assert(buf.len == 26);
    assert(memcmp("hello, world!hzw\0\0\0amazing", buf.ptr, buf.len) == 0);

    buffer_free(&buf);
}

static void test_str_bytes() {
    buffer2_t buf = {0};

    replace_header(&buf, "k1", "value1");
    assert(strcmp("k1: value1\r\n", buf.ptr) == 0);

    append_bytes(&buf, "hello, world!hahaha", 13);
    assert(memcmp("k1: value1\r\nhello, world!", buf.ptr, buf.len) == 0);

    replace_header(&buf, "k2", "value2");
    assert(strcmp("k1: value1\r\nhello, world!k2: value2\r\n", buf.ptr) == 0);

    buffer_free(&buf);

    append_bytes(&buf, "hello, world!hahaha", 13);
    assert(memcmp("hello, world!", buf.ptr, buf.len) == 0);

    prepend_str(&buf, "hzw");
    assert(strcmp("hzwhello, world!", buf.ptr) == 0);

    buffer_free(&buf);

    append_bytes(&buf, "hello, world!hahaha", 13);
    assert(memcmp("hello, world!", buf.ptr, buf.len) == 0);

    append_str(&buf, "hzw");
    assert(strcmp("hello, world!hzw", buf.ptr) == 0);

    buffer_free(&buf);

    append_bytes(&buf, "hello: world!\r\nhahaha", 15);
    assert(memcmp("hello: world!\r\n", buf.ptr, buf.len) == 0);
    
    replace_header(&buf, "hello", "hzw");
    assert(strcmp("hello: hzw\r\n", buf.ptr) == 0);

    buffer_free(&buf);
}

void test_buffer2() {
    test_delete_header();
    test_replace_header();
    test_op_header();
    test_edge_case();
    test_append_str();
    test_edge_case1();
    test_prepend_str();
    test_prepend_str1();
    test_append_bytes();
    test_str_bytes();
}
