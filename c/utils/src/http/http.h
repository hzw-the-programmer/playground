#if !defined(__HTTP_H__)
#define __HTTP_H__

#include "slice/slice.h"
#include "socket/sock.h"

#define CRNL "\r\n"
#define CRNL_LEN 2
#define COLON ":"
#define COLON_LEN 1

#define CONTENT_LENGTH "Content-Length"
#define CONTENT_LENGTH_LEN 14

#define FIRST_LINE 0x00
#define HEADERS 0x01
#define BODY 0x02
#define STATE_MASK 0x03

typedef struct http_ctx_s {
    void (*header_cb)(struct http_ctx_s*, slice_t*, slice_t*);
    void (*body_cb)(struct http_ctx_s*, slice_t*);
    uint32_t flags;
    uint64_t len;
} http_ctx_t;

void http_cb(sock_t *sock, void *arg);

#endif