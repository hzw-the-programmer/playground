#include <stdlib.h>
#include <assert.h>
#include "mem/mem.h"
#include "sock_mock.h"

mock_sock_ctx_t* mock_sock_ctx_new(int nsock, int cap) {
    mock_sock_ctx_t *ctx;
    int i;

    ctx = malloc(sizeof(*ctx));
    assert(ctx);

    ctx->sock = malloc(sizeof(ctx->sock) * nsock);
    assert(ctx->sock);
    ctx->nsock = nsock;

    for (i = 0; i < nsock; i++) {
        ctx->sock[i].buf = buf_new(cap);
        assert(ctx->sock[i].buf);
        ctx->sock[i].n = 1;
    }

    return ctx;
}

void mock_sock_ctx_free(mock_sock_ctx_t *ctx) {
    int i;

    for (i = 0; i < ctx->nsock; i++) {
        free(ctx->sock[i].buf);
    }
    free(ctx->sock);
    free(ctx);
}

int mock_sock_recv(mock_sock_t *sock, uint8_t *out, int len) {
    if (len > sock->n) {
        len = sock->n;
    }
    return buf_read(sock->buf, out, len);
}
