MS_Code\MS_MMI\source\mmi_app\app\im\c\wdp_update_wintab.c
OpenUpdateWindow

```
#define HZW_MODULE
#define HZW_WIN_ID
#define HZW_HOST
#define HZW_PORT
#define HZW_PATH
#define HZW_HOST_MAX_LEN 50
#define HZW_PATH_MAX_LEN 50
#define HZW_BUF_LEN 1024
#define HZW_REQ_LINE "%s %s HTTP/1.1\n"
#define HZW_HOST_HEADER "Host: %s:%d\n"
#define HZW_LF "\n"
#define HZW_HEADER_MAX_LEN 50

typedef struct {
	int r;
	int w;
	char buf[HZW_BUF_LEN];
} HZW_BUF;

typedef enum {
	HZW_SIGNAL_CODE_HTTP_REQ,
	HZW_SIGNAL_CODE_NW,
} HZW_SIGNAL_CODE;

typedef struct {
	SIGNAL_VARS
	void *data;
} HZW_SIGNAL;

typedef enum {
	HZW_HTTP_METHOD_GET,
	HZW_HTTP_METHOD_POST,
} HZW_HTTP_METHOD;

typedef enum {
	HZW_SOCK_STATUS_CREATED,
	HZW_SOCK_STATUS_CONNECTING,
	HZW_SOCK_STATUS_CONNECTED,
	HZW_SOCK_STATUS_CLOSED,
} HZW_SOCK_STATUS;

typedef struct {
	char host[HZW_HOST_MAX_LEN];
	unsigned short port;
	HZW_HTTP_METHOD method;
	char path[HZW_PATH_MAX_LEN];
	TCPIP_SOCKET_T sock;
	HZW_SOCK_STATUS sock_status;
	struct HZW_HTTP_REQ *next;
	HZW_BUF wbuf;
	HZW_BUF rbuf;
} HZW_HTTP_REQ;

typedef struct {
	BOOLEAN activated;
	uint32 id;
} HZW_NW;

MMI_HANDLE_T hzw_win_handle = 0;
BLOCK_ID hzw_tid = 0;
SCI_MUTEX_PTR hzw_lock = NULL;

void hzw_buf_init(HZW_BUF *buf) {
	buf->r = 0;
	buf->w = 0;
}

void hzw_buf_write(HZW_BUF *buf, char *data, int len) {
	SCI_MEMCPY(buf->buf + buf->w, data, len);
	buf->w += len;
}

void hzw_buf_read(HZW_BUF *buf, char *data, int *len) {
	int l = buf->w - buf->r;

	if (l > len) {
		l = len;
	}
	SCI_MEMCPY(data, buf->buf + buf->r, l);
	buf->r += l;
	*len = l;
}

void hzw_buf_write_to_sock(HZW_BUF *buf, TCPIP_SOCKET_T sock) {
	int len = buf->w - buf->r;

	SCI_ASSERT(len >= 0);
	if (!len) {
		buf->w = buf->r = 0;
		return;
	}

	len = sci_sock_send(sock, buf->buf + buf->r, len, 0);
	if (len == 0 || len == TCPIP_SOCKET_ERROR) {
		return;
	}

	buf->r += len;
}

void hzw_buf_read_from_sock(HZW_BUF *buf, TCPIP_SOCKET_T sock) {
	int len = buf->w - buf->r;
	int left_len = 0;

	if (!len) {
		buf->w = buf->r = 0;
	} else if (buf->r > 0) {
		SCI_MEMCPY(buf->buf, buf->buf + buf->r, len);
		buf->r = 0;
	}

	do {
		len = sci_sock_asyncrecv(sock, buf->buf + buf->w, HZW_BUF_LEN - len, 0, &left_len);
		if (len == 0 || len == TCPIP_SOCKET_ERROR) {
			break;
		}
		buf->w += len;
	} while (left_len);
}

void hzw_paint() {
	GUI_LCD_DEV_INFO dev_info = {GUI_MAIN_LCD_ID, GUI_BLOCK_MAIN};
	GUI_RECT_T box = {0, 0, 239, 319};
	GUI_COLOR_T color = MMI_RED_COLOR;

	LCD_FillRect(&dev_info, box, color);
}

void hzw_critical() {
	BLOCK_ID tid = SCI_IdentifyThread();
	SCI_GetMutex(hzw_lock, SCI_WAIT_FOREVER);
	SCI_SLEEP(10000);
	SCI_PutMutex(hzw_lock);
}

BOOLEAN hzw_send_signal(BLOCK_ID tid, HZW_SIGNAL_CODE code, void *data) {
	HZW_SIGNAL *s = NULL;

	s = SCI_ALLOC(sizeof(HZW_SIGNAL));
	if (!s) {
		return FALSE;
	}

	s->SignalCode = code;
	s->SignalSize = sizeof(xSignalHeaderRec);
	s->Sender = NULL;
	s->data = data;

	if (SCI_SendSignal((xSignalHeader)s, tid)) {
		SCI_Free(s);
		return FALSE;
	}

	return TRUE;
}

BOOLEAN hzw_send_http_req(char *host,
	unsigned short port,
	HZW_HTTP_METHOD method,
	char *path) {
	HZW_HTTP_REQ *req = NULL;

	req = SCI_ALLOC(sizeof(HZW_HTTP_REQ));
	if (!req) {
		return FALSE;
	}

	strcpy(req->host, host);
	req->port = port;
	req->method = method;
	strcpy(req->path, path);

	if (!hzw_send_signal(hzw_tid, HZW_SIGNAL_CODE_HTTP_REQ, req)) {
		SCI_Free(req);
		return FALSE;
	}

	return TRUE;
}

BOOLEAN hzw_send_nw(BOOLEAN activated, uint32 id) {
	HZW_NW *nw = NULL;

	nw = SCI_ALLOC(sizeof(HZW_NW));
	if (!nw) {
		return FALSE;
	}

	nw->activated = activated;
	nw->id = id;

	if (!hzw_send_signal(hzw_tid, HZW_SIGNAL_CODE_NW, nw)) {
		SCI_Free(nw);
		return FALSE;
	}

	return TRUE;
}

void hzw_handle_msg(MMIPDP_CNF_INFO_T *msg_ptr) {
	switch(msg_ptr->msg_id)
	{
	case MMIPDP_ACTIVE_CNF:
		if(MMIPDP_RESULT_SUCC == msg_ptr->result)
		{
			hzw_send_nw(TRUE, msg_ptr->nsapi);
		}
		break;
	}
}

void hzw_activate_nw() {
	MMIPDP_ACTIVE_INFO_T info = {0};
	uint16 sim_sys = MN_DUAL_SYS_1;
	uint8 setting_index = 0;
	MMICONNECTION_LINKSETTING_DETAIL_T *setting_item_ptr = NULL;

	if (MMIAPISET_GetMultiSysSettingNum(&sim_sys, 1) <= 0) {
		return;
	}

	setting_index = MMIBROWSER_GetNetSettingIndex(sim_sys);
	setting_item_ptr = MMICONNECTION_GetLinkSettingItemByIndex(sim_sys,setting_index);

	info.app_handler = HZW_MODULE;
	info.auth_type = setting_item_ptr->auth_type;
	info.apn_ptr = (char*)setting_item_ptr->apn;
	info.user_name_ptr = (char*)setting_item_ptr->username;
	info.psw_ptr = (char*)setting_item_ptr->password;
	info.dual_sys = sim_sys;
	info.priority = 3;
	info.ps_service_rat = MN_UNSPECIFIED;
	info.handle_msg_callback = hzw_handle_msg;
	info.ps_service_type = IM_E;
	info.storage = MN_GPRS_STORAGE_ALL;
	MMIAPIPDP_Active(&info);
}

MMI_RESULT_E hzw_win_func(MMI_WIN_ID_T win_id, MMI_MESSAGE_ID_E msg_id, DPARAM param) {
	switch (msg_id) {
	case MSG_FULL_PAINT:
		hzw_paint();
		break;
	case MSG_KEYUP_1:
		hzw_send_http_req(HZW_HOST,
			HZW_PORT,
			HZW_HTTP_METHOD_GET,
			HZW_PATH);
		break;
	case MSG_KEYUP_2:
		hzw_activate_nw();
		break;
	}
}

WINDOW_TABLE(hzw_window) = {
	WIN_ID(HZW_WIN_ID),
	WIN_FUNC(hzw_win_func),
	WIN_HIDE_STATUS,
	END_WIN,
};

void hzw_create_window() {
	hzw_win_handle = MMK_CreateWin(hzw_window, NULL);
}

static BOOLEAN hzw_nw_activated = FALSE;
static uint32 hzw_nw_id = 0;
static HZW_HTTP_REQ *hzw_http_req = NULL;

void hzw_add_http_req(HZW_HTTP_REQ *req) {
	HZW_HTTP_REQ **r = &hzw_http_req;

	while (*r) {
		r = &((*r)->next);
	}

	req->next = NULL;
	*r = req;
}

void hzw_activate_http_req(HZW_HTTP_REQ *req) {
	TCPIP_SOCKET_T sock = req->sock;
	struct sci_sockaddr addr = {0};

	SCI_ASSERT(sci_sock_modifynetid(sock, hzw_nw_id) == 0);
	addr.family = AF_INET;
	SCI_ASSERT(inet_aton(req->host, &addr.ip_addr) == 1);
	addr.port = htons(req->port);
	SCI_MEMSET(addr.sa_data, 0, sizeof(addr.sa_data));

	SCI_ASSERT(sci_sock_connect(sock, &addr, 0) == -1);
	SCI_ASSERT(sci_sock_errno(sock) == EINPROGRESS);

	req->sock_status = HZW_SOCK_STATUS_CONNECTING;
}

char* hzw_get_http_method(HZW_HTTP_METHOD method) {
	switch (method) {
	case HZW_HTTP_METHOD_GET:
		return "GET";
	case HZW_HTTP_METHOD_POST:
		return "POST";
	}
}

void hzw_process_http_req(HZW_HTTP_REQ *req) {
	BLOCK_ID tid = SCI_IdentifyThread();
	TCPIP_SOCKET_T sock = TCPIP_SOCKET_INVALID;
	char buf[HZW_HEADER_MAX_LEN] = {0};

	sock = sci_sock_socket(AF_INET, SOCK_STREAM, 0, 0);
	SCI_ASSERT(sock != TCPIP_SOCKET_INVALID);
	SCI_ASSERT(sci_sock_asyncselect(sock, tid, AS_CONNECT | AS_READ | AS_WRITE | AS_CLOSE) != -1);

	req->sock = sock;
	req->sock_status = HZW_SOCK_STATUS_CREATED;

	hzw_buf_init(&req->wbuf);
	hzw_buf_init(&req->rbuf);

	sprintf(buf, HZW_REQ_LINE, hzw_get_http_method(req->method), req->path);
	hzw_buf_write(&req->wbuf, buf, strlen(buf));

	sprintf(buf, HZW_HOST_HEADER, req->host, req->port);
	hzw_buf_write(&req->wbuf, buf, strlen(buf));

	hzw_buf_write(&req->wbuf, HZW_LF, strlen(HZW_LF));

	hzw_add_http_req(req);

	if (!hzw_nw_activated) {
		return;
	}

	hzw_activate_http_req(req);
}

void hzw_process_nw(HZW_NW *nw) {
	HZW_HTTP_REQ *req = hzw_http_req;

	hzw_nw_activated = nw->activated;
	hzw_nw_id = nw->id;
	SCI_Free(nw);

	while (req) {
		if (req->sock_status == HZW_SOCK_STATUS_CREATED) {
			hzw_activate_http_req(req);
		}
		req = req->next;
	}
}

HZW_HTTP_REQ* hzw_get_http_req(TCPIP_SOCKET_T sock) {
	HZW_HTTP_REQ *req = hzw_http_req;

	while (req && req->sock != sock) {
		req = req->next;
	}

	return req;
}

void hzw_process_connect(SOCKET_CONNECT_EVENT_IND_SIG_T *sig) {
	TCPIP_SOCKET_T sock = sig->socket_id;
	HZW_HTTP_REQ *req = hzw_get_http_req(sock);

	SCI_ASSERT(req);

	req->sock_status = HZW_SOCK_STATUS_CONNECTED;

	hzw_buf_write_to_sock(&req->wbuf, sock);
}

void hzw_process_read(SOCKET_READ_EVENT_IND_SIG_T *sig) {
	TCPIP_SOCKET_T sock = sig->socket_id;
	HZW_HTTP_REQ *req = hzw_get_http_req(sock);

	SCI_ASSERT(req);

	hzw_buf_read_from_sock(&req->rbuf, sock);
}

void hzw_process_write(SOCKET_WRITE_EVENT_IND_SIG_T *sig) {

}

void hzw_process_close(SOCKET_CONNECTION_CLOSE_EVENT_IND_SIG_T *sig) {
	TCPIP_SOCKET_T sock = sig->socket_id;
	HZW_HTTP_REQ *req = hzw_get_http_req(sock);

	SCI_ASSERT(req);

	req->sock_status = HZW_SOCK_STATUS_CLOSED;
}

void hzw_entry(uint32 argc, void *argv) {
	BLOCK_ID tid = SCI_IdentifyThread();
	xSignalHeaderRec *shr = NULL;

	while (1) {
		shr = SCI_GetSignal(tid);

		switch (shr->SignalCode) {
			case HZW_SIGNAL_CODE_HTTP_REQ:
				hzw_process_http_req((HZW_HTTP_REQ*)(((HZW_SIGNAL*)shr)->data));
				break;
			case HZW_SIGNAL_CODE_NW:
				hzw_process_nw((HZW_NW*)(((HZW_SIGNAL*)shr)->data));
				break;
			case SOCKET_CONNECT_EVENT_IND:
				hzw_process_connect(shr);
				break;
			case SOCKET_READ_EVENT_IND:
				hzw_process_read(shr);
				break;
			case SOCKET_WRITE_EVENT_IND:
				hzw_process_write(shr);
				break;
			case SOCKET_CONNECTION_CLOSE_EVENT_IND:
				hzw_process_close(shr);
				break;
		}

		SCI_Free(shr);
	}
}

void hzw_create_thread() {
	hzw_tid = SCI_CreateThread("HZWTN",
		"HZWQN",
		hzw_entry,
		0,
		NULL,
		4096,
		8,
		29,
		SCI_PREEMPT,
		SCI_AUTO_START);
}

void hzw_launch() {
	hzw_lock = SCI_CreateMutex("hzw_lock", SCI_INHERIT);
	hzw_create_window();
	hzw_create_thread();
}
```
