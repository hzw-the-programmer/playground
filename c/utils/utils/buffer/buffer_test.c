#include "buffer.h"
#include <assert.h>

void buffer_test() {
    char *data;
    h_buf buf = {0};
    
    data = "GET /chat HTTP/1.1\r\n";
    h_buf_write(&buf, data, strlen(data));
    assert(buf.cap == strlen(data));
    assert(buf.len == strlen(data));
    assert(strncmp(buf.data, data, buf.len) == 0);
}