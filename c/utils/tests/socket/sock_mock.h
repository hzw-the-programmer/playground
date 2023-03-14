#if !defined(__SOCK_MOCK_H__)
#define __SOCK_MOCK_H__

#include "buffer/buffer.h"

typedef struct {
    buf_t *buf;
    int n;
} mock_sock_t;

typedef struct {
    mock_sock_t *sock;
    int nsock;
} mock_sock_ctx_t;

mock_sock_ctx_t* mock_sock_ctx_new(int nsock, int cap);
void mock_sock_ctx_free(mock_sock_ctx_t *ctx);

int mock_sock_recv(mock_sock_t *sock, uint8_t *out, int len);

#endif
