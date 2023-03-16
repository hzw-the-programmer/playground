#if !defined(__SEP_READER_WRITER_H__)
#define __SEP_READER_WRITER_H__

#include "buffer/buffer.h"
#include "slice/slice.h"

typedef struct {
    buf_t *buf;
    slice_t sep;
} sep_writer_t;

int sep_writer_write(void *arg, const slice_t *slice);

#endif
