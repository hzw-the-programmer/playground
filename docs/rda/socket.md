typedef struct {
	UINT8 cid;
} nw_ctx_t;

static nw_ctx_t g_ctx;

char* event_str(UINT32 id)
{
	switch (id)
	{
		case EV_CFW_TCPIP_SOCKET_CONNECT_RSP:
			return "EV_CFW_TCPIP_SOCKET_CONNECT_RSP";
		case EV_CFW_TCPIP_SOCKET_CLOSE_RSP:
			return "EV_CFW_TCPIP_SOCKET_CLOSE_RSP";
		case EV_CFW_TCPIP_SOCKET_SEND_RSP:
			return "EV_CFW_TCPIP_SOCKET_SEND_RSP";
		case EV_CFW_TCPIP_REV_DATA_IND:
			return "EV_CFW_TCPIP_REV_DATA_IND";
		case EV_CFW_TCPIP_ACCEPT_IND:
			return "EV_CFW_TCPIP_ACCEPT_IND";
		case EV_CFW_DNS_RESOLV_SUC_IND:
			return "EV_CFW_DNS_RESOLV_SUC_IND";
		case EV_CFW_DNS_RESOLV_ERR_IND:
			return "EV_CFW_DNS_RESOLV_ERR_IND";
		case EV_CFW_TCPIP_ERR_IND:
			return "EV_CFW_TCPIP_ERR_IND";
		case EV_CFW_TCPIP_CLOSE_IND:
			return "EV_CFW_TCPIP_CLOSE_IND";
		default:
			return "UNKNOWN";
	}
}

char *gprs_str(UINT32 id)
{
	switch(id)
	{
		case EV_CFW_GPRS_CTX_MODIFY_ACC_RSP:
			return "EV_CFW_GPRS_CTX_MODIFY_ACC_RSP";
		case EV_CFW_GPRS_ATT_RSP:
			return "EV_CFW_GPRS_ATT_RSP";
		case EV_CFW_GPRS_ACT_RSP:
			return "EV_CFW_GPRS_ACT_RSP";
		case EV_CFW_GPRS_CXT_ACTIVE_RSP:
			return "EV_CFW_GPRS_CXT_ACTIVE_RSP";
		case EV_CFW_GPRS_MOD_RSP:
			return "EV_CFW_GPRS_MOD_RSP";
		case EV_CFW_WIFI_CONNECTED_RSP:
			return "EV_CFW_WIFI_CONNECTED_RSP";

		case EV_CFW_GPRS_CXT_ACTIVE_IND:
			return "EV_CFW_GPRS_CXT_ACTIVE_IND";
		case EV_CFW_GPRS_CXT_DEACTIVE_IND:
			return "EV_CFW_GPRS_CXT_DEACTIVE_IND";
		case EV_CFW_GPRS_MOD_IND:
			return "EV_CFW_GPRS_MOD_IND";
		case EV_CFW_GPRS_STATUS_IND:
			return "EV_CFW_GPRS_STATUS_IND";
		case EV_CFW_GPRS_DATA_IND:
			return "EV_CFW_GPRS_DATA_IND";
		//case EV_CFW_GPRS_CTRL_RELEASE_IND:
		//	return "EV_CFW_GPRS_CTRL_RELEASE_IND";
	}
}

char* status_str(CFW_SIM_STATUS status)
{
	switch (status)
	{
		case CFW_SIM_ABSENT:
			return "CFW_SIM_ABSENT";
		case CFW_SIM_NORMAL:
			return "CFW_SIM_NORMAL";
		case CFW_SIM_TEST:
			return "CFW_SIM_TEST";
		case CFW_SIM_ABNORMAL:
			return "CFW_SIM_ABNORMAL";
		case CFW_SIM_STATUS_END:
			return "CFW_SIM_STATUS_END";
		default:
			return "sim status unknown";
	}
}

void http_get(SOCKET sock)
{
	UINT8 *buf = "GET / HTTP/1.1\r\n"
			  "\r\n";
	UINT16 avail, len;
	INT32 ret;
	UINT32 err;

	avail = CFW_TcpipAvailableBuffer(sock);
	len = strlen(buf);
	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:http_get,avail=%d,len=%d", avail, len);
	ret = CFW_TcpipSocketSend(sock, buf, len, 0);
	err = CFW_TcpipGetLastError();
	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:http_get,ret=%d,err=%d", ret, err);
}

void http_res(SOCKET sock)
{
	#define BUF_LEN 128
	UINT8 buf[BUF_LEN];
	INT32 ret;
	UINT32 err;
	UINT16 avail;

	avail = CFW_TcpipGetRecvAvailable(sock);
	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:http_res,avail=%d", avail);
	while (1)
	{
	    ret =  CFW_TcpipSocketRecv(sock, buf, BUF_LEN-1, 0);
	    err = CFW_TcpipGetLastError();
	    mmi_trace(MMI_TRACE_LEVEL_1, "hzw:http_res,ret=%d,err=%d", ret, err);
	    if (ret <= 0)
	    {
			break;
	    }
	    buf[ret] = 0;
	    mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s", buf);
	}
}

// MmiNetAttachGPRS AppNetWork.c
// MmiNetGPRSConnect AppNetWork.c
// app_SocketEventcb adp_GPRS_Event.c
BOOL sock_cb(COS_EVENT *ev)
{
	UINT32 id = ev->nEventId;
	in_addr temp;

	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:sock_cb,ev=%s", event_str(id));
	switch (id)
	{
		case EV_CFW_DNS_RESOLV_SUC_IND:
			if(NULL != ev->nParam1)
			{
				temp.s_addr = ev->nParam1;
				mmi_trace(MMI_TRACE_LEVEL_1, "hzw: ip=%s", inet_ntoa(temp));
				sock_connect(ev->nParam1);
			}
			break;
		case EV_CFW_TCPIP_SOCKET_CONNECT_RSP:
			http_get(ev->nParam1);
			break;
		case EV_CFW_TCPIP_REV_DATA_IND:
			http_res(ev->nParam1);
			break;
	}
}

void sock_connect(UINT32 addr)
{
	SOCKET sock;
	CFW_TCPIP_SOCKET_ADDR sock_addr = {0};
	UINT16 port = 80;
	UINT32 ret, err;

	sock = CFW_TcpipSocket(
		CFW_TCPIP_AF_INET,
		CFW_TCPIP_SOCK_STREAM,
		CFW_TCPIP_IPPROTO_IP);
	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:CFW_TcpipSocket,sock=%d", sock);

	CFW_SetTCPIPCallBackEx(sock_cb, sock);

	sock_addr.sin_family = CFW_TCPIP_AF_INET;
	sock_addr.sin_addr.s_addr = addr;
	//sock_addr.sin_addr.s_addr = CFW_TcpipInetAddr("36.152.44.96");
	sock_addr.sin_port = htons(port);

	ret = CFW_TcpipSocketConnect(sock, &sock_addr, sizeof(sock_addr));
	err = CFW_TcpipGetLastError();
	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:CFW_TcpipSocketConnect,ret=%d,err=%d", ret, err);
}

void dns(nw_ctx_t *ctx)
{
	struct ip_addr addr = {0};
	UINT32 ret;

	CFW_SetTCPIPCallBackEx(sock_cb, MEMP_NUM_NETCONN);
	ret = CFW_Gethostbyname("www.baidu.com", &addr, ctx->cid, CFW_SIM_0);
	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:CFW_Gethostbyname,ret=%d", ret);
}

bool gprs_cb(COS_EVENT *ev)
{
	UINT32 id;
	UINT8 cid, type;
	UINT16 uti;
	UINT32 ret;
	UINT8 state;
	bool processed = FALSE;

	id = ev->nEventId;
	cid = (UINT8)ev->nParam1;
	uti = HIUINT16(ev->nParam3);
	type = HIUINT8(ev->nParam3);

	mmi_trace(MMI_TRACE_LEVEL_1,
		"hzw:gprs_cb,ev=%s,cid=%d,uti=%d,type=%d",
		gprs_str(id), cid, uti, type);

	switch (id)
	{
		case EV_CFW_GPRS_ATT_RSP:
			switch (type)
			{
				case 0x01:
					activate_gprs(&g_ctx);
					break;
			}
			break;
		case EV_CFW_GPRS_ACT_RSP:
			switch (type)
			{
				case 0x01:
					dns(&g_ctx);
					break;
			}
			break;
	}

	return processed;
}

void activate_gprs(nw_ctx_t *ctx)
{
	UINT32 ret;
	UINT8 state;
	CFW_GPRS_QOS qos = {0};
	CFW_GPRS_PDPCONT_INFO pdp = {0};
	char *apnAddr = "CMWAP";
	UINT8 cid, uti;

	ret = CFW_GetFreeCID(&cid, CFW_SIM_0);
	ctx->cid = cid;

	qos.nDelay = 4;
	qos.nMean = 16;
	qos.nPeak = 4;
	qos.nPrecedence = 3;
	qos.nReliability = 3;
	ret = CFW_GprsSetReqQos(cid, &qos, CFW_SIM_0);

	pdp.nApnSize = strlen(apnAddr);
	pdp.pApn = apnAddr;
	pdp.nPdpAddrSize = 0;
	pdp.pPdpAddr = NULL ;
	pdp.nDComp = 0;
	pdp.nHComp = 0;
	pdp.nPdpType = CFW_GPRS_PDP_TYPE_IP;
	pdp.nApnUserSize = 0;
	pdp.pApnUser = NULL;
	pdp.nApnPwdSize = 0;
	pdp.pApnPwd = NULL;
	ret = CFW_GprsSetPdpCxt(cid, &pdp, CFW_SIM_0);

	CFW_SetDataConnFunEx(gprs_cb, CFW_SIM_0, cid);
	CFW_GetFreeUTI(0, &uti);
	ret = CFW_GprsAct(CFW_GPRS_ACTIVED, cid, uti, CFW_SIM_0);
	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:CFW_GprsAct,ret=%d", ret);
}

void attach_gprs()
{
	UINT32 ret;
	UINT8 uti;

	CFW_SetDataConnFunEx(gprs_cb, CFW_SIM_0, 0);
	CFW_GetFreeUTI(0, &uti);
	ret = CFW_GprsAtt(CFW_GPRS_ATTACHED, uti, CFW_SIM_0);
	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:CFW_GprsAtt,ret=%d", ret);
}

void sock_test()
{
	attach_gprs();
}
