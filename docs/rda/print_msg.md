#define MAX_MSGS 10
#define MSG_MAX_SIZE 16

typedef struct {
	UINT8 msgs[MAX_MSGS][MSG_MAX_SIZE];
	UINT8 start;
} msg_ctx_t;

static msg_ctx_t g_msg_ctx;

static void push_msg(msg_ctx_t *ctx, const UINT8 *msg) {
    if (ctx->start == MAX_MSGS) {
        memcpy(&ctx->msgs[0], &ctx->msgs[1], MSG_MAX_SIZE*(MAX_MSGS-1));
        strncpy(ctx->msgs[ctx->start-1], msg, MSG_MAX_SIZE);
    } else {
        strncpy(ctx->msgs[ctx->start++], msg, MSG_MAX_SIZE);
    }
}

static void draw_msgs(const msg_ctx_t *ctx) {
    int i, x = 0, y = 0;

    gdi_layer_clear(GDI_COLOR_WHITE);

    gui_set_font(&MMI_small_font);
    gui_set_text_color(gui_color(0, 0, 0));

    for (i = 0; i < ctx->start; i++) {
        gui_move_text_cursor(x, y);
        gui_printf("%s", ctx->msgs[i]);
        y += 15;
    }

    gdi_layer_blt_previous(0, 0, UI_DEVICE_WIDTH-1, UI_DEVICE_HEIGHT-1);
}

static void push_msg_and_draw(msg_ctx_t *ctx, const UINT8 *msg) {
    push_msg(ctx, msg);
    draw_msgs(ctx);
}