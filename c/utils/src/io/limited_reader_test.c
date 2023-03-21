#include <string.h>
#include "buffer/buffer.h"
#include "mem/mem.h"

void limited_reader_test() {
    char *data = "0123456789abcdefghijklmnopqrstuvwxyz";
    buf_t *buf = buf_new(1024);

    buf_write(buf, slice_new(data, strlen(data)));
    free(buf);
}