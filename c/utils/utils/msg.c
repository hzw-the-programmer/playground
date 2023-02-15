#include "assert.h"

#define MAX_MSGS (sizeof(int)<<3)

struct ctx_s {
    int msg;
    void *arg;
    void (*cb)(void *arg, int id);
};

static int post_msg(struct ctx_s *ctx, int id) {
    int empty = ctx->msg == 0;
    ctx->msg |= 1<<id;
    return empty;
}

static void exe_msg(struct ctx_s *ctx) {
    int i, msg;

    msg = ctx->msg;
    ctx->msg = 0;
    for (i = 0; i < MAX_MSGS; i++) {
        if (msg&(1<<i)) {
            ctx->cb(ctx->arg, i);
        }
    }
}

static void test_cb_1(void *arg, int id) {
    int *d = arg;

    *d |= 1<<id;
}

static void test_cb_2(void *arg, int id) {
    struct ctx_s *ctx = arg;

    ctx->msg |= 1<<id;
}

void test_msg_1() {
    int arg = 0;
    struct ctx_s ctx = {0, &arg, test_cb_1};

    assert(post_msg(&ctx, 0) == 1);
    assert(ctx.msg == 0x01);
    assert(post_msg(&ctx, 0) == 0);
    assert(ctx.msg == 0x01);
    assert(post_msg(&ctx, 1) == 0);
    assert(ctx.msg == 0x03);
    assert(post_msg(&ctx, 2) == 0);
    assert(ctx.msg == 0x07);

    exe_msg(&ctx);
    assert(ctx.msg == 0x00);
    assert(arg == 0x07);

    ctx.arg = &ctx;
    ctx.cb = test_cb_2;

    assert(post_msg(&ctx, 0) == 1);
    assert(ctx.msg == 0x01);
    assert(post_msg(&ctx, 0) == 0);
    assert(ctx.msg == 0x01);
    assert(post_msg(&ctx, 1) == 0);
    assert(ctx.msg == 0x03);
    assert(post_msg(&ctx, 2) == 0);
    assert(ctx.msg == 0x07);

    exe_msg(&ctx);
    assert(ctx.msg == 0x07);
}

void test_msg() {
    test_msg_1();
}