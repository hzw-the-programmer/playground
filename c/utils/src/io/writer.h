#if !defined(__WRITER_H__)
#define __WRITER_H__

#include "buffer/buffer.h"

typedef struct {
    int len;
    buf_t *buf;
} len_writer;

int len_writer_write(len_writer *w, const uint8_t *p, int plen, int *len);

#endif
