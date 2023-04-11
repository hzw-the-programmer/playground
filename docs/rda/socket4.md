#define LOG_BUF_LEN 1024
#if defined(WIN32)
#define LOG_FILE_SEP_CHAR '\\'
#else
#define LOG_FILE_SEP_CHAR '/'
#endif
#define LOG_PREFIX "hzw"

static FS_HANDLE g_log = FS_INVALID_FILE_HANDLE;

#define LOG(fmt, ...) log(__FILE__, __LINE__, __FUNCTION__, fmt, ##__VA_ARGS__)

static void log(const char *file, int line, const char *func, const char *fmt, ...) {
	char buf[LOG_BUF_LEN];
	char *p, *file_p;
	int len, wlen, ret;
	va_list ap;
	MYTIME dt;

	if (g_log < 0) {
		g_log = MMI_FS_Open(L"D:/hzw.log", FS_CREATE_ALWAYS | FS_READ_WRITE);
		if (g_log < 0) {			
			mmi_trace(MMI_TRACE_LEVEL_1, LOG_PREFIX "create log file failed,g_log=%d", g_log);
			return;
		}
	}

	DTGetRTCTime(&dt);

	file_p = strrchr(file, LOG_FILE_SEP_CHAR);
	if (file_p) {
		file_p += 1;
	} else {
		file_p = file;
	}

	p = buf;
	len = sprintf(p, "[%d-%02d-%02d %02d:%02d:%02d][%s:%d:%s][%s]: ",
		dt.nYear, dt.nMonth, dt.nDay,
		dt.nHour, dt.nMin, dt.nSec,
		file_p, line, func,
		LOG_PREFIX);
	p += len;

	va_start(ap, fmt);
	len += vsnprintf(p, LOG_BUF_LEN, fmt, ap);
	va_end(ap);
	buf[len++] = '\n';
	buf[len] = '\0';
	ret = MMI_FS_Write(g_log, buf, len, &wlen);
	mmi_trace(MMI_TRACE_LEVEL_1, "%s", buf);
}

typedef enum {
	STATE_IDLE,
	STATE_ATTACHING,
	STATE_ATTACHED,
	STATE_ACTIVATING,
	STATE_ACTIVATED,
} state_t;

typedef struct net_ctx_s {
	CFW_SIM_ID sim;
	UINT8 cid;
	UINT8 *apn;
	state_t state;
	UINT32 ticks;
} net_ctx_t;

static net_ctx_t g_net_ctx;

static void draw(const char *text) {
    gdi_layer_clear(GDI_COLOR_WHITE);

    gui_set_font(&MMI_small_font);
    gui_set_text_color(gui_color(0, 0, 0));
	gui_move_text_cursor(50, 50);
	gui_printf("%s", text);

    gdi_layer_blt_previous(0, 0, UI_DEVICE_WIDTH-1, UI_DEVICE_HEIGHT-1);
}

static char* net_state2str(state_t state) {
	switch (state) {
		case STATE_IDLE:
			return "IDLE";
		case STATE_ATTACHING:
			return "ATTACHING";
		case STATE_ATTACHED:
			return "ATTACHED";
		case STATE_ACTIVATING:
			return "ACTIVATING";
		case STATE_ACTIVATED:
			return "ACTIVATED";
	}
}

static void net_set_state(net_ctx_t *ctx, state_t state) {
	ctx->state = state;
	draw(net_state2str(state));
}

static void net_gprs_activate_rsp(void *p) {
	net_ctx_t *ctx = &g_net_ctx;
	mmi_ps_gprs_connect_status_ind_struct *msg = (mmi_ps_gprs_connect_status_ind_struct*)p;

	LOG("status=0x%02x", msg->status);
	if (msg->status == 0x01) {
		net_set_state(ctx, STATE_ACTIVATED);
	}
}

static void net_gprs_deactivate_rsp(void *p) {
	net_ctx_t *ctx = &g_net_ctx;
	mmi_ps_gprs_deconnect_status_ind_struct *msg = (mmi_ps_gprs_deconnect_status_ind_struct*)p;

	LOG("status=0x%02x", msg->status);
	net_set_state(ctx, STATE_IDLE);
}

static void net_gprs_activate(net_ctx_t *ctx) {
	INT32 ret;
	UINT8 state;

	LOG("cid=%d", ctx->cid);
	if (!ctx->cid) {
		CFW_GetFreeCID(&ctx->cid, ctx->sim);
		LOG("cid=%d", ctx->cid);
	}

	CFW_GetGprsActState(ctx->cid, &state, ctx->sim);
	if (state == CFW_GPRS_ACTIVED) {
		LOG("CFW_GPRS_ACTIVED");
		net_set_state(ctx, STATE_ACTIVATED);
		return;
	}

	ret = soc_StartActiveGPRSExt(ctx->sim, ctx->cid, 
		ctx->apn, "", "");
	if (ret == 0) {
		net_set_state(ctx, STATE_ACTIVATING);
		ctx->ticks = csw_TMGetTick();
		SetProtocolEventHandler(net_gprs_activate_rsp, MSG_ID_MMI_NW_GPRS_CONNECTED_RSP);
		SetProtocolEventHandler(net_gprs_deactivate_rsp, MSG_ID_MMI_NW_GPRS_DECONNECTED_RSP);
	}
}

static void net_gprs_attach_rsp(void *p) {
	net_ctx_t *ctx = &g_net_ctx;
	mmi_ps_gprs_deconnect_status_ind_struct *msg = (mmi_ps_gprs_deconnect_status_ind_struct*)p;

	LOG("status=0x%02x", msg->status);
	if (msg->status == 0x01) {
		net_set_state(ctx, STATE_ATTACHED);
		net_gprs_activate(ctx);
	}
}

static void net_gprs_dettach_rsp(void *p) {
	net_ctx_t *ctx = &g_net_ctx;
	mmi_ps_gprs_deconnect_status_ind_struct *msg = (mmi_ps_gprs_deconnect_status_ind_struct*)p;

	LOG("status=0x%02x", msg->status);
	net_set_state(ctx, STATE_IDLE);
}

static void net_gprs_attach(net_ctx_t *ctx) {
	UINT32 ret;

	ret = adp_AttachGPRSExt(ctx->sim);
	if (ret == ADP_GPRS_ATTACH_ALREADY) {
		LOG("ADP_GPRS_ATTACH_ALREADY");
		net_gprs_activate(ctx);
	} else if (ret == ADP_GPRS_ATTACH_ASYN) {
		LOG("ADP_GPRS_ATTACH_ASYN");
		net_set_state(ctx, STATE_ATTACHING);
		ctx->ticks = csw_TMGetTick();
		SetProtocolEventHandler(net_gprs_attach_rsp, MSG_ID_MMI_NW_GPRS_ATTACH_RSP);
		SetProtocolEventHandler(net_gprs_dettach_rsp, MSG_ID_MMI_NW_GPRS_DETTACH_RSP);
	} else {
		LOG("ADP_GPRS_ATTACH_ERROR");
	}
}

static void lsk() {
}

static void key_1() {
}

static void key_2() {
	net_ctx_t *ctx = &g_net_ctx;

	LOG("sim=%d", ctx->sim);
	if (!ctx->apn) {
		ctx->apn = "CMWAP";
		ctx->sim = MTPNP_AD_GET_UsableSide_Index();
		LOG("sim=%d", ctx->sim);
	}

	net_gprs_attach(ctx);
}

static void key_3() {
	net_ctx_t *ctx = &g_net_ctx;

	LOG("");
	adp_StopGPRSExt(ctx->sim, ctx->cid);
}

static void key_4() {
}

static void key_5() {
}

static void key_6() {
}

static void key_7() {
}

static void register_handlers() {
    SetLeftSoftkeyFunction(lsk, KEY_EVENT_UP);
    SetRightSoftkeyFunction(GoBackHistory, KEY_EVENT_UP);
	SetKeyHandler(key_1, KEY_1, KEY_EVENT_DOWN);
	SetKeyHandler(key_2, KEY_2, KEY_EVENT_DOWN);
	SetKeyHandler(key_3, KEY_3, KEY_EVENT_DOWN);
	SetKeyHandler(key_4, KEY_4, KEY_EVENT_DOWN);
	SetKeyHandler(key_5, KEY_5, KEY_EVENT_DOWN);
	SetKeyHandler(key_6, KEY_6, KEY_EVENT_DOWN);
	SetKeyHandler(key_7, KEY_7, KEY_EVENT_DOWN);
}

static void exit_app() {
}

static void enter_app() {
	LOG("%d", sxr_GetCurrentTaskId());

	EntryNewScreen(666, exit_app, enter_app, NULL);
    // no history
    // ExecuteCurrExitHandler_Ext -> GenericExitScreen
    //EntryNewScreen(666, exit_app, NULL, NULL);

	register_handlers();
	draw("");
}
