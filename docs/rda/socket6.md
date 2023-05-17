typedef enum {
    STATE_ACTIVATED,
} state_e;

typedef struct {
    int sim;
    UINT8 cid;
    void (*cb)(UINT8 state);
} gprs_ctx_t;

static gprs_ctx_t g_gprs_ctx;

static UINT8* ipstr(UINT32 addr) {
	in_addr tmp;
	tmp.s_addr = addr;
	return inet_ntoa(tmp);
}

static UINT8* localip(const gprs_ctx_t *ctx) {
	UINT32 addr;
	UINT8 len = 0;

	CFW_GprsGetPdpAddr(ctx->cid, &len, &addr, ctx->sim);
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

#define ARR_SIZE(arr) (sizeof(arr)/sizeof(arr[0]))

typedef struct {
    UINT8 *domain;
    UINT32 ip;
} dns_entry_t;

#define BAIDU_INDEX 0
#define DOUYING_INDEX 1

static dns_entry_t g_dns_entry[] = {
    {"www.baidu.com", 0},
    {"www.douyin.com", 0},
};

static dns_entry_t* get_dns_entry(UINT8 index) {
    if (index < ARR_SIZE(g_dns_entry)) {
        return &g_dns_entry[index];
    }
    return NULL;
}

static void dns_cb(void* p) {
	app_soc_get_host_by_name_ind_struct *msg = (app_soc_get_host_by_name_ind_struct*)p;
    dns_entry_t *entry;
    int i;

    LOG("%s:%d:%d", "dnsCb", msg->result, msg->access_id);
    if (!msg->result) {
        return;
    }
    entry = get_dns_entry(msg->access_id);
    if (!entry) {
        return;
    }

    LOG("%s", entry->domain);
    for (i = 0; i < msg->num_entry; i++) {
        UINT32 ip;
        memcpy(&ip, msg->entry[i].address, sizeof(UINT32));
        LOG("%s", ipstr(ip));
        entry->ip = ip;
    }
}

static void dns(gprs_ctx_t *ctx, UINT8 index) {
    dns_entry_t *entry;
    INT8 ret;
    UINT32 ip;
    UINT8 len = 0;

    LOG("dns:%d", index);

    entry = get_dns_entry(index);
    if (!entry) {
        return;
    }

    LOG("%s", entry->domain);

	ret = soc_gethostbynameExt(ctx->sim, ctx->cid, KAL_FALSE,
		MOD_MMI, WEP_DNS_ID, entry->domain, &ip, &len, index, 0);

	if (ret == SOC_SUCCESS) {
        LOG("%s", ipstr(ip));
        entry->ip = ip;
	} else if (ret == SOC_WOULDBLOCK) {
		SetProtocolEventHandler(dns_cb, MSG_ID_APP_SOC_GET_HOST_BY_NAME_IND);
	}
}

static void notify(void *p) {
	app_soc_notify_ind_struct *msg = (app_soc_notify_ind_struct*)p;

    switch(msg->event_type) {
    case SOC_CONNECT:
        LOG("SOC_CONNECT:%d", msg->result);
        {
            kal_int32 n;
            kal_uint8 *buf = "GET / HTTP/1.1";
            n = soc_send(msg->socket_id, buf, strlen(buf), 0);
            LOG("send:%d,%d", n, strlen(buf));
        }
        break;

    case SOC_WRITE:
        LOG("SOC_WRITE:%d", msg->result);
        break;

    case SOC_READ:
        LOG("SOC_READ:%d", msg->result);
        {
            kal_int32 n = 0;
            while (n != SOC_WOULDBLOCK) {                
                kal_uint8 buf[8] = {0};
                n = soc_recv(msg->socket_id, buf, 8-1, 0);
                LOG("recv:%d", n);
                LOG("recv:%s", buf);
            }
        }
        break;

    case SOC_CLOSE:
        LOG("SOC_CLOSE:%d", msg->result);
        break;

    case SOC_ERROR_IND:
        LOG("SOC_ERROR_IND:%d", msg->result);
        break;

    case SOC_ACCEPT:
        LOG("SOC_ACCEPT:%d", msg->result);
        break;

    default:
        LOG("SOC_0x%02x:%d", msg->event_type, msg->result);
        break;
    }
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
    int i;

    for (i = 0; i < ARR_SIZE(g_dns_entry); i++) {
        dns(&g_gprs_ctx, i);
    }
}

static void key_4() {
}

static void key_5() {
    kal_int8 soc;
    kal_int8 ret;
    sockaddr_struct addr = {0};
    dns_entry_t* entry;
    kal_uint8 *ip;

    entry = get_dns_entry(BAIDU_INDEX);
    if (!entry) {
        return;
    }
    ip = &entry->ip;

    soc = soc_create(2, 1, 0, MOD_MMI, 0);
    LOG("soc=%d", soc);
    if (soc < 0) return;

    addr.addr[0] = ip[0];
    addr.addr[1] = ip[1];
    addr.addr[2] = ip[2];
    addr.addr[3] = ip[3];
    addr.port = 80;
    addr.addr_len = 4;

    ret = soc_connect(soc, &addr);
    if (ret == SOC_WOULDBLOCK) {
        LOG("conRet:block");
        SetProtocolEventHandler(notify, MSG_ID_APP_SOC_NOTIFY_IND);
    } else {
        LOG("conRet:%d", ret);
    }
}

static void key_6() {
}

static void key_7() {
}

static void key_8() {
}

static void key_9() {
    gprs_ctx_t *ctx = &g_gprs_ctx;
    UINT32 ret;
    UINT8 state;

    ret = CFW_GetGprsAttState(&state, ctx->sim);
    LOG("att:s=%d,r=%d", state, ret);

    ret = CFW_GetGprsActState(ctx->cid, &state, ctx->sim);
    LOG("act:s=%d,r=%x", state, ret);
}

static void key_0() {
    gprs_ctx_t *ctx = &g_gprs_ctx;

    ctx->sim++;
    ctx->sim %= 2;
    
    LOG("selected:%d", ctx->sim);
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
	SetKeyHandler(key_8, KEY_8, KEY_EVENT_DOWN);
	SetKeyHandler(key_9, KEY_9, KEY_EVENT_DOWN);
	SetKeyHandler(key_0, KEY_0, KEY_EVENT_DOWN);
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