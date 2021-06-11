#define NULL 0

#define MAX_HANDLERS 10

typedef struct context_s context_t;
typedef void handler_t(context_t*);

struct context_s {
    int index;
    handler_t *handlers[MAX_HANDLERS];
};

static void register_handler(context_t *ctx, handler_t *handler) {
    int i;
    for (i = 0; i < MAX_HANDLERS; i++) {
        if (ctx->handlers[i] == NULL) {
            ctx->handlers[i] = handler;
            break;
        }
    }
}

static void reset(context_t *ctx) {
    memset(ctx, 0, sizeof(context_t));
}

static void next(context_t *ctx) {
    while (ctx->handlers[ctx->index]) {
        ctx->index++;
        ctx->handlers[ctx->index-1](ctx);
    }
}

void with_next(context_t *ctx) {
    printf("with_next pre, index=%d\n", ctx->index - 1);
    next(ctx);
    printf("with_next post, index=%d\n", ctx->index - 1);
}

void without_next(context_t *ctx) {
    printf("without_next, index=%d\n", ctx->index - 1);
}

void test_mw() {
    context_t ctx = {0};

     printf("***mw01***\n");
    register_handler(&ctx, with_next);
    register_handler(&ctx, with_next);
    register_handler(&ctx, with_next);
    register_handler(&ctx, with_next);
    next(&ctx);

    printf("***mw02***\n");
    reset(&ctx);
    register_handler(&ctx, without_next);
    register_handler(&ctx, without_next);
    register_handler(&ctx, without_next);
    register_handler(&ctx, without_next);
    next(&ctx);

    printf("***mw03***\n");
    reset(&ctx);
    register_handler(&ctx, with_next);
    register_handler(&ctx, without_next);
    register_handler(&ctx, with_next);
    register_handler(&ctx, without_next);
    next(&ctx);
}
