typedef enum {
    STATE_ACTIVATED,
} state_e;

typedef struct {
    int sim;
    UINT8 cid;
    void (*cb)(UINT8 state);
} gprs_ctx_t;

static gprs_ctx_t g_gprs_ctx;

static UINT8* ipstr(UINT8 *addr) {
	in_addr tmp = {0};

	tmp.s_addr = *(UINT32*)addr;
	return inet_ntoa(tmp);
}

static UINT8* localip(const gprs_ctx_t *ctx) {
	UINT8 len = 0;
	UINT32 addr[MAX_SOCK_ADDR_LEN>>2] = {0};

	CFW_GprsGetPdpAddr(ctx->cid, &len, addr, ctx->sim);
	return ipstr(addr);
}

static void gprs_activate_rsp(void *p)
{
    gprs_ctx_t *ctx = &g_gprs_ctx;
    mmi_ps_gprs_connect_status_ind_struct *msg =
        (mmi_ps_gprs_connect_status_ind_struct*)p;

    if (msg->status == 0x01) {
        LOG("ACT_SUCCESS");
        ctx->cb(STATE_ACTIVATED);
    } else if (msg->status == 0xF1) {
        LOG("ACT_FAIL");
    } else {
        LOG("ACT_ERR:0x%02x", msg->status);
    }
}

static void gprs_deactivate_rsp(void *p)
{
    mmi_ps_gprs_deconnect_status_ind_struct *msg =
        (mmi_ps_gprs_deconnect_status_ind_struct*)p;

    if (msg->status == 0x00) {
        LOG("DCT_SUCCESS");
    } else if (msg->status == 0xF0) {
        LOG("DCT_FAIL");
    } else {
        LOG("DCT_ERR:0x%02x", msg->status);
    }
}

static void gprs_activate(gprs_ctx_t *ctx)
{
    INT32 ret;
    UINT8 state;

    if (!ctx->cid) {
        CFW_GetFreeCID(&ctx->cid, ctx->sim);
    }
    LOG("cid:%d", ctx->cid);

    CFW_GetGprsActState(ctx->cid, &state, ctx->sim);
    if (state == CFW_GPRS_ACTIVED) {
        LOG("ACTIVED");
        ctx->cb(STATE_ACTIVATED);
        return;
    }

    ret = soc_StartActiveGPRSExt(ctx->sim, ctx->cid,
        "CMNET", "", "");
    if (ret == -1) {
        LOG("ACTIVATING ERR");
    } else {
        LOG("ACTIVATING");
        SetProtocolEventHandler(
            gprs_activate_rsp,
            MSG_ID_MMI_NW_GPRS_CONNECTED_RSP);
        SetProtocolEventHandler(
            gprs_deactivate_rsp,
            MSG_ID_MMI_NW_GPRS_DECONNECTED_RSP);
    }
}

static void gprs_attach_rsp(void *p)
{
    gprs_ctx_t *ctx = &g_gprs_ctx;
    mmi_ps_gprs_deconnect_status_ind_struct *msg =
        (mmi_ps_gprs_deconnect_status_ind_struct*)p;

    if (msg->status == 0x01) {
        LOG("ATT_SUCCESS");
        gprs_activate(ctx);
    } else if (msg->status == 0xF1) {
        LOG("ATT_FAIL");
    } else {
        LOG("ATT_ERR:0x%02x", msg->status);
    }
}

static void gprs_dettach_rsp(void *p)
{
    mmi_ps_gprs_deconnect_status_ind_struct *msg =
        (mmi_ps_gprs_deconnect_status_ind_struct*)p;

    if (msg->status == 0x00) {
        LOG("DTT_SUCCESS");
    } else if (msg->status == 0xF0) {
        LOG("DTT_FAIL");
    } else {
        LOG("DTT_ERR:0x%02x", msg->status);
    }
}

static void gprs_attach(gprs_ctx_t *ctx) {
    UINT32 ret;

    ret = adp_AttachGPRSExt(ctx->sim);
	if (ret == ADP_GPRS_ATTACH_ALREADY) {
        LOG("ATT_ALREADY");
        gprs_activate(ctx);
	} else if (ret == ADP_GPRS_ATTACH_ASYN) {
        LOG("ATT_ASYN");
        SetProtocolEventHandler(
            gprs_attach_rsp,
            MSG_ID_MMI_NW_GPRS_ATTACH_RSP);
        //SetSlaveProtocolEventHandler(
        //    gprs_attach_rsp,
        //    MSG_ID_MMI_NW_GPRS_ATTACH_RSP);
        SetProtocolEventHandler(
            gprs_dettach_rsp,
            MSG_ID_MMI_NW_GPRS_DETTACH_RSP);
        //SetSlaveProtocolEventHandler(
        //    gprs_dettach_rsp,
        //    MSG_ID_MMI_NW_GPRS_DETTACH_RSP);
	} else {
        LOG("ATT_ERR: 0x%x", ret);
	}
}

void log_ex(UINT8 *msg) {
    LOG("%s", msg);
}

// PhnsetFlightModeSelectionOK
// SaveDmSet
// MTPNP_PFAL_Switch_Mode
// mmi_dm_set_rsp_hdlr
void flight_mode_cb() {
    gprs_ctx_t *ctx = &g_gprs_ctx;

    LOG("fmcb:fm=%d", MTPNP_AD_Is_Flight_Mode());
    LOG("%s", localip(ctx));
    if (!MTPNP_AD_Is_Flight_Mode()) {
        gprs_attach(ctx);
    }
}

static void gprs_cb(UINT8 state) {
    gprs_ctx_t *ctx = &g_gprs_ctx;

    LOG("%s", localip(ctx));
}

static void lsk() {
}

static void key_1() {
}

static void key_2() {
    gprs_ctx_t *ctx = &g_gprs_ctx;

    LOG("current:%d", ctx->sim);
    ctx->cb = gprs_cb;
    gprs_attach(ctx);
}

static void key_3() {
}

static void key_4() {
}

static void key_5() {
    g_gprs_ctx.sim = 0;
    LOG("selected:%d", g_gprs_ctx.sim);
}

static void key_6() {
    g_gprs_ctx.sim = 1;
    LOG("selected:%d", g_gprs_ctx.sim);
}

static void key_7() {
    gprs_ctx_t *ctx = &g_gprs_ctx;
    UINT32 ret;
    UINT8 state;

    ret = CFW_GetGprsAttState(&state, ctx->sim);
    LOG("att:state=%d,ret=%d", state, ret);

    ret = CFW_GetGprsActState(ctx->cid, &state, ctx->sim);
    LOG("act:state=%d,ret=%x", state, ret);
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
	EntryNewScreen(666, exit_app, enter_app, NULL);
    // no history
    // ExecuteCurrExitHandler_Ext -> GenericExitScreen
    //EntryNewScreen(666, exit_app, NULL, NULL);

	register_handlers();
	LOG("enter");
}