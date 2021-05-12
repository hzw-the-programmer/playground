```
#define HZW_MODULE_ID

typedef enum {
	HZW_WIN_ID_START = HZW_MODULE_ID << 16,
	HZW_WIN_ID_MAIN,
	HZW_WIN_ID_END,
} HZW_WIN_ID;

typedef enum {
	HZW_CTRL_ID_START = HZW_WIN_ID_END,
	HZW_CTRL_ID_ROOT_FORM,
	HZW_CTRL_ID_SET_TIME_FORM,
	HZW_CTRL_ID_SET_TIME_LABEL,
	HZW_CTRL_ID_SET_TIME_EDIT,
	HZW_CTRL_ID_SET_DATE_FORM,
	HZW_CTRL_ID_SET_DATE_LABEL,
	HZW_CTRL_ID_SET_DATE_EDIT,
	HZW_CTRL_ID_END,
} HZW_CTRL_ID;

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
	MMK_CloseWin(HZW_WIN_ID_MAIN);
}

static void hzwConfigSetTimeCtrl() {
	MMI_STRING_T title = {0};
	SCI_TIME_T time = {0};
	GUIEDIT_TIME_STYLE_E timeStyle = GUIEDIT_TIME_STYLE_12;

	GUIFORM_SetStyle(HZW_CTRL_ID_SET_TIME_FORM, GUIFORM_STYLE_UNIT);

	title.wstr_ptr = L"Set time";
	title.wstr_len = MMIAPICOM_Wstrlen(title.wstr_ptr);
	GUILABEL_SetText(HZW_CTRL_ID_SET_TIME_LABEL, &title, FALSE);

	TM_GetSysTime(&time);
	GUIEDIT_SetTime(HZW_CTRL_ID_SET_TIME_EDIT, time.hour, time.min, 0);
	GUIEDIT_SetTimeStyle(HZW_CTRL_ID_SET_TIME_EDIT, NULL, &timeStyle, NULL, FALSE);
	GUIEDIT_SetAlign(HZW_CTRL_ID_SET_TIME_EDIT, ALIGN_LVMIDDLE);
}

static void hzwConfigSetDateCtrl() {
	MMI_STRING_T title = {0};
	SCI_DATE_T date = {0};
	GUIEDIT_DATE_STYLE_E dateStyle = GUIEDIT_DATE_STYLE_MDY;

	GUIFORM_SetStyle(HZW_CTRL_ID_SET_DATE_FORM, GUIFORM_STYLE_UNIT);

	title.wstr_ptr = L"Set date";
	title.wstr_len = MMIAPICOM_Wstrlen(title.wstr_ptr);
	GUILABEL_SetText(HZW_CTRL_ID_SET_DATE_LABEL, &title, FALSE);

	TM_GetSysDate(&date);
	GUIEDIT_SetDate(HZW_CTRL_ID_SET_DATE_EDIT, date.year, date.mon, date.mday);
	GUIEDIT_SetDateStyle(HZW_CTRL_ID_SET_DATE_EDIT, NULL, &dateStyle, FALSE);
	GUIEDIT_SetAlign(HZW_CTRL_ID_SET_DATE_EDIT, ALIGN_LVMIDDLE);
}

static MMI_RESULT_E hzwWinFuncMain(MMI_WIN_ID_T winId, MMI_MESSAGE_ID_E msgId, DPARAM param) {
	char *msgStr = hzwMsgStr(msgId);

	SCI_TRACE_LOW("hzwWinHandleMsg: %s", msgStr);

	SCI_ASSERT(winId == HZW_WIN_ID_MAIN);

	switch (msgId) {
		case MSG_OPEN_WINDOW:
			GUIFORM_SetType(HZW_CTRL_ID_ROOT_FORM, GUIFORM_TYPE_TP);
			GUIFORM_SetCircularHandleUpDown(HZW_CTRL_ID_ROOT_FORM, TRUE);
			hzwConfigSetTimeCtrl();
			hzwConfigSetDateCtrl();
			MMK_SetAtvCtrl(winId, HZW_CTRL_ID_SET_TIME_EDIT);
			break;
		case MSG_FULL_PAINT:
			hzwPaint();
			break;
		case MSG_KEYUP_RED:
			hzwExit();
			break;
	}
}

WINDOW_TABLE(HZW_WIN_TAB) = {
    WIN_ID(HZW_WIN_ID_MAIN),
    WIN_FUNC((uint32)hzwWinFuncMain),

	CREATE_FORM_CTRL(GUIFORM_LAYOUT_ORDER, HZW_CTRL_ID_ROOT_FORM),

	CHILD_FORM_CTRL(TRUE, GUIFORM_LAYOUT_ORDER, HZW_CTRL_ID_SET_TIME_FORM, HZW_CTRL_ID_ROOT_FORM),
	CHILD_LABEL_CTRL(GUILABEL_ALIGN_LEFT, FALSE, HZW_CTRL_ID_SET_TIME_LABEL, HZW_CTRL_ID_SET_TIME_FORM),
	CHILD_EDIT_TIME_CTRL(TRUE, HZW_CTRL_ID_SET_TIME_EDIT, HZW_CTRL_ID_SET_TIME_FORM),

	CHILD_FORM_CTRL(TRUE, GUIFORM_LAYOUT_ORDER, HZW_CTRL_ID_SET_DATE_FORM, HZW_CTRL_ID_ROOT_FORM),
	CHILD_LABEL_CTRL(GUILABEL_ALIGN_LEFT, FALSE, HZW_CTRL_ID_SET_DATE_LABEL, HZW_CTRL_ID_SET_DATE_FORM),
	CHILD_EDIT_DATE_CTRL(TRUE, HZW_CTRL_ID_SET_DATE_EDIT, HZW_CTRL_ID_SET_DATE_FORM),

    END_WIN
};

void HzwLaunch() {
	MMK_CreateWin(HZW_WIN_TAB, NULL);
	SCI_TRACE_LOW("HzwLaunch: after MMK_CreateWin");
}
```
