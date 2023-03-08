#define INVALID_CID 255

typedef struct {
	CFW_SIM_ID sim;
	UINT8 cid;
	UINT8 *apn;
} net_ctx_t;

static net_ctx_t g_net_ctx;

static void debug_task(const char *func) {
	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,taskid=%d", func, sxr_GetCurrentTaskId());
}

static void net_gprs_activate_rsp(void *p)
{
	mmi_ps_gprs_connect_status_ind_struct *msg = (mmi_ps_gprs_connect_status_ind_struct*)p;

	debug_task(__func__);

	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,status=0x%x", __func__, msg->status);
}

static void net_gprs_deactivate_rsp(void *p)
{
	mmi_ps_gprs_deconnect_status_ind_struct *msg = (mmi_ps_gprs_deconnect_status_ind_struct*)p;

	debug_task(__func__);

	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,status=0x%x", __func__, msg->status);
}

static void net_gprs_activate(net_ctx_t *ctx) {
	INT32 ret;
	UINT8 state;

	debug_task(__func__);

	CFW_GetGprsActState(ctx->cid, &state, ctx->sim);
	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,state=%d", __func__, state);
	if (state == 1) {
		return;
	}

	ret = soc_StartActiveGPRSExt(ctx->sim, ctx->cid, 
		ctx->apn, "", "");
	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,soc_StartActiveGPRSExt=%d", __func__, ret);

	SetProtocolEventHandler(net_gprs_activate_rsp, MSG_ID_MMI_NW_GPRS_CONNECTED_RSP);
	SetProtocolEventHandler(net_gprs_deactivate_rsp, MSG_ID_MMI_NW_GPRS_DECONNECTED_RSP);
}

static void net_gprs_attach_rsp(void *p)
{
	mmi_ps_gprs_deconnect_status_ind_struct *msg = (mmi_ps_gprs_connect_status_ind_struct*)p;

	debug_task(__func__);

	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,status=0x%x", __func__, msg->status);
	if (msg->status == 0x01) {
		net_gprs_activate(&g_net_ctx);
	}
}

static void net_gprs_dettach_rsp(void *p)
{
	mmi_ps_gprs_deconnect_status_ind_struct *msg = (mmi_ps_gprs_connect_status_ind_struct*)p;

	debug_task(__func__);

	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,status=0x%x", __func__, msg->status);
}

static void net_gprs_attach(net_ctx_t *ctx) {
	UINT32 ret;

	ret = adp_AttachGPRSExt(ctx->sim);
	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,adp_AttachGPRSExt=%d", __func__, ret);
	if (ret == ADP_GPRS_ATTACH_ALREADY) {
		net_gprs_activate(ctx);
	} else if (ret == ADP_GPRS_ATTACH_ASYN) {
		SetProtocolEventHandler(net_gprs_attach_rsp, MSG_ID_MMI_NW_GPRS_ATTACH_RSP);
		SetProtocolEventHandler(net_gprs_dettach_rsp, MSG_ID_MMI_NW_GPRS_DETTACH_RSP);
	}
}

static void net_gprs_connect(net_ctx_t *ctx) {
	UINT32 ret;

	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,sim=%d", __func__, ctx->sim);
	if (!MTPNP_PFAL_Is_Card_Usable(ctx->sim)) {
		ctx->sim = MTPNP_AD_GET_UsableSide_Index();
	}
	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,sim=%d", __func__, ctx->sim);

	if (ctx->cid == INVALID_CID) {
		ret = CFW_GetFreeCID(&ctx->cid, ctx->sim);
		mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,CFW_GetFreeCID=%d,cid=%d", __func__, ret, ctx->cid);
	}

	net_gprs_attach(ctx);
}

static void net_test() {
	net_ctx_t *ctx = &g_net_ctx;

	if (!ctx->apn) {
		ctx->apn = "CMWAP";
		ctx->cid = INVALID_CID;
	}

	net_gprs_connect(ctx);
}