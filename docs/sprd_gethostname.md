```
#define HZW_MODULE_ID
#define HZW_WIN_ID

static BLOCK_ID hzwTid = 0;
static uint32 hzwNid = 0;
static TCPIP_HOST_HANDLE hzwHostHandle = 0;

static char* hzwKeyCodeStr(MMI_MESSAGE_ID_E msgId) {
	switch (msgId & 0x00ff) {
		case KEY_OK:
			return "KEY_OK";
		case KEY_CANCEL:
			return "KEY_CANCEL";
		case KEY_GREEN:
			return "KEY_GREEN";
		case KEY_RED:
			return "KEY_RED";

		case KEY_WEB:
			return "KEY_WEB";

		case KEY_UP:
			return "KEY_UP";
		case KEY_DOWN:
			return "KEY_DOWN";
		case KEY_LEFT:
			return "KEY_LEFT";
		case KEY_RIGHT:
			return "KEY_RIGHT";

		case KEY_1:
			return "KEY_1";
		case KEY_2:
			return "KEY_2";
		case KEY_3:
			return "KEY_3";

		case KEY_4:
			return "KEY_4";
		case KEY_5:
			return "KEY_5";
		case KEY_6:
			return "KEY_6";

		case KEY_7:
			return "KEY_7";
		case KEY_8:
			return "KEY_8";
		case KEY_9:
			return "KEY_9";

		case KEY_STAR:
			return "KEY_STAR";
		case KEY_0:
			return "KEY_0";
		case KEY_HASH:
			return "KEY_HASH";

		default:
			return NULL;
	}
}

static char* hzwKeyEventStr(MMI_MESSAGE_ID_E msgId) {
	switch (msgId & 0xff00) {
		case KEY_PRESSED:
			return "KEY_PRESSED";
		case KEY_RELEASED:
			return "KEY_RELEASED";
		default:
			return NULL;
	}
}

static char* hzwKeyMsgStr(MMI_MESSAGE_ID_E msgId) {
	static char str[100] = {0};
	char *keyCodeStr = hzwKeyCodeStr(msgId);
	char *keyEventStr = hzwKeyEventStr(msgId);

	if (!keyCodeStr || !keyEventStr) {
		return NULL;
	}

	sprintf(str, "%s %s", keyCodeStr, keyEventStr);

	return str;
}

static char* hzwMsgStr(MMI_MESSAGE_ID_E msgId) {
	switch (msgId) {
		//applet
		case MSG_UPDATE_SCREEN:
			return "MSG_UPDATE_SCREEN";
		case MSG_APPLET_START:
			return "MSG_APPLET_START";
		case MSG_APPLET_STOP:
			return "MSG_APPLET_STOP";
		case MSG_APPLET_SUSPEND:
			return "MSG_APPLET_SUSPEND";
		case MSG_APPLET_RESUME:
			return "MSG_APPLET_RESUME";
		case MSG_APPLET_SWITCH:
			return "MSG_APPLET_SWITCH";
		case MSG_APPLET_CLEAR_FREE_MODULE:
			return "MSG_APPLET_CLEAR_FREE_MODULE";
		case MSG_APPLET_RESOLVE_CONFLICT:
			return "MSG_APPLET_RESOLVE_CONFLICT";

		//timer
		case MSG_TIMER:
			return "MSG_TIMER";

		//window
		case MSG_OPEN_WINDOW:
			return "MSG_OPEN_WINDOW";
		case MSG_CLOSE_WINDOW:
			return "MSG_CLOSE_WINDOW";
		case MSG_LOSE_FOCUS:
			return "MSG_LOSE_FOCUS";
		case MSG_GET_FOCUS:
			return "MSG_GET_FOCUS";
		case MSG_FULL_PAINT:
			return "MSG_FULL_PAINT";
		case MSG_LCD_SWITCH:
			return "MSG_LCD_SWITCH";
		case MSG_PRE_FULL_PAINT:
			return "MSG_PRE_FULL_PAINT";
		case MSG_END_FULL_PAINT:
			return "MSG_END_FULL_PAINT";
		case MSG_PRE_LCD_SWITCH:
			return "MSG_PRE_LCD_SWITCH";

		default: {
			char *str = hzwKeyMsgStr(msgId);
			if (str) return str;
			return "DEFAULT";
		}
	}
}

static void hzwPaint() {
	GUI_LCD_DEV_INFO dev = {GUI_MAIN_LCD_ID, GUI_BLOCK_MAIN};
	GUI_RECT_T rect = {0, 0, MMI_MAINSCREEN_WIDTH, MMI_MAINSCREEN_HEIGHT};
	MMI_STRING_T text = {0};
	GUISTR_STYLE_T style = {0};
	GUISTR_STATE_T state = 0;

	LCD_FillRect(&dev, rect, MMI_WINDOW_BACKGROUND_COLOR);

	style.font = MMI_DEFAULT_NORMAL_FONT;
	style.font_color = MMI_WHITE_COLOR;
	//style.align = ALIGN_HVMIDDLE;
	state = GUISTR_STATE_SINGLE_LINE | GUISTR_STATE_ELLIPSIS;

	text.wstr_ptr = L"Hello World!";
	text.wstr_len = MMIAPICOM_Wstrlen(text.wstr_ptr);

	GUISTR_DrawTextToLCDInRect(&dev,
		&rect,
		&rect,
		&text,
		&style,
		state,
		GUISTR_TEXT_DIR_AUTO);
}

static void hzwExit() {
	MMK_CloseWin(HZW_WIN_ID);
}

static char* hzwGetInterfaceStr(MMIPDP_INTERFACE_E interface) {
	switch (interface) {
		case MMIPDP_INTERFACE_GPRS:
			return "MMIPDP_INTERFACE_GPRS";
		case MMIPDP_INTERFACE_WIFI:
			return "MMIPDP_INTERFACE_WIFI";
		case MMIPDP_INTERFACE_MAX:
			return "MMIPDP_INTERFACE_MAX";
		default:
			return "DEFAULT";
	}
}

static char* hzwGetIdStr(MMIPDP_APP_MSG_E id) {
	switch (id) {
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
			return "DEFAULT";
	}
}

static char* hzwGetResultStr(MMIPDP_RESULT_E result) {
	switch (result) {
		case MMIPDP_RESULT_SUCC:
			return "MMIPDP_RESULT_SUCC";
		case MMIPDP_RESULT_FAIL:
			return "MMIPDP_RESULT_FAIL";
#ifndef GPRSMPDP_SUPPORT
		case MMIPDP_RESULT_AT_IS_ON: //正在使用AT拨号
			return "MMIPDP_RESULT_AT_IS_ON";
#endif
		case MMIPDP_RESULT_DIFFER_DUALSYS:
			return "MMIPDP_RESULT_DIFFER_DUALSYS";
		case MMIPDP_RESULT_TIMEOUT:
			return "MMIPDP_RESULT_TIMEOUT";
		case MMIPDP_RESULT_NOT_PERMIT: //不允许使用网络，比如飞行模式
			return "MMIPDP_RESULT_NOT_PERMIT";
#ifdef DATA_ROAMING_SUPPORT
		case MMIPDP_RESULT_NOT_PERMIT_ROAMING: //漫游模式关闭网络(added by feng.xiao)
			return "MMIPDP_RESULT_NOT_PERMIT_ROAMING";
#endif
		case MMIPDP_RESULT_FDN_NOT_PERMIT: //不允许使用网络，FDN only
			return "MMIPDP_RESULT_FDN_NOT_PERMIT";
		default:
			return "DEFAULT";
	}
}

static void hzwThreadEntry(uint32 argc, void *argv) {
	BLOCK_ID tid = SCI_IdentifyThread();
	xSignalHeaderRec *sig = NULL;

	while (1) {
		sig = SCI_GetSignal(tid);
		switch (sig->SignalCode) {
			case SOCKET_ASYNC_GETHOSTBYNAME_CNF: {
				ASYNC_GETHOSTBYNAME_CNF_SIG_T *cnf = (ASYNC_GETHOSTBYNAME_CNF_SIG_T*)sig;
				TCPIP_IPADDR_T ip = 0;
				SCI_TRACE_LOW("hzwThreadEntry: error=%d, reqId=%d", cnf->error_code, cnf->request_id);
				if (cnf->error_code != 0 || cnf->request_id != hzwHostHandle) {
					break;
				}
				SCI_Memcpy(&ip, cnf->hostent.h_addr_list[0], cnf->hostent.h_length);
				SCI_TRACE_LOW("hzwThreadEntry: ip=%s", inet_ntoa(ip));
				break;
			}
		}
	}
}

static void hzwActiveNetworkCb(MMIPDP_CNF_INFO_T *msg) {
	char *interfaceStr = hzwGetInterfaceStr(msg->ps_interface);
	char *idStr = hzwGetIdStr(msg->msg_id);
	char *resultStr = hzwGetResultStr(msg->result);

	SCI_TRACE_LOW("hzwActiveNetworkCb: interface=%s, id=%s, result=%s, nsapi=%d",
		interfaceStr, idStr, resultStr, msg->nsapi);

	hzwNid = msg->nsapi;
}

static void hzwActiveNetwork() {
	MN_DUAL_SYS_E sim = MN_DUAL_SYS_MAX;
	uint8 index = 0;
	MMICONNECTION_LINKSETTING_DETAIL_T *item = NULL;
	MMIPDP_ACTIVE_INFO_T info = {0};
	BOOLEAN result = FALSE;

	sim = MN_DUAL_SYS_1;
	index = MMIAPIBROWSER_GetNetSettingIndex(sim);
	SCI_TRACE_LOW("hzwActiveNetwork: index=%d", index);
	item = MMIAPICONNECTION_GetLinkSettingItemByIndex(sim, index);
	SCI_TRACE_LOW("hzwActiveNetwork: item=%x", item);

	info.app_handler = HZW_MODULE_ID;
	info.auth_type = item->auth_type;
	info.apn_ptr = item->apn;
	info.user_name_ptr = item->username;
	info.psw_ptr = item->password;
	info.dual_sys = sim;
	info.priority = 3;
	info.ps_service_rat = MN_UNSPECIFIED;
	info.handle_msg_callback = hzwActiveNetworkCb;
	info.ps_service_type = IM_E;
	info.storage = MN_GPRS_STORAGE_ALL;

	result = MMIAPIPDP_Active(&info);
	SCI_TRACE_LOW("hzwActiveNetwork: result=%d", result);
}

static void hzwGetHostByName() {
	hzwHostHandle = sci_async_gethostbyname("www.baidu.com", hzwTid, 1000, hzwNid);
	SCI_TRACE_LOW("hzwGetHostByName: hzwTid=%d, hzwNid=%d, handle=%d", hzwTid, hzwNid, hzwHostHandle);
}

static MMI_RESULT_E hzwWinHandleMsg(MMI_WIN_ID_T winId, MMI_MESSAGE_ID_E msgId, DPARAM param) {
	char *msgStr = hzwMsgStr(msgId);

	SCI_TRACE_LOW("hzwWinHandleMsg: %s", msgStr);

	SCI_ASSERT(winId == HZW_WIN_ID);

	switch (msgId) {
		case MSG_FULL_PAINT:
			hzwPaint();
			break;
		case MSG_KEYUP_RED:
			hzwExit();
			break;
		case MSG_KEYUP_1:
			hzwActiveNetwork();
			break;
		case MSG_KEYUP_2:
			hzwGetHostByName();
			break;
	}
}

WINDOW_TABLE(HZW_WIN_TAB) = {
    WIN_ID(HZW_WIN_ID),
	WIN_HIDE_STATUS,
    WIN_FUNC((uint32)hzwWinHandleMsg),
    END_WIN
};

void HzwLaunch() {
	MMK_CreateWin(HZW_WIN_TAB, NULL);
	SCI_TRACE_LOW("HzwLaunch: after MMK_CreateWin");
	hzwTid = SCI_CreateThread("T_HZW",
							"Q_HZW",
							hzwThreadEntry,
							0,
							NULL,
							4096,
							8,
							20, //SCI_PRIORITY_NORMAL,
							SCI_PREEMPT,
							SCI_AUTO_START);
}
```

```
000000130023		13-23:HzwLaunch: after MMK_CreateWin
000000130030		13-30:hzwWinHandleMsg: MSG_OPEN_WINDOW
000000130034		13-34:hzwWinHandleMsg: MSG_PRE_FULL_PAINT
000000130037		13-37:hzwWinHandleMsg: MSG_FULL_PAINT
000000130040		13-40:hzwWinHandleMsg: MSG_END_FULL_PAINT
000000150040		15-40:hzwWinHandleMsg: KEY_WEB KEY_RELEASED
000000320030		32-30:hzwWinHandleMsg: KEY_1 KEY_PRESSED
000000330036		33-36:hzwWinHandleMsg: KEY_1 KEY_RELEASED
000000330037		33-37:hzwActiveNetwork: index=0
000000330038		33-38:hzwActiveNetwork: item=4627cd8
000000360023		36-23:hzwActiveNetwork: result=1
000000910029		91-29:hzwActiveNetworkCb: interface=MMIPDP_INTERFACE_GPRS, id=MMIPDP_ACTIVE_CNF, result=MMIPDP_RESULT_SUCC, nsapi=6
000001370026		137-26:hzwWinHandleMsg: KEY_2 KEY_PRESSED
000001380017		138-17:hzwWinHandleMsg: KEY_2 KEY_RELEASED
000001380022		138-22:hzwGetHostByName: hzwTid=67694140, hzwNid=6, handle=1
000001650004		165-4:hzwThreadEntry: error=0, reqId=1
000001650005		165-5:hzwThreadEntry: ip=36.152.44.95
000002340021		234-21:hzwWinHandleMsg: KEY_2 KEY_PRESSED
000002340036		234-36:hzwWinHandleMsg: KEY_2 KEY_RELEASED
000002340039		234-39:hzwGetHostByName: hzwTid=67694140, hzwNid=6, handle=2
000002350042		235-42:hzwThreadEntry: error=0, reqId=2
000002350043		235-43:hzwThreadEntry: ip=36.152.44.95
000002630007		263-7:hzwWinHandleMsg: MSG_LOSE_FOCUS
000003020010		302-10:hzwWinHandleMsg: MSG_GET_FOCUS
000003020019		302-19:hzwWinHandleMsg: MSG_PRE_FULL_PAINT
000003020022		302-22:hzwWinHandleMsg: MSG_FULL_PAINT
000003020055		302-55:hzwWinHandleMsg: MSG_END_FULL_PAINT
000003030039		303-39:hzwWinHandleMsg: KEY_CANCEL KEY_RELEASED
000003280059		328-59:hzwWinHandleMsg: KEY_2 KEY_PRESSED
000003290044		329-44:hzwWinHandleMsg: KEY_2 KEY_RELEASED
000003290047		329-47:hzwGetHostByName: hzwTid=67694140, hzwNid=6, handle=3
000003330015		333-15:hzwThreadEntry: error=0, reqId=3
000003330016		333-16:hzwThreadEntry: ip=36.152.44.95
```
