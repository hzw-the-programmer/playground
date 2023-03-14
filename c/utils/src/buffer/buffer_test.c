#include <assert.h>
#include "buffer/buffer.h"

static void buf_test() {
    int pool[(sizeof(buf_t)+16)/sizeof(int)];
    buf_t *buf = (buf_t*)pool;
    uint8_t i;
    
    buf->cap = 16;
    buf->w = buf->r = 0;
    buf->ptr = (uint8_t*)pool + sizeof(buf_t);

    for (i = 0; i < 128; i++) {
        int n = 1;
        if (i > 15) n = 0;
        assert(buf_write(buf, &i, 1) == n);
    }
}

void buffer_test() {
    buf_test();
}
