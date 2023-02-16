#include <string.h>
#include <assert.h>

#define MAX_MSGS (10)

struct ctx_s {
    void (*cb)(void *arg, int id);
    void *arg;
    int len;
    int msgs[MAX_MSGS];
};

int post_msg(struct ctx_s *ctx, int id) {
    int empty;
    
    if (ctx->len >= MAX_MSGS) {
        return 0;
    }
    
    empty = ctx->len == 0;
    if (!empty && ctx->msgs[ctx->len-1] == id) {
        return 0;
    }
    ctx->msgs[ctx->len++] = id;
    
    return empty;
}

void exe_msgs(struct ctx_s *ctx) {
    int len, i;
    int msgs[MAX_MSGS];

    len = ctx->len;
    memcpy(msgs, ctx->msgs, sizeof(ctx->msgs));
    ctx->len = 0;
    memset(ctx->msgs, 0, sizeof(ctx->msgs));

    for (i = 0; i < len; i++) {
        ctx->cb(ctx->arg, msgs[i]);
    }
}

void test_msg2_1_cb(void *arg, int id) {
    struct {
        int len;
        int msgs[MAX_MSGS];
    } *msgs = arg;

    msgs->msgs[msgs->len++] = id;
}

void test_msg2_1_cb_2(void *arg, int id) {
    struct ctx_s *ctx = arg;

    ctx->msgs[ctx->len++] = id;
}

void test_msg2_1() {
    struct {
        int len;
        int msgs[MAX_MSGS];
    } arg = {0};
    struct ctx_s ctx = {test_msg2_1_cb, &arg, 0, {0}};

    // 1
    assert(post_msg(&ctx, 0) == 1);
    assert(ctx.len == 1);
    assert(ctx.msgs[0] == 0);

    assert(post_msg(&ctx, 0) == 0);
    assert(ctx.len == 1);
    assert(ctx.msgs[0] == 0);

    assert(post_msg(&ctx, 2) == 0);
    assert(ctx.len == 2);
    assert(ctx.msgs[0] == 0);
    assert(ctx.msgs[1] == 2);

    assert(post_msg(&ctx, 2) == 0);
    assert(ctx.len == 2);
    assert(ctx.msgs[0] == 0);
    assert(ctx.msgs[1] == 2);

    assert(post_msg(&ctx, 0) == 0);
    assert(ctx.len == 3);
    assert(ctx.msgs[0] == 0);
    assert(ctx.msgs[1] == 2);
    assert(ctx.msgs[2] == 0);

    exe_msgs(&ctx);
    assert(ctx.len == 0);
    assert(arg.len == 3);
    assert(arg.msgs[0] == 0);
    assert(arg.msgs[1] == 2);
    assert(arg.msgs[2] == 0);

    // 2
    assert(post_msg(&ctx, 0) == 1);
    assert(ctx.len == 1);
    assert(ctx.msgs[0] == 0);

    assert(post_msg(&ctx, 0) == 0);
    assert(ctx.len == 1);
    assert(ctx.msgs[0] == 0);

    assert(post_msg(&ctx, 2) == 0);
    assert(ctx.len == 2);
    assert(ctx.msgs[0] == 0);
    assert(ctx.msgs[1] == 2);

    assert(post_msg(&ctx, 2) == 0);
    assert(ctx.len == 2);
    assert(ctx.msgs[0] == 0);
    assert(ctx.msgs[1] == 2);

    assert(post_msg(&ctx, 0) == 0);
    assert(ctx.len == 3);
    assert(ctx.msgs[0] == 0);
    assert(ctx.msgs[1] == 2);
    assert(ctx.msgs[2] == 0);

    ctx.cb = test_msg2_1_cb_2;
    ctx.arg = &ctx;
    exe_msgs(&ctx);
    assert(ctx.len == 3);
    assert(ctx.msgs[0] == 0);
    assert(ctx.msgs[1] == 2);
    assert(ctx.msgs[2] == 0);
}

void test_msg2() {
    test_msg2_1();
}