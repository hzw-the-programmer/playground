#include "circular_buffer.h"

circular_buffer_t circular_buffer_new(int cap) {
    circular_buffer_t buf = {0};

    buf.data = malloc(cap);
    buf.cap = cap;
}

