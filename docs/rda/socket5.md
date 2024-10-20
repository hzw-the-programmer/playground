typedef enum {
	STATE_IDLE,
	STATE_ATTACHING,
	STATE_ATTACHED,
	STATE_ACTIVATING,
	STATE_ACTIVATED,
	STATE_DEACTIVATING,
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
		case STATE_DEACTIVATING:
			return "DEACTIVATING";
	}
}

static INT32 delay(net_ctx_t *ctx) {
	return (csw_TMGetTick() - ctx->ticks) / (MMI_TICK1S/1000);
}

static void net_set_state(net_ctx_t *ctx, state_t state) {
	ctx->state = state;
	draw(net_state2str(state));
}

static void net_gprs_activate_rsp(void *p) {
	net_ctx_t *ctx = &g_net_ctx;
	mmi_ps_gprs_connect_status_ind_struct *msg = (mmi_ps_gprs_connect_status_ind_struct*)p;

	LOG("status=0x%02x,delay=%dms", msg->status, delay(ctx));
	if (msg->status == 0x01) {
		net_set_state(ctx, STATE_ACTIVATED);
	}
}

static void net_gprs_deactivate_rsp(void *p) {
	net_ctx_t *ctx = &g_net_ctx;
	mmi_ps_gprs_deconnect_status_ind_struct *msg = (mmi_ps_gprs_deconnect_status_ind_struct*)p;

	LOG("status=0x%02x,delay=%dms", msg->status, delay(ctx));
	net_set_state(ctx, STATE_ATTACHED);
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

	LOG("status=0x%02x,delay=%dms", msg->status, delay(ctx));
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
	net_set_state(ctx, STATE_DEACTIVATING);
	ctx->ticks = csw_TMGetTick();
	adp_StopGPRSExt(ctx->sim, ctx->cid);
}

static void key_4() {
}

#include "tcpip_tcp.h"

static void key_5() {
	kal_int8 soc;
	INT32 keepidle = 30;
	INT32 keepinterval = 1;
	INT32 keepcount = 3;
	INT32  no_delay;
	INT32  len = sizeof(INT32);
	UINT32 ret;

	soc = soc_create(/*PF_INET, SOCK_STREAM,*/2, 1, 0, MOD_MMI, 0);
	LOG("soc=%d", soc);

	ret = CFW_TcpipSocketSetsockopt(
		soc,
		CFW_TCPIP_IPPROTO_TCP,
		TCP_ALIVE_KEEPIDLE,
		(void *)&keepidle,
		sizeof(keepidle));
	LOG("ret=%d", ret);

	ret = CFW_TcpipSocketSetsockopt(
		soc,
		CFW_TCPIP_IPPROTO_TCP,
		TCP_ALIVE_KEEPINTVL,
		(void *)&keepinterval,
		sizeof(keepinterval));
	LOG("ret=%d", ret);

	ret = CFW_TcpipSocketSetsockopt(
		soc,
		CFW_TCPIP_IPPROTO_TCP,
		TCP_ALIVE_KEEPCNT,
		(void *)&keepcount,
		sizeof(keepcount));
	LOG("ret=%d", ret);

	ret = CFW_TcpipSocketGetsockopt(
		soc,
		CFW_TCPIP_IPPROTO_TCP,
		TCP_NODELAY,
		&no_delay,
		&len);
	LOG("TCP_NODELAY:ret=%d,value=%d", ret, no_delay);

	keepidle = -1;
	ret = CFW_TcpipSocketGetsockopt(
		soc,
		CFW_TCPIP_IPPROTO_TCP,
		TCP_ALIVE_KEEPIDLE,
		&keepidle,
		&len);
	LOG("TCP_ALIVE_KEEPIDLE:ret=%d,value=%d", ret, keepidle);

	keepinterval = -1;
	ret = CFW_TcpipSocketGetsockopt(
		soc,
		CFW_TCPIP_IPPROTO_TCP,
		TCP_ALIVE_KEEPINTVL,
		&keepinterval,
		&len);
	LOG("TCP_ALIVE_KEEPINTVL:ret=%d,value=%d", ret, keepinterval);

	keepcount = -1;
	ret = CFW_TcpipSocketGetsockopt(
		soc,
		CFW_TCPIP_IPPROTO_TCP,
		TCP_ALIVE_KEEPCNT,
		&keepcount,
		&len);
	LOG("TCP_ALIVE_KEEPCNT:ret=%d,value=%d", ret, keepcount);
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
