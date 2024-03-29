#if !defined(__LEN_READER_WRITER_H__)
#define __LEN_READER_WRITER_H__

#include "buffer/buffer.h"

#define LEN_SIZE_1 0x00
#define LEN_SIZE_2 0x01
#define LEN_SIZE_4 0x02
#define LEN_SIZE_MASK 0x03

#define LEN_READED 0x04

#define LEN_SIZE(rw) (1<<((rw)->flags&LEN_SIZE_MASK))

typedef struct {
    buf_t *buf;
    uint32_t flags;
} len_writer_t;

typedef struct {
    buf_t *buf;
    uint32_t flags;
    int len;
} len_reader_t;

int len_writer_write(void *arg, slice_t slice);
int len_reader_read(void *arg, slice_t *slice);

#endif
