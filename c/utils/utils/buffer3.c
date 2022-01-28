#include <stdlib.h>
#include <string.h>
#include <assert.h>

typedef struct {
    int cap;
    int r;
    int w;
    unsigned char ptr[sizeof(int)];
} buf3;

void buf3_init(buf3 **buf, int cap) {
    int size;

    assert(buf != 0);
    
    size = sizeof(buf3) + cap - 4;
    *buf = malloc(size);
    if (*buf == 0) {
        return;
    }
    memset(*buf, 0x00, size);
    (*buf)->cap = cap;
}

void buf3_deinit(buf3 **buf) {
    free(*buf);
    *buf = 0;
}

int buf3_len(buf3 *buf) {
    return buf->w - buf->r;
}

int buf3_available(buf3 *buf) {
    return buf->cap - buf->w;
}

void buf3_reset(buf3 *buf) {
    buf->r = buf->w = 0;
}

void buf3_tidy(buf3 *buf) {
    int len;
    
    if (buf->r == 0) {
        return;
    }

    len = buf3_len(buf);

    if (len == 0) {
        buf3_reset(buf);
        return;
    }

    memmove(buf->ptr, buf->ptr + buf->r, len);
    buf->w -= buf->r;
    buf->r = 0;
}

int buf3_write(buf3 *buf, unsigned char *data, int len) {
    int l;

    if (len == 0) {
        return 0;
    }

    l = buf3_available(buf);
    if (l < len) {
        buf3_tidy(buf);
        l = buf3_available(buf);
    }
    if (l == 0) {
        return 0;
    }

    if (l > len) {
        l = len;
    }

    memcpy(buf->ptr + buf->w, data, l);
    buf->w += l;

    return l;
}

int buf3_read(buf3 *buf, unsigned char *data, int len) {
    int l;

    if (len == 0) {
        return 0;
    }

    l = buf3_len(buf);
    if (l == 0) {
        return 0;
    }
    if (l > len) {
        l = len;
    }

    memcpy(data, buf->ptr + buf->r, l);
    buf->r += l;

    return l;
}

static void test_buf3_1() {
    buf3 *buf;
    unsigned char rb[10];
    int i;

    buf3_init(&buf, 8);

    for (i = 0; i < 100; i++) {
        assert(1 == buf3_write(buf, "a", 1));
        assert(memcmp("a", buf->ptr, buf3_len(buf)) == 0);
        assert(1 == buf3_write(buf, "b", 1));
        assert(memcmp("ab", buf->ptr, buf3_len(buf)) == 0);
        assert(1 == buf3_write(buf, "c", 1));
        assert(memcmp("abc", buf->ptr, buf3_len(buf)) == 0);

        assert(3 == buf3_write(buf, "def", 3));
        assert(memcmp("abcdef", buf->ptr, buf3_len(buf)) == 0);

        assert(2 == buf3_write(buf, "ghi", 3));
        assert(memcmp("abcdefgh", buf->ptr, buf3_len(buf)) == 0);

        assert(0 == buf3_write(buf, "jkl", 3));
        assert(memcmp("abcdefgh", buf->ptr, buf3_len(buf)) == 0);

        assert(1 == buf3_read(buf, rb, 1));
        assert(memcmp("a", rb, 1) == 0);
        assert(1 == buf3_read(buf, rb + 1, 1));
        assert(memcmp("ab", rb, 2) == 0);
        assert(1 == buf3_read(buf, rb + 2, 1));
        assert(memcmp("abc", rb, 3) == 0);
        assert(3 == buf3_read(buf, rb + 3, 3));
        assert(memcmp("abcdef", rb, 6) == 0);
        assert(2 == buf3_read(buf, rb + 6, 3));
        assert(memcmp("abcdefgh", rb, 8) == 0);
    }

    buf3_deinit(&buf);
}

static void test_buf3_2() {
    buf3 *buf;
    unsigned char rb[10];

    buf3_init(&buf, 8);

    assert(buf3_read(buf, rb, 10) == 0);
    assert(buf3_write(buf, "abcde", 5) == 5);
    assert(memcmp("abcde", buf->ptr, buf3_len(buf)) == 0);
    assert(buf3_read(buf, rb, 10) == 5);
    assert(memcmp("abcde", rb, 5) == 0);
    assert(buf3_write(buf, "abcdefghijk", 11) == 8);
    assert(buf3_read(buf, rb, 1) == 1);
    assert(memcmp("a", rb, 1) == 0);
    assert(buf3_write(buf, "abcdefghijk", 11) == 1);
    assert(memcmp("bcdefgha", buf->ptr, buf3_len(buf)) == 0);
    assert(buf3_read(buf, rb, 2) == 2);
    assert(memcmp("bc", rb, 2) == 0);
    assert(buf3_write(buf, "bcdefghijk", 10) == 2);
    assert(memcmp("defghabc", buf->ptr, buf3_len(buf)) == 0);

    assert(buf3_read(buf, rb, 3) == 3);
    assert(memcmp("def", rb, 3) == 0);
    assert(buf3_write(buf, "defghijklm", 10) == 3);
    assert(memcmp("ghabcdef", buf->ptr, buf3_len(buf)) == 0);

    assert(buf3_read(buf, rb, 4) == 4);
    assert(memcmp("ghab", rb, 4) == 0);
    assert(buf3_write(buf, "ghijklmnop", 10) == 4);
    assert(memcmp("cdefghij", buf->ptr, buf3_len(buf)) == 0);

    buf3_deinit(&buf);
}

void test_buf3() {
    test_buf3_1();
    test_buf3_2();
}
