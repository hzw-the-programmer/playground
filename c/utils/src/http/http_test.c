#include <string.h>
#include <stdio.h>
#include <assert.h>

#include "utils.h"
#include "mem/mem.h"
#include "buffer/buffer.h"
#include "io/sep_reader_writer.h"
#include "socket/sock.h"
#include "socket/sock_mock.h"
#include "http/http.h"

#define KV_SEP ": "
#define KV_SEP_LEN 2

#define MAX_BUF 1024

static char *first_line_1 = "HTTP/1.1 200 OK";
static char* kvs_1[] = {
    "Accept-Ranges", "bytes",
    "Cache-Control", "private, no-cache, no-store, proxy-revalidate, no-transform",
    "Connection", "keep-alive",
    "Content-Type", "text/html",
    "Date", "Sat, 11 Mar 2023 08:52:44 GMT",
    "Etag", "\"588604f8-94d\"",
    "Last-Modified", "Mon, 23 Jan 2017 13:28:24 GMT",
    "Pragma", "no-cache",
    "Server", "bfe/1.0.8.18",
    "Set-Cookie", "BDORZ=27315; max-age=86400; domain=.baidu.com; path=/",
};
static char *body_1 = "hello\r\nhzw";

static void resp_write(buf_t *buf,
    const char *first_line, const char **kvs, int kvs_len, const uint8_t *body, int body_len) {
    sep_writer_t w;
    slice_t slice;
    int i;
    char content_len[128];

    w.buf = buf;

    w.sep.ptr = CRNL;
    w.sep.len = CRNL_LEN;
    slice.ptr = (uint8_t*)first_line;
    slice.len = strlen(first_line);
    sep_writer_write(&w, slice);

    for (i = 0; i < kvs_len; i+=2) {
        w.sep.ptr = KV_SEP;
        w.sep.len = KV_SEP_LEN;
        slice.ptr = (uint8_t*)kvs[i];
        slice.len = strlen(kvs[i]);
        sep_writer_write(&w, slice);

        w.sep.ptr = CRNL;
        w.sep.len = CRNL_LEN;
        slice.ptr = (uint8_t*)kvs[i+1];
        slice.len = strlen(kvs[i+1]);
        sep_writer_write(&w, slice);
    }

    if (!body) {
        slice.len = 0;
        sep_writer_write(&w, slice);
        return;
    }
    
    sprintf(content_len, CONTENT_LENGTH ": %d", body_len);
    slice.ptr = content_len;
    slice.len = strlen(content_len);
    sep_writer_write(&w, slice);

    slice.len = 0;
    sep_writer_write(&w, slice);

    slice.ptr = (uint8_t*)body;
    slice.len = body_len;
    buf_write(buf, slice);
}

static void resp_write_test() {
    char *want =
        "HTTP/1.1 200 OK\r\n"
        "Accept-Ranges: bytes\r\n"
        "Cache-Control: private, no-cache, no-store, proxy-revalidate, no-transform\r\n"
        "Connection: keep-alive\r\n"
        "Content-Type: text/html\r\n"
        "Date: Sat, 11 Mar 2023 08:52:44 GMT\r\n"
        "Etag: \"588604f8-94d\"\r\n"
        "Last-Modified: Mon, 23 Jan 2017 13:28:24 GMT\r\n"
        "Pragma: no-cache\r\n"
        "Server: bfe/1.0.8.18\r\n"
        "Set-Cookie: BDORZ=27315; max-age=86400; domain=.baidu.com; path=/\r\n"
        "Content-Length: 10\r\n"
        "\r\n"
        "hello\r\nhzw";
    int pool[MAX_BUF];
    buf_t *buf = buf_static((uint8_t*)pool, sizeof(pool));
    int len = strlen(want);

    resp_write(buf, first_line_1, kvs_1, ARRAY_SIZE(kvs_1), body_1, strlen(body_1));
    assert(buf_buffered(buf) == strlen(want));
    assert(!strncmp(want, buf_read_ptr(buf), buf_buffered(buf)));
}

typedef struct {
    http_ctx_t ctx;
    int count;
} http_test_ctx_t;

void header_test_cb(http_ctx_t *ctx, slice_t *k, slice_t *v) {
    http_test_ctx_t *tctx = (http_test_ctx_t*)ctx;
    char *key = kvs_1[tctx->count++];
    char *value = kvs_1[tctx->count++];

    assert(!strncmp(key, k->ptr, strlen(key)));
    assert(!strncmp(value, v->ptr, strlen(value)));
}

void body_test_cb(http_ctx_t *ctx, slice_t *slice) {
    http_test_ctx_t *tctx = (http_test_ctx_t*)ctx;

    assert(tctx->count == ARRAY_SIZE(kvs_1));
    assert(!strncmp(body_1, slice->ptr, strlen(body_1)));
}

static void http_test_1() {
    mock_sock_ctx_t *mctx;
    mock_sock_t *msock;
    sock_t sock;
    http_test_ctx_t ctx;
    int n, j; 

    mctx = mock_sock_ctx_new(2, MAX_BUF);
    msock = &mctx->sock[0];
    resp_write(msock->buf, first_line_1, kvs_1, ARRAY_SIZE(kvs_1), body_1, strlen(body_1));

    ctx.ctx.header_cb = header_test_cb;
    ctx.ctx.body_cb = body_test_cb;
    sock.cb = http_cb;
    sock.ctx = &ctx;
    sock.recv_buf = buf_new(MAX_BUF);
    assert(sock.recv_buf);

    for (j = 1; j <= MAX_BUF; j++) {
        msock->buf->r = 0;
        msock->n = j;
        sock.recv_buf->w = 0;
        ctx.ctx.flags = 0;
        ctx.count = 0;
        
        while (1) {
            n = mock_sock_recv(msock, buf_write_ptr(sock.recv_buf), buf_available(sock.recv_buf));
            if (n > 0) {
                buf_write_inc(sock.recv_buf, n);
                sock.cb(&sock, sock.ctx);
            } else {
                assert(buf_buffered(sock.recv_buf) == 0);
                break;
            }
        }
    }

    mock_sock_ctx_free(mctx);
    free(sock.recv_buf);
}

void http_test() {
    resp_write_test();
    http_test_1();
}
