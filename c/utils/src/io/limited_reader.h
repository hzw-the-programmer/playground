#if !defined(__LIMITED_READER_H__)
#define __LLIMITED_READER_H__

#include "buffer/buffer.h"

typedef struct {
    buf_t *buf;
    int len;
} limited_reader_t;

int limited_reader_read(void *arg, slice_t *slice);

#endif