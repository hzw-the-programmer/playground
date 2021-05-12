```
#define HZW_MODULE_ID
#ifdef WIN32
#define HZW_DEBUG
#else
#define HZW_DEBUG(fmt, ...) SCI_TRACE_LOW("HZW_DEBUG: " fmt, ##__VA_ARGS__)
#endif

typedef enum {
	HZW_WIN_ID_START = HZW_MODULE_ID << 16,
	HZW_WIN_ID_MAIN,
	HZW_WIN_ID_END,
} HZW_WIN_ID;

typedef enum {
	HZW_CTRL_ID_START = HZW_WIN_ID_END,
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

#include "mmipb_app.h"
#include "mmipb_menu.h"

static void hzwAddContactUpdateCallback(
    MMI_HANDLE_T applet_handle,
    uint16  msg_id,
    MMIPB_ERROR_E error,
    MMIPB_OPC_E opc_type,
    uint16 count)
{
    MMIPB_APPLET_T* applet_ptr = (MMIPB_APPLET_T*)MMK_GetAppletPtr(applet_handle);
    if(applet_ptr != PNULL)
    {
        MMIPB_CloseApplet(applet_ptr->caf_applet.app_handle);
    }
}

static void hzwAddContact(wchar *name, char *num) {
    MMIPB_INSTANCE_T pb_instance = {0};
    MMI_HANDLE_T applet_handle = 0;
    MMIPB_APPLET_T *applet_ptr = PNULL;
    MMIPB_MENU_DATA_INFOR_T add_data = {0};
    MMIPB_BCD_NUMBER_T bcd_num = {0};

    MMIAPIPB_StringToBCDNumber(num, &bcd_num);

    pb_instance.update_callback = hzwAddContactUpdateCallback;
    pb_instance.function_type = MMIPB_ADD_ENTRY;
    applet_handle = MMIPB_StartApplet(&pb_instance);

    applet_ptr = (MMIPB_APPLET_T*)MMK_GetAppletPtr(applet_handle);
    if(PNULL == applet_ptr)
    {
        MMIPB_CloseApplet(applet_handle);
        return;
    }

    add_data.applet_handle = applet_handle;
    add_data.operate_type = MMIPB_OPERATE_TYPE_ADD_ONE;
    add_data.contact_ptr =SCI_ALLOCA(sizeof(MMIPB_CONTACT_T));
    if(PNULL == add_data.contact_ptr)
    {
        MMIPB_CloseApplet(applet_handle);
        return;
    }
    SCI_MEMSET(add_data.contact_ptr, 0x00, sizeof(MMIPB_CONTACT_T));
    add_data.contact_ptr->storage_id = 0;
    add_data.need_reset_storage = TRUE;
    SCI_MEMCPY(&add_data.contact_ptr->number[0], &bcd_num, sizeof(MMIPB_BCD_NUMBER_T));
    add_data.contact_ptr->num_type[0] = MMIPB_ADN_NUMBER;
    add_data.contact_ptr->name.wstr_len = MMIAPICOM_Wstrlen(name);
    MMIAPICOM_Wstrcpy(add_data.contact_ptr->name.wstr, name);
    add_data.contact_ptr->union_t.group =  0;

    if ((!MMIPB_GetProtectStatus()) && (MMIAPISET_IsOpenPrivacyProtect(MMISET_PROTECT_PB)))
    {
        MMIAPISET_ValidatePrivacy(
            MMISET_PROTECT_PB,
            MMIPB_AddOrEditContactOfPbExt,
            (DPARAM)&add_data,
            sizeof(MMIPB_MENU_DATA_INFOR_T));
    }
    else
    {
        MMIPB_AddOrEditContactOfPb(&add_data);
        SCI_FREE(add_data.contact_ptr);
    }
}

static MMI_RESULT_E hzwWinFuncMain(MMI_WIN_ID_T winId, MMI_MESSAGE_ID_E msgId, DPARAM param) {
	char *msgStr = hzwMsgStr(msgId);

	HZW_DEBUG("hzwWinFuncMain: %s", msgStr);

	SCI_ASSERT(winId == HZW_WIN_ID_MAIN);

	switch (msgId) {
		case MSG_FULL_PAINT:
			hzwPaint();
			break;
		case MSG_KEYUP_RED:
			hzwExit();
			break;
        case KEY_1 | KEY_RELEASED:
            hzwAddContact(L"hzw", "12345");
            break;
	}
}

WINDOW_TABLE(HZW_WIN_TAB_MAIN) = {
    WIN_ID(HZW_WIN_ID_MAIN),
	WIN_HIDE_STATUS,
    WIN_FUNC((uint32)hzwWinFuncMain),
    END_WIN
};

void HzwLaunch() {
	MMK_CreateWin(HZW_WIN_TAB_MAIN, NULL);
	HZW_DEBUG("HzwLaunch: after MMK_CreateWin");
}
```
