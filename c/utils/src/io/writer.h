#if !defined(__WRITER_H__)
#define __WRITER_H__

#include "buffer/buffer.h"

#define LEN_SIZE_MASK 0x03
#define LEN_SIZE_1 0x00
#define LEN_SIZE_2 0x01
#define LEN_SIZE_4 0x02

#define LEN_SIZE(w) (1<<((w)->flags&LEN_SIZE_MASK))

typedef struct {
    buf_t *buf;
    uint8_t flags;
} len_writer;

int len_writer_write(len_writer *w, const uint8_t *p, int plen, int *len);

#endif
