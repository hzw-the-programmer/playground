# file
interface/inet_ps/soc_api.h

## dns
soc_gethostbyname
soc_gethostbyname2
SOC_SUCCESS
SOC_WOULDBLOCK
MSG_ID_APP_SOC_GET_HOST_BY_NAME_IND

## socket
soc_create
soc_setsockopt
soc_connect
soc_send
soc_recv
MSG_ID_APP_SOC_NOTIFY_IND

# file
MS_Code\DAPS\export\inc\socket_api.h

## file
MS_Code\MS_MMI\source\mmi_app\app\pdp\c\mmipdp_api.c
## func
MMIAPIPDP_Active

## file
MS_Code\MS_MMI\source\mmi_app\app\apple\c\apple_export.c
## func

## file
MS_Code\MS_MMI\source\mmi_app\kernel\h\mmk_regapp.def
## func
REG_APP(SOCKET_SIG_BEGIN, SOCKET_SIG_END, &g_opera_mobile_app)

## file
MS_Code\DAPS\export\inc\IN_Message.h
## func
SOCKET_SIG_BEGIN
SOCKET_READ_EVENT_IND

## example
```
typedef struct {
	SIGNAL_VARS
	void *data;
} HZW_SIGNAL_T;

typedef enum {
	HZW_SIGNAL_REQUEST,
} HZW_SIGNAL_E;

static MMI_HANDLE_T hzw_win_handle = 0;
static BLOCK_ID hzw_tid = 0;
static TCPIP_NETID_T hzw_net_id = 0;
static TCPIP_SOCKET_T hzw_sock = 0;

char* hzw_network_active_msg(MMIPDP_APP_MSG_E msg) {
	switch (msg) {
	case MMIPDP_APP_MSG_INVALID:
		return "MMIPDP_APP_MSG_INVALID";
    case MMIPDP_ACTIVE_REQ:
		return "MMIPDP_ACTIVE_REQ";
    case MMIPDP_ACTIVE_CNF:
		return "MMIPDP_ACTIVE_CNF";
    case MMIPDP_DEACTIVE_REQ:
		return "MMIPDP_DEACTIVE_REQ";
    case MMIPDP_DEACTIVE_CNF:
		return "MMIPDP_DEACTIVE_CNF";
    case MMIPDP_DEACTIVE_IND:
		return "MMIPDP_DEACTIVE_IND";
    case MMIPDP_SERVICE_CHANGE_IND:
		return "MMIPDP_SERVICE_CHANGE_IND";
    case MMIPDP_SUSPEND_IND:
		return "MMIPDP_SUSPEND_IND";
    case MMIPDP_RESUME_IND:
		return "MMIPDP_RESUME_IND";
    case MMIPDP_ACTIVE_TIMOUT_IND:
		return "MMIPDP_ACTIVE_TIMOUT_IND";
    case MMIPDP_REACTIVE_TIMER_IND:
		return "MMIPDP_REACTIVE_TIMER_IND";
    case MMIPDP_PS_ACTIVE_CNF:
		return "MMIPDP_PS_ACTIVE_CNF";
    case MMIPDP_PS_DEACTIVE_CNF:
		return "MMIPDP_PS_DEACTIVE_CNF";
    case MMIPDP_PS_DEACTIVE_IND:
		return "MMIPDP_PS_DEACTIVE_IND";
	default:
		return "UNKNOWN";
	}
}

char* hzw_network_active_result(MMIPDP_RESULT_E result) {
	switch (result) {
	case MMIPDP_RESULT_SUCC:
		return "MMIPDP_RESULT_SUCC";
    case MMIPDP_RESULT_FAIL:
		return "MMIPDP_RESULT_FAIL";
#ifndef GPRSMPDP_SUPPORT
    case MMIPDP_RESULT_AT_IS_ON:
		return "MMIPDP_RESULT_AT_IS_ON";
#endif
    case MMIPDP_RESULT_DIFFER_DUALSYS:
		return "MMIPDP_RESULT_DIFFER_DUALSYS";
    case MMIPDP_RESULT_TIMEOUT:
		return "MMIPDP_RESULT_TIMEOUT";
    case MMIPDP_RESULT_NOT_PERMIT:
		return "MMIPDP_RESULT_NOT_PERMIT";
#ifdef DATA_ROAMING_SUPPORT
    case MMIPDP_RESULT_NOT_PERMIT_ROAMING:
		return "MMIPDP_RESULT_NOT_PERMIT_ROAMING";
#endif
    case MMIPDP_RESULT_FDN_NOT_PERMIT:
		return "MMIPDP_RESULT_FDN_NOT_PERMIT";
	default:
		return "UNKNOWN";
	}
}

BOOLEAN hzw_send_signal(HZW_SIGNAL_E code, void *data, BLOCK_ID tid) {
	HZW_SIGNAL_T *signal = NULL;

	SCI_TRACE_LOW("hzw_send_signal code=0x%08x", code);

	signal = (HZW_SIGNAL_T*)SCI_ALLOC(sizeof(HZW_SIGNAL_T));
	if (!signal) {
		return FALSE;
	}
	SCI_MEMSET(signal, 0, sizeof(HZW_SIGNAL_T));

	signal->SignalCode = code;
	signal->SignalSize = sizeof(xSignalHeaderRec);
	signal->Sender = NULL;
	signal->data = data;

	return SCI_SendSignal((xSignalHeader)signal, tid) == SCI_SUCCESS;
}

static void hzw_network_active_cb(MMIPDP_CNF_INFO_T *msg) {
	BLOCK_ID tid = SCI_IdentifyThread();

	SCI_TRACE_LOW("hzw_network_active_cb tid=0x%08x", tid);

	if (msg == NULL) {
		return;
	}

	SCI_TRACE_LOW(
		"hzw_network_active_cb msg_id=%s, result=%s, nsapi=0x%08x",
		hzw_network_active_msg(msg->msg_id),
		hzw_network_active_result(msg->result),
		msg->nsapi
	);

	switch (msg->msg_id) {
	case MMIPDP_ACTIVE_CNF:
		if (msg->result == MMIPDP_RESULT_SUCC) {
			hzw_net_id = msg->nsapi;
			hzw_send_signal(HZW_SIGNAL_REQUEST, (void*)hzw_net_id, hzw_tid);
		}
		break;
	}
}

BOOLEAN hzw_network_active() {
	MN_DUAL_SYS_E sim = MN_DUAL_SYS_1;
	uint8 index = 0;
	MMICONNECTION_LINKSETTING_DETAIL_T *item = NULL;
	MMIPDP_ACTIVE_INFO_T info = {0};
	BOOLEAN ret;
	BLOCK_ID tid = SCI_IdentifyThread();

	SCI_TRACE_LOW("hzw_network_active tid=0x%08x", tid);

	for (sim = MN_DUAL_SYS_1; sim < MN_DUAL_SYS_MAX; sim++) {
		if (MMIAPIPHONE_IsSimAvailable(sim)) {
			break;
		}
	}

	if (sim == MN_DUAL_SYS_MAX) {
		return;
	}

	index = MMIAPIBROWSER_GetNetSettingIndex(sim);
	item = MMIAPICONNECTION_GetLinkSettingItemByIndex(sim, index);

	if (item == NULL) {
		return;
	}

	info.app_handler = HZW_MODULE;
	info.auth_type = item->auth_type;
	info.apn_ptr = (char*)item->apn;
	info.user_name_ptr = (char*)item->username;
	info.psw_ptr = (char*)item->password;
	info.dual_sys = sim;
	info.priority = 3;
	info.ps_service_rat = MN_UNSPECIFIED;
	info.handle_msg_callback = hzw_network_active_cb;
	info.ps_service_type = IM_E;
	info.storage = MN_GPRS_STORAGE_ALL;

	return MMIAPIPDP_Active(&info);
}

char* hzw_get_win_msg_str(MMI_MESSAGE_ID_E msg_id) {
	static char str[4] = {0};

	switch (msg_id) {
		case MSG_OPEN_WINDOW:
			return "MSG_OPEN_WINDOW";
		case MSG_PRE_FULL_PAINT:
			return "MSG_PRE_FULL_PAINT";
		case MSG_FULL_PAINT:
			return "MSG_FULL_PAINT";
		case MSG_END_FULL_PAINT:
			return "MSG_END_FULL_PAINT";
		case MSG_KEYREPEAT_WEB:
			return "MSG_KEYREPEAT_WEB";
		case MSG_KEYUP_WEB:
			return "MSG_KEYUP_WEB";
		case MSG_CLOSE_WINDOW:
			return "MSG_CLOSE_WINDOW";	
		case MSG_KEYDOWN_1:
			return "MSG_KEYDOWN_1";
		case MSG_KEYUP_1:
			return "MSG_KEYUP_1";
		case MSG_KEYDOWN_WEB:
			return "MSG_KEYDOWN_WEB";
		case MSG_KEYDOWN_OK:
			return "MSG_KEYDOWN_OK";
		case MSG_KEYUP_OK:
			return "MSG_KEYUP_OK";
		case MSG_LOSE_FOCUS:
			return "MSG_LOSE_FOCUS";
		case MSG_GET_FOCUS:
			return "MSG_GET_FOCUS";
		case MSG_KEYDOWN_CANCEL:
			return "MSG_KEYDOWN_CANCEL";
		case MSG_KEYUP_CANCEL:
			return "MSG_KEYUP_CANCEL";
		default:
			sprintf(str, "0x%08x", msg_id);
			return str;
		}

}

void hzw_paint() {
	GUI_LCD_DEV_INFO layer = {GUI_MAIN_LCD_ID, GUI_BLOCK_MAIN};
	GUI_RECT_T rect = {0, 0, 239, 319};

	LCD_FillRect(&layer, rect, MMI_RED_COLOR);
}

MMI_RESULT_E hzw_handle_win_msg(MMI_WIN_ID_T win_id, MMI_MESSAGE_ID_E msg_id, DPARAM param) {
	SCI_TRACE_LOW("hzw_handle_win_msg msg=%s", hzw_get_win_msg_str(msg_id));

	switch (msg_id) {
	case MSG_OPEN_WINDOW:
		break;
	case MSG_PRE_FULL_PAINT:
		break;
	case MSG_FULL_PAINT:
		hzw_paint();
		break;
	case MSG_END_FULL_PAINT:
		break;
	case MSG_KEYREPEAT_WEB:
		break;
	case MSG_KEYUP_WEB:
		break;

	case MSG_KEYDOWN_CANCEL:
		MMK_CloseWin(hzw_win_handle);
		break;
	case MSG_CLOSE_WINDOW:
		break;

	case MSG_KEYDOWN_1:
		hzw_network_active();
		break;

	case MSG_KEYDOWN_2:
		hzw_send_signal(HZW_SIGNAL_REQUEST, (void*)hzw_net_id, hzw_tid);
		break;

	default:
		break;
	}
}

WINDOW_TABLE(hzw_win_table) = {
	WIN_ID(HZW_WIN_ID),
	WIN_FUNC(hzw_handle_win_msg),
	WIN_HIDE_STATUS,
	END_WIN,
};

void hzw_task(uint32 argc, void *argv) {
	HZW_SIGNAL_T *sig = NULL;
	BLOCK_ID tid = SCI_IdentifyThread();

	SCI_TRACE_LOW("hzw_task tid=0x%08x", tid);

	while (1) {
		sig = (HZW_SIGNAL_T*)SCI_GetSignal(tid);
		if (!sig) {
			continue;
		}

		SCI_TRACE_LOW("hzw_task SignalCode=0x%08x", sig->SignalCode);

		switch (sig->SignalCode) {
		case HZW_SIGNAL_REQUEST:
			{
				TCPIP_SOCKET_T sock = TCPIP_SOCKET_INVALID;
				struct sci_sockaddr addr = {0};
				char *send_buf = "GET /ping HTTP/1.1\n"
					"Host: 10.86.111.32:8080\n"
					"\n";
				int send_buf_len = strlen(send_buf);
				int sent_len = 0;
				char recv_buf[256] = {0};
				int len = 0;
				TCPIP_NETID_T net_id = (TCPIP_NETID_T)sig->data;

				sock = sci_sock_socket(AF_INET, SOCK_STREAM, 0, net_id);
				if (sock == TCPIP_SOCKET_INVALID) {
					break;
				}
				addr.family = AF_INET;
				addr.ip_addr = 0x206f560a; // 0x0a566f20;
				addr.port = 0x901f; // 0x1f90;
				if (sci_sock_connect(sock, &addr, 0) == TCPIP_SOCKET_ERROR) {
					break;
				}
				sent_len = sci_sock_send(sock, send_buf, send_buf_len, 0);
				if (sent_len == 0 || sent_len == TCPIP_SOCKET_ERROR) {
					break;
				}
				len = sci_sock_recv(sock, recv_buf, 256, 0);
				if (len == 0 || len == TCPIP_SOCKET_ERROR) {
					break;
				}
				if (sci_sock_socketclose(sock) == TCPIP_SOCKET_ERROR) {
					break;
				}
			}
			break;
		}

		SCI_Free(sig);
	}
}

void hzw_launch() {
	hzw_win_handle = MMK_CreateWin(hzw_win_table, NULL);
	hzw_tid = SCI_CreateThread(
		"T_HZW", // thread name
		"Q_HZW", // queue name
		hzw_task, // entry function
		0, // argc
		NULL, // argv
		4096, // stack size
		8, // queue size
		29, // priority
		SCI_PREEMPT,
		SCI_AUTO_START);
}
```

## log
```
hzw_task tid=0x0339d7b8
hzw_handle_win_msg msg=MSG_OPEN_WINDOW
hzw_handle_win_msg msg=MSG_PRE_FULL_PAINT
hzw_handle_win_msg msg=MSG_FULL_PAINT
hzw_handle_win_msg msg=MSG_END_FULL_PAINT
hzw_handle_win_msg msg=MSG_KEYUP_WEB
hzw_handle_win_msg msg=MSG_KEYDOWN_1
hzw_network_active tid=0x00000015
hzw_network_active_cb tid=0x00000015
hzw_network_active_cb msg_id=MMIPDP_ACTIVE_CNF, result=MMIPDP_RESULT_SUCC, nsapi=0x00000000
hzw_send_signal code=0x00000000
hzw_task SignalCode=0x00000000
hzw_handle_win_msg msg=MSG_KEYUP_1
```
