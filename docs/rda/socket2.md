#define INVALID_CID 255

typedef struct net_ctx_s {
	CFW_SIM_ID sim;
	UINT8 cid;
	UINT8 *apn;
	void (*cb)(struct net_ctx_s *ctx);
	UINT8 *host;
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

static void net_gprs_activate_rsp(void *p)
{
	net_ctx_t *ctx = &g_net_ctx;
	mmi_ps_gprs_connect_status_ind_struct *msg = (mmi_ps_gprs_connect_status_ind_struct*)p;

	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,status=0x%x,ip=%s",
		__func__, msg->status, localip(ctx));
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
	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,CFW_GetGprsActState=%d,ip=%s",
		__func__, state, localip(ctx));
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
	net_ctx_t *ctx = &g_net_ctx;
	mmi_ps_gprs_deconnect_status_ind_struct *msg = (mmi_ps_gprs_connect_status_ind_struct*)p;

	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,status=0x%x,ip=%s",
		__func__, msg->status, localip(ctx));
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
	app_soc_get_host_by_name_ind_struct *msg = (app_soc_get_host_by_name_ind_struct*)p;

	//mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,result=%d,len=%d,addr=%s", __func__, msg->result, msg->addr_len, ipstr(msg->entry[0].address));
	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,id=%d,result=%d,len=%d,addr=%d.%d.%d.%d",
		__func__, msg->access_id, msg->result, msg->addr_len, msg->entry[0].address[0], msg->entry[0].address[1], msg->entry[0].address[2], msg->entry[0].address[3]);
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
	dns(ctx, "www.baidu.com", 2);
}

static void key3() {
	net_ctx_t *ctx = &g_net_ctx;
	dns(ctx, "www.163.com", 3);

}

static void key5() {
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
}

[16:36:48.497]   MMI 01 : hzw:dns,host=www.baidu.com,id=2,ret=-1,len=0,addr=0.0.0.0
[16:36:51.630]   MMI 01 : hzw:dns,host=www.163.com,id=3,ret=-1,len=0,addr=0.0.0.0
[16:36:53.976]   MMI 01 : hzw:net_gprs_connect,sim=0,cid=1
[16:36:54.069]   MMI 01 : hzw:net_gprs_attach,adp_AttachGPRSExt=1
[16:36:56.041]   MMI 01 : hzw:net_gprs_attach_rsp,status=0x1,ip=0.0.0.0
[16:36:56.041]   MMI 01 : hzw:net_gprs_activate,CFW_GetGprsActState=0,ip=0.0.0.0
[16:36:56.042]   MMI 01 : hzw:net_gprs_activate,soc_StartActiveGPRSExt=0
[16:36:56.580]   MMI 01 : hzw:net_gprs_activate_rsp,status=0x1,ip=10.1.166.64
[16:37:28.183]   MMI 01 : hzw:dns,host=www.baidu.com,id=2,ret=-2,len=0,addr=0.0.0.0
[16:37:29.438]   MMI 01 : hzw:dns_cb,id=2,result=1,len=4,addr=36.152.44.95
[16:37:35.336]   MMI 01 : hzw:dns,host=www.163.com,id=3,ret=-2,len=0,addr=0.0.0.0
[16:37:36.092]   MMI 01 : hzw:dns_cb,id=3,result=1,len=4,addr=112.15.45.28
[16:37:39.945]   MMI 01 : hzw:dns,host=www.baidu.com,id=2,ret=0,len=4,addr=36.152.44.95
[16:37:41.683]   MMI 01 : hzw:dns,host=www.163.com,id=3,ret=0,len=4,addr=112.15.45.28
[16:37:49.198]   MMI 01 : hzw:dns,host=www.163.com,id=3,ret=0,len=4,addr=112.15.45.28
[16:37:50.239]   MMI 01 : hzw:dns,host=www.baidu.com,id=2,ret=0,len=4,addr=36.152.44.95
[16:37:52.918]   MMI 01 : hzw:dns,host=www.163.com,id=3,ret=0,len=4,addr=112.15.45.28
[16:37:54.571]   MMI 01 : hzw:dns,host=www.baidu.com,id=2,ret=0,len=4,addr=36.152.44.95
[16:40:04.056]   MMI 01 : hzw:keydown,adp_StopGPRSExt=0
[16:40:04.651]   MMI 01 : hzw:net_gprs_deactivate_rsp,status=0x0,ip=0.0.0.0
[16:40:16.232]   MMI 01 : hzw:dns,host=www.baidu.com,id=2,ret=-1,len=0,addr=0.0.0.0
[16:40:19.652]   MMI 01 : hzw:dns,host=www.163.com,id=3,ret=-1,len=0,addr=0.0.0.0
[16:40:36.412]   MMI 01 : hzw:net_gprs_connect,sim=0,cid=1
[16:40:36.412]   MMI 01 : hzw:net_gprs_attach,adp_AttachGPRSExt=0
[16:40:36.412]   MMI 01 : hzw:net_gprs_activate,CFW_GetGprsActState=0,ip=0.0.0.0
[16:40:36.510]   MMI 01 : hzw:net_gprs_activate,soc_StartActiveGPRSExt=0
[16:40:37.085]   MMI 01 : hzw:net_gprs_activate_rsp,status=0x1,ip=10.1.47.209
[16:41:13.679]   MMI 01 : hzw:dns,host=www.baidu.com,id=2,ret=0,len=4,addr=36.152.44.95
[16:41:19.039]   MMI 01 : hzw:dns,host=www.163.com,id=3,ret=0,len=4,addr=112.15.45.28