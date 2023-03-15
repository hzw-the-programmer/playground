#include <assert.h>
#include "buffer/buffer.h"

static buf_t* buf_static(uint8_t *ptr, int len) {
    buf_t *buf = NULL;

    buf = (buf_t*)ptr;
    buf->cap = len - sizeof(*buf);
    buf->w = buf->r = 0;
    buf->ptr = ptr + sizeof(*buf);

    return buf;
}

static void buf_test() {
    int pool[(sizeof(buf_t)+16)/sizeof(int)];
    buf_t *buf = buf_static((uint8_t*)pool, sizeof(pool));
    uint8_t i;

    for (i = 0; i < 128; i++) {
        int n = 1;
        if (i > 15) {
            n = 0;
        }
        assert(buf_write(buf, &i, 1) == n);
    }
}

void buffer_test() {
    buf_test();
}
