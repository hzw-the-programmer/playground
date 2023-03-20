#define INVALID_CID 255

typedef struct net_ctx_s {
	CFW_SIM_ID sim;
	UINT8 cid;
	UINT8 *apn;
	UINT32 ticks;
} net_ctx_t;

static net_ctx_t g_net_ctx;

static void debug_task(const char *func) {
	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,taskid=%d", func, sxr_GetCurrentTaskId());
}

static UINT8* ipstr(UINT8 *addr) {
	in_addr tmp = {0};

	tmp.s_addr = *(UINT32*)addr;
	return inet_ntoa(tmp);
}

static UINT8* localip(net_ctx_t *ctx) {
	UINT8 len = 0;
	UINT8 addr[MAX_SOCK_ADDR_LEN] = {0};

	CFW_GprsGetPdpAddr(ctx->cid, &len, addr, ctx->sim);
	return ipstr(addr);
}

static INT32 delay(net_ctx_t *ctx) {
	return (csw_TMGetTick() - ctx->ticks) / (MMI_TICK1S/1000);
}

static void net_gprs_activate_rsp(void *p)
{
	net_ctx_t *ctx = &g_net_ctx;
	mmi_ps_gprs_connect_status_ind_struct *msg = (mmi_ps_gprs_connect_status_ind_struct*)p;

	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,status=0x%x,ip=%s,ticks=%d",
		__func__, msg->status, localip(ctx), delay(ctx));
}

static void net_gprs_deactivate_rsp(void *p)
{
	net_ctx_t *ctx = &g_net_ctx;
	mmi_ps_gprs_deconnect_status_ind_struct *msg = (mmi_ps_gprs_deconnect_status_ind_struct*)p;

	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,status=0x%x,ip=%s",
		__func__, msg->status, localip(ctx));
}

static void net_gprs_activate(net_ctx_t *ctx) {
	INT32 ret;
	UINT8 state;

	CFW_GetGprsActState(ctx->cid, &state, ctx->sim);
	if (state == 1) {
		mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,state=%d,ip=%s",
			__func__, state, localip(ctx));
		return;
	}

	ret = soc_StartActiveGPRSExt(ctx->sim, ctx->cid, 
		ctx->apn, "", "");
	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,state=%d,ip=%s,soc_StartActiveGPRSExt=0x%x",
		__func__, state, localip(ctx), ret);

	if (ret == 0) {
		ctx->ticks = csw_TMGetTick();
		SetProtocolEventHandler(net_gprs_activate_rsp, MSG_ID_MMI_NW_GPRS_CONNECTED_RSP);
		SetProtocolEventHandler(net_gprs_deactivate_rsp, MSG_ID_MMI_NW_GPRS_DECONNECTED_RSP);
	}
}

static void net_gprs_attach_rsp(void *p)
{
	net_ctx_t *ctx = &g_net_ctx;
	mmi_ps_gprs_deconnect_status_ind_struct *msg = (mmi_ps_gprs_connect_status_ind_struct*)p;

	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,status=0x%x,ip=%s,ticks=%d",
		__func__, msg->status, localip(ctx), delay(ctx));
	if (msg->status == 0x01) {
		net_gprs_activate(ctx);
	}
}

static void net_gprs_dettach_rsp(void *p)
{
	net_ctx_t *ctx = &g_net_ctx;
	mmi_ps_gprs_deconnect_status_ind_struct *msg = (mmi_ps_gprs_connect_status_ind_struct*)p;

	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,status=0x%x,ip=%s",
		__func__, msg->status, localip(ctx));
}

static void net_gprs_attach(net_ctx_t *ctx) {
	UINT32 ret;

	ret = adp_AttachGPRSExt(ctx->sim);
	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,adp_AttachGPRSExt=%d", __func__, ret);
	if (ret == ADP_GPRS_ATTACH_ALREADY) {
		net_gprs_activate(ctx);
	} else if (ret == ADP_GPRS_ATTACH_ASYN) {
		ctx->ticks = csw_TMGetTick();
		SetProtocolEventHandler(net_gprs_attach_rsp, MSG_ID_MMI_NW_GPRS_ATTACH_RSP);
		SetProtocolEventHandler(net_gprs_dettach_rsp, MSG_ID_MMI_NW_GPRS_DETTACH_RSP);
	}
}

static void net_gprs_connect(net_ctx_t *ctx) {
	if (!MTPNP_PFAL_Is_Card_Usable(ctx->sim)) {
		ctx->sim = MTPNP_AD_GET_UsableSide_Index();
	}

	if (ctx->cid == INVALID_CID) {
		CFW_GetFreeCID(&ctx->cid, ctx->sim);
	}

	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,sim=%d,cid=%d",
		__func__, ctx->sim, ctx->cid);

	net_gprs_attach(ctx);
}

static void dns_cb(void* p) {
	net_ctx_t *ctx = &g_net_ctx;
	app_soc_get_host_by_name_ind_struct *msg = (app_soc_get_host_by_name_ind_struct*)p;

	//mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,result=%d,len=%d,addr=%s", __func__, msg->result, msg->addr_len, ipstr(msg->entry[0].address));
	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,id=%d,result=%d,len=%d,addr=%d.%d.%d.%d,ticks=%d",
		__func__, msg->access_id, msg->result, msg->addr_len,
		msg->entry[0].address[0], msg->entry[0].address[1],
		msg->entry[0].address[2], msg->entry[0].address[3],
		delay(ctx));

	//ClearProtocolEventHandler(MSG_ID_APP_SOC_GET_HOST_BY_NAME_IND);
}

static void dns(net_ctx_t *ctx, UINT8 *host, UINT8 id) {
	INT8 ret;
	UINT8 addr[12] = {0};
	UINT8 len = 0;

	ret = soc_gethostbynameExt(ctx->sim, ctx->cid, KAL_FALSE, 
		MOD_MMI, WEP_DNS_ID, host, addr, &len, id, 0);

	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,host=%s,id=%d,ret=%d,len=%d,addr=%s",
		__func__, host, id, ret, len, ipstr(addr));

	if (ret == SOC_SUCCESS) {

	} else if (ret == SOC_WOULDBLOCK) {
		ctx->ticks = csw_TMGetTick();
		SetProtocolEventHandler(dns_cb, MSG_ID_APP_SOC_GET_HOST_BY_NAME_IND);
	}
}

static void net_gprs_close(net_ctx_t *ctx) {
	UINT32 ret;
	
	ret = adp_StopGPRSExt(ctx->sim, ctx->cid);
	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,ret=%d", __func__, ret);
}

static void net_test() {
	net_ctx_t *ctx = &g_net_ctx;

	if (!ctx->apn) {
		ctx->apn = "CMWAP";
		ctx->cid = INVALID_CID;
	}

	net_gprs_connect(ctx);
}

static void lsk() {
}

static void keyup() {
	net_test();
}

static void keydown() {
	net_ctx_t *ctx = &g_net_ctx;
	UINT32 ret;

	ret = adp_StopGPRSExt(ctx->sim, ctx->cid);
	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,adp_StopGPRSExt=%d", __func__, ret);
}

static void key2() {
	net_ctx_t *ctx = &g_net_ctx;

	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s----------", __func__);
	dns(ctx, "www.baidu.com", 2);
}

static void key3() {
	net_ctx_t *ctx = &g_net_ctx;

	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s----------", __func__);
	dns(ctx, "www.163.com", 3);
}

static void key5() {
	net_ctx_t *ctx = &g_net_ctx;

	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s----------", __func__);

	dns(ctx, "www.toutiao.com", 5);
	dns(ctx, "www.douyin.com", 6);
}

static void key6() {
	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s----------", __func__);

	net_test();
	net_test();
}

static void test_entry() {
	EntryNewScreen(CHAT_BASE, NULL, NULL, NULL);

	gdi_layer_clear(GDI_COLOR_WHITE);
	gdi_layer_blt_previous(0, 0, UI_device_width-1, UI_device_height-1);

	SetKeyHandler(keyup, KEY_UP_ARROW, KEY_EVENT_UP);
	SetKeyHandler(keydown, KEY_DOWN_ARROW, KEY_EVENT_UP);
	SetKeyHandler(key2, KEY_2, KEY_EVENT_UP);
	SetKeyHandler(key3, KEY_3, KEY_EVENT_UP);
	SetKeyHandler(key5, KEY_5, KEY_EVENT_UP);
	SetKeyHandler(key6, KEY_6, KEY_EVENT_UP);
}

[19:46:47.983]   MMI 01 : hzw:key2----------
[19:46:48.090]   MMI 01 : hzw:dns,host=www.baidu.com,id=2,ret=-1,len=0,addr=0.0.0.0
[19:46:58.394]   MMI 01 : hzw:net_gprs_connect,sim=0,cid=1
[19:46:58.504]   MMI 01 : hzw:net_gprs_attach,adp_AttachGPRSExt=1
[19:47:05.641]   MMI 01 : hzw:net_gprs_attach_rsp,status=0x1,ip=0.0.0.0,ticks=7348
[19:47:05.641]   MMI 01 : hzw:net_gprs_activate,state=0,ip=0.0.0.0,soc_StartActiveGPRSExt=0x0
[19:47:06.367]   MMI 01 : hzw:net_gprs_activate_rsp,status=0x1,ip=10.5.29.144,ticks=739
[19:48:54.413]   MMI 01 : hzw:key2----------
[19:48:54.516]   MMI 01 : hzw:dns,host=www.baidu.com,id=2,ret=-2,len=0,addr=0.0.0.0
[19:48:56.796]   MMI 01 : hzw:dns_cb,id=2,result=1,len=4,addr=36.152.44.95,ticks=2350
[19:49:11.412]   MMI 01 : hzw:key3----------
[19:49:11.512]   MMI 01 : hzw:dns,host=www.163.com,id=3,ret=-2,len=0,addr=0.0.0.0
[19:49:12.239]   MMI 01 : hzw:dns_cb,id=3,result=1,len=4,addr=112.15.45.25,ticks=814
[19:49:42.876]   MMI 01 : hzw:key6----------
[19:49:42.876]   MMI 01 : hzw:net_gprs_connect,sim=0,cid=1
[19:49:42.876]   MMI 01 : hzw:net_gprs_attach,adp_AttachGPRSExt=0
[19:49:42.876]   MMI 01 : hzw:net_gprs_activate,state=1,ip=10.5.29.144
[19:49:42.876]   MMI 01 : hzw:net_gprs_connect,sim=0,cid=1
[19:49:42.876]   MMI 01 : hzw:net_gprs_attach,adp_AttachGPRSExt=0
[19:49:42.986]   MMI 01 : hzw:net_gprs_activate,state=1,ip=10.5.29.144
[19:49:51.331]   MMI 01 : hzw:keydown,adp_StopGPRSExt=0
[19:49:52.045]   MMI 01 : hzw:net_gprs_deactivate_rsp,status=0x0,ip=0.0.0.0
[19:50:10.239]   MMI 01 : hzw:key5----------
[19:50:10.239]   MMI 01 : hzw:dns,host=www.toutiao.com,id=5,ret=-1,len=0,addr=0.0.0.0
[19:50:10.346]   MMI 01 : hzw:dns,host=www.douyin.com,id=6,ret=-1,len=0,addr=0.0.0.0
[19:50:19.267]   MMI 01 : hzw:key6----------
[19:50:19.267]   MMI 01 : hzw:net_gprs_connect,sim=0,cid=1
[19:50:19.267]   MMI 01 : hzw:net_gprs_attach,adp_AttachGPRSExt=0
[19:50:19.268]   MMI 01 : hzw:net_gprs_activate,state=0,ip=0.0.0.0,soc_StartActiveGPRSExt=0x0
[19:50:19.268]   MMI 01 : hzw:net_gprs_connect,sim=0,cid=1
[19:50:19.268]   MMI 01 : hzw:net_gprs_attach,adp_AttachGPRSExt=0
[19:50:19.268]   MMI 01 : hzw:net_gprs_activate,state=0,ip=0.0.0.0,soc_StartActiveGPRSExt=0x0
[19:50:19.904]   MMI 01 : hzw:net_gprs_activate_rsp,status=0x1,ip=10.5.48.134,ticks=652
[19:51:07.051]   MMI 01 : hzw:key2----------
[19:51:07.128]   MMI 01 : hzw:dns,host=www.baidu.com,id=2,ret=0,len=4,addr=36.152.44.95
[19:51:11.234]   MMI 01 : hzw:key3----------
[19:51:11.328]   MMI 01 : hzw:dns,host=www.163.com,id=3,ret=0,len=4,addr=112.15.45.25
