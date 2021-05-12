```
MMK_OpenWin(unsigned long 17498122, unsigned long 0) line 2108
AppletCreateWindow(void * 0x1c54fa5c, unsigned char 1) line 320 + 15 bytes
MMK_CreateWinTable(const MMI_WINDOW_TABLE_CREATE_T * 0x1c54facc) line 216 + 11 bytes
MMK_CreateWin(unsigned long * 0x01c9a5e8 _MMISET_SET_PHONE_WIN_TAB, void * 0x00000000) line 7136 + 9 bytes
MMIAPISET_EnterPhoneSettingWin() line 2612 + 12 bytes
HandleMainMenuWinMsg(unsigned long 2, unsigned long 57345, void * 0x07798484) line 1849
MMK_RunWinProc(unsigned long 17104900, unsigned long 57345, void * 0x07798484) line 4796 + 23 bytes
MMK_DispatchToHandle(unsigned long 17104900, unsigned long 57345, void * 0x07798484) line 911 + 17 bytes
MMK_DispatchWinMSG(Message_tag * 0x07796a6c) line 686 + 25 bytes
MMK_DispatchMSGQueue(Message_tag * 0x07796a6c) line 394 + 9 bytes
APP_Task(unsigned long 0, void * 0x00000000) line 2141 + 9 bytes
MSDEVKERNEL! SCI_CreateThread line 856 + 11 bytes
MSDEVKERNEL! _tx_thread_shell_entry line 100 + 18 bytes
MSDEVKERNEL! _tx_win32_thread_entry@4 line 164
KERNEL32! 75c1343d()
NTDLL! 775d9812()
NTDLL! 775d97e5()
```
```
MMK_OpenWin(unsigned long 16908289, unsigned long 0) line 2108
AppletCreateWindow(void * 0x1c52f4b4, unsigned char 0) line 320 + 15 bytes
MMK_CreateWindow(const MMI_WINDOW_CREATE_T * 0x1c52f538) line 240 + 11 bytes
MMIAPIIDLE_OpenIdleWin() line 1868 + 9 bytes
PlayPowerOnOffMP4CallBack(int 1) line 1289
HandleNormalStartupWindow(unsigned long 31, unsigned long 61472, void * 0x1c52fd94) line 560 + 7 bytes
MMK_RunWinProc(unsigned long 16777217, unsigned long 61472, void * 0x1c52fd94) line 4796 + 23 bytes
MMK_DispatchToHandle(unsigned long 16777217, unsigned long 61472, void * 0x1c52fd94) line 911 + 17 bytes
MMK_DispatchMSGTimer(Signal * 0x074aea3c) line 748 + 32 bytes
MMK_DispatchExtSig(MmiSignalSTag * * 0x1c52fe64) line 626 + 14 bytes
APP_Task(unsigned long 0, void * 0x00000000) line 2206 + 9 bytes
MSDEVKERNEL! SCI_CreateThread line 856 + 11 bytes
MSDEVKERNEL! _tx_thread_shell_entry line 100 + 18 bytes
MSDEVKERNEL! _tx_win32_thread_entry@4 line 164
KERNEL32! 75c1343d()
NTDLL! 775d9812()
NTDLL! 775d97e5()
```

## example
```
MMI_RESULT_E hzw_handle_win_msg(MMI_WIN_ID_T win_id, MMI_MESSAGE_ID_E msg_id, DPARAM param) {
	char *msg = NULL;
	switch (msg_id) {
	case MSG_OPEN_WINDOW:
		msg = "MSG_OPEN_WINDOW";
		break;
	case MSG_PRE_FULL_PAINT:
		msg = "MSG_PRE_FULL_PAINT";
		break;
	case MSG_FULL_PAINT:
		msg = "MSG_FULL_PAINT";
		{
			GUI_LCD_DEV_INFO layer = {GUI_MAIN_LCD_ID, GUI_BLOCK_MAIN};
			GUI_RECT_T rect = {0, 0, 239, 319};

			LCD_FillRect(&layer, rect, MMI_RED_COLOR);
		}
		break;
	case MSG_END_FULL_PAINT:
		msg = "MSG_END_FULL_PAINT";
		break;
	case MSG_KEYREPEAT_WEB:
		msg = "MSG_KEYREPEAT_WEB";
		break;
	case MSG_KEYUP_WEB:
		msg = "MSG_KEYUP_WEB";
		break;

	case MSG_APP_CANCEL:
		msg = "MSG_APP_CANCEL";
		MMK_CloseWin(hzw_win_id);
		break;
	case MSG_CLOSE_WINDOW:
		msg = "MSG_CLOSE_WINDOW";
		break;
	}

	SCI_TRACE_LOW("hzw_handle_win_msg msg=%s", msg);
}

WINDOW_TABLE(hzw_win_table) = {
	WIN_ID(HZW_WIN_ID),
	WIN_FUNC(hzw_handle_win_msg),
	WIN_HIDE_STATUS,
	END_WIN,
};

void hzw_launch() {
	hzw_win_id = MMK_CreateWin(hzw_win_table, NULL);
}
```

## log
```
hzw_handle_win_msg msg=MSG_OPEN_WINDOW
hzw_handle_win_msg msg=MSG_PRE_FULL_PAINT
hzw_handle_win_msg msg=MSG_FULL_PAINT
hzw_handle_win_msg msg=MSG_END_FULL_PAINT
hzw_handle_win_msg msg=MSG_KEYUP_WEB
hzw_handle_win_msg msg=MSG_CLOSE_WINDOW
hzw_handle_win_msg msg=MSG_APP_CANCEL

hzw_handle_win_msg msg=MSG_OPEN_WINDOW
hzw_handle_win_msg msg=MSG_PRE_FULL_PAINT
hzw_handle_win_msg msg=MSG_FULL_PAINT
hzw_handle_win_msg msg=MSG_END_FULL_PAINT
hzw_handle_win_msg msg=MSG_KEYUP_WEB
hzw_handle_win_msg msg=MSG_CLOSE_WINDOW
hzw_handle_win_msg msg=MSG_APP_CANCEL
```
