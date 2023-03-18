#if !defined(__SOCK_H__)
#define __SOCK_H__

#include "buffer/buffer.h"

typedef struct sock_s {
    int fd;
    buf_t *send_buf;
    buf_t *recv_buf;
    void *ctx;
    void (*cb)(struct sock_s*, void *arg);
} sock_t;

#endif