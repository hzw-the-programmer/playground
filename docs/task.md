## file
MS_Code\MS_MMI\source\mmi_app\app\tts\iflytek\c\tts_iflytek_api.c
## func
TTS_Task

## file
MS_Code\MS_MMI\source\mmi_kernel_sys\source\c\mmk_window.c
## func
MMK_UpdateScreen

## file
MS_Code\MS_MMI\source\mmi_kernel\source\c\mmk_msg.c
## func
MMK_PostMsg
MMK_GetMSGQueue

MMK_SendMsg

## example

```
#define HZW_TASK_STACK_SIZE 4096
#define HZW_TASK_QUEUE_NUM 8
#define HZW_TASK_PRIORITY 29

typedef enum {
	HZW_SIG_CODE_ADD,
} HZW_SIG_CODE_E;

typedef struct {
	SIGNAL_VARS
	int32 data;
} HZW_SIG_T;

typedef struct {
	int32 data;
} HZW_RESULT_T;

BLOCK_ID hzw_tid;
MMI_HANDLE_T hzw_wid;

extern MMI_RESULT_E HZW_HandleWindowMsg(MMI_WIN_ID_T win_id, MMI_MESSAGE_ID_E msg_id, DPARAM param);

WINDOW_TABLE(hzw_window) = {
	WIN_ID(HZW_WIN_ID),
	WIN_FUNC((uint32)HZW_HandleWindowMsg),
	WIN_HIDE_STATUS,
	END_WIN,
};

void HZW_SendSignalToTask(HZW_SIG_CODE_E sig_code, BLOCK_ID tid, int32 data) {
	HZW_SIG_T *sig = NULL;

	sig = (HZW_SIG_T*)SCI_ALLOC(sizeof(HZW_SIG_T));
	if (sig == NULL) {
		return;
	}

	sig->SignalCode = (uint16)sig_code;
	sig->SignalSize = sizeof(xSignalHeaderRec);
	sig->Sender = NULL;
	sig->data = data;

	if(SCI_SUCCESS != SCI_SendSignal((xSignalHeader)sig, tid)) {
	}
}

void HZW_Task(uint32 argc, void * argv) {
	static int32 data = 0;
	HZW_SIG_T *sig = NULL;
	BLOCK_ID tid = SCI_IdentifyThread();
	HZW_RESULT_T ret = {0};

	while (1) {
		sig = (HZW_SIG_T*)SCI_GetSignal(tid);
		if (sig != NULL) {
			switch (sig->SignalCode) {
				case HZW_SIG_CODE_ADD:
					data += sig->data;
					ret.data = data;
					MMK_PostMsg(hzw_wid, MSG_FULL_PAINT, &ret, sizeof(HZW_RESULT_T));
					break;
			}
			SCI_FREE(sig);
		}
	}
}

uint16 asc_to_ucs2(wchar *out, char *in) {
	uint16 len = 0;

	while(*in) {
		*out++ = *in++;
		len++;
	}
	*out++ = 0;

	return len;
}

MMI_RESULT_E HZW_HandleWindowMsg(MMI_WIN_ID_T win_id, MMI_MESSAGE_ID_E msg_id, DPARAM param) {
	GUI_LCD_DEV_INFO layer = {GUI_MAIN_LCD_ID, GUI_BLOCK_MAIN};
	GUI_RECT_T rect = {0, 0, 239, 359};
	BLOCK_ID tid = SCI_IdentifyThread();
	char c[256] = {0};
	wchar wc[256] = {0};
	int32 data = 0;
	MMI_STRING_T str;
	GUISTR_STYLE_T style = {0};
	GUISTR_STATE_T state = 0;

	switch (msg_id) {
		case MSG_FULL_PAINT:
			if (param != NULL) {
				data = ((HZW_RESULT_T*)param)->data;
			}
			sprintf(c, "tid=%d, data=%d", tid, data);
			asc_to_ucs2(wc, c);

			str.wstr_ptr = wc;
			str.wstr_len = MMIAPICOM_Wstrlen(wc);
			style.font = SONG_FONT_16;
			style.font_color = MMI_BLACK_COLOR;
			LCD_FillRect(&layer, rect, MMI_WHITE_COLOR);

			GUISTR_DrawTextToLCDInRect(&layer,
				&rect,
				&rect,
				&str,
				&style,
				state,
				GUISTR_TEXT_DIR_AUTO);
			break;
		case MSG_APP_OK:
			HZW_SendSignalToTask(HZW_SIG_CODE_ADD, hzw_tid, 11);
			break;
	}
}

void HZW_MAIN() {
	hzw_wid = MMK_CreateWin(hzw_window, NULL);
	hzw_tid = SCI_CreateThread(
		"T_HZW",
		"Q_HZW",
		HZW_Task,
		0,
		0,
		HZW_TASK_STACK_SIZE,
		HZW_TASK_QUEUE_NUM,
		HZW_TASK_PRIORITY,
		SCI_PREEMPT,
		SCI_AUTO_START
		);
}
```

## MMK_PostMsg为何能起作用？

因为在MS_Code\MS_MMI\source\mmi_app\kernel\c\mmimain.c中的APP_Task函数有：
```
void APP_Task(uint32 argc, void * argv)
{
	...
	MMI_MESSAGE_PTR_T mmi_msg;
	...
	while (1)
	{
		...
		if (MMK_GetMSGQueue(&mmi_msg))
		{
			...
			MMK_DispatchMSGQueue(mmi_msg);
			...
			MMK_FreeMSG(mmi_msg);
		}
		else
		{
			...
			MmiReceiveSignal(P_APP, &receiveSignal);
			...
		}
		...
	}
	...
}
```
