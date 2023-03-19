#include <string.h>

#include "http.h"
#include "socket/sock.h"
#include "io/sep_reader_writer.h"
#include "io/len_reader_writer.h"
#include "slice/split.h"

static int parse_header(http_ctx_t *ctx, buf_t *buf) {
    sep_reader_t r;
    slice_t slice;

    r.buf = buf;
    r.sep.data = CRNL;
    r.sep.len = CRNL_LEN;

    sep_reader_read(&r, &slice);
    
    if (slice.len) {
        if ((ctx->flags&STATE_MASK) == FIRST_LINE) {
            ctx->flags = HEADERS;
        } else {
            split_t split;
            slice_t k, v;

            split = split_new(slice, slice_new(COLON, COLON_LEN));
            
            k = slice_trim_space(split_next(&split));
            v = slice_trim_space(split_next(&split));

            if (k.len == CONTENT_LENGTH_LEN &&
                !strncmp(k.data, CONTENT_LENGTH, k.len)) {
                ctx->len = slice_to_uint64(v);
            } else if (ctx->header_cb) {
                ctx->header_cb(ctx, &k, &v);
            }
        }
        
        return 1;
    }
    
    if (slice.data) {
        ctx->flags = BODY;
        return 1;
    }

    buf_tidy(buf);
    return 0;
}

static int parse_body(http_ctx_t *ctx, buf_t *buf) {
    len_reader_t r;
    slice_t slice;

    r.buf = buf;
    r.flags = LEN_READED;
    r.len = ctx->len;

    if (len_reader_read(&r, &slice) == -1) {
        return 0;
    }
    buf_tidy(buf);

    if (ctx->body_cb) {
        ctx->body_cb(ctx, &slice);
    }

    return 0;
}

void http_cb(sock_t *sock, void *arg) {
    http_ctx_t *ctx = arg;
    int ret = 1;

    while (ret) {
        switch (ctx->flags&STATE_MASK) {
            case FIRST_LINE:
            case HEADERS:
                ret = parse_header(ctx, sock->recv_buf);
                break;
            case BODY:
                ret = parse_body(ctx, sock->recv_buf);
                break;
        }
    }
}
