#if !defined(__SOCK_MOCK_H__)
#define __SOCK_MOCK_H__

#include "buffer/buffer.h"

typedef struct {
    buf_t *buf;
    int n;
} sock_t;

typedef struct {
    sock_t *sock;
    int nsock;
} sock_ctx_t;

sock_ctx_t* sock_ctx_new(int nsock, int cap);
void sock_ctx_free(sock_ctx_t *ctx);

int sock_recv(sock_t *sock, uint8_t *out, int len);

#endif
