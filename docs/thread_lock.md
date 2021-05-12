```
MMI_HANDLE_T hzw_win_handle = 0;
BLOCK_ID hzw_tid = 0;
SCI_MUTEX_PTR hzw_lock = NULL;

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

MMI_RESULT_E hzw_win_func(MMI_WIN_ID_T win_id, MMI_MESSAGE_ID_E msg_id, DPARAM param) {
	switch (msg_id) {
	case MSG_FULL_PAINT:
		hzw_paint();
		break;
	case MSG_KEYUP_1:
		hzw_critical();
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

void hzw_entry(uint32 argc, void *argv) {
	hzw_critical();
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
