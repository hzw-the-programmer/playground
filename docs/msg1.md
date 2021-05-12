```
#define HZW_WIN_ID
#define HZW_MAX_TIMER 50

typedef struct {
	int id;
	uint32 tick;
	SCI_TIMER_PTR timer;
} HZW_TIMER;

MMI_HANDLE_T hzwWinHandle = 0;
uint8 hzwTimerId = 0;
HZW_TIMER hzwTimer1 = {0};
HZW_TIMER hzwTimer2 = {0};
HZW_TIMER hzwTimers[HZW_MAX_TIMER] = {0};
BLOCK_ID hzwTid = 0;

MMI_RESULT_E hzwProcessMsg(PWND app_ptr, uint16 msg_id, DPARAM param) {
	BLOCK_ID tid = SCI_IdentifyThread();
	char name[SCI_MAX_NAME_SIZE] = {0};
	BOOLEAN active = FALSE;
	uint32 remaining_time;
	uint32 reschedule_time;
	HZW_TIMER *timer = NULL;
	uint32 tick = SCI_GetTickCount();

	switch (msg_id) {
		case HZW_SIGNALCODE_TIMER:
			timer = (HZW_TIMER*)param;
			SCI_GetTimerInfo(timer->timer, name, &active, &remaining_time, &reschedule_time);
			SCI_TRACE_LOW("hzw tid=%u, id=%d, name=%s, active=%d, remaining_time=%d, reschedule_time=%d, period=%d",
				tid, timer->id, name, active, remaining_time, reschedule_time, tick - timer->tick);
			break;
	}
}

MMI_APPLICATION_T g_hzw_app = {hzwProcessMsg, 0, 0};

void hzwPaint(uint32 *num) {
	GUI_LCD_DEV_INFO dev_info = {0};
	GUI_RECT_T box = {0, 0, 239, 319};
	MMI_STRING_T str = {0};
	char txt[128] = {0};
	wchar wtxt[256] = {0};
	int i = 0;
	GUISTR_STYLE_T style = {0};
	GUISTR_STATE_T state = 0;

	LCD_FillRect(&dev_info, box, MMI_RED_COLOR);

	sprintf(txt, "num: %d", (*num)++);
	while (txt[i]) {
		wtxt[i] = txt[i];
		i++;
	}
	str.wstr_ptr = wtxt;
	str.wstr_len = i;

	style.font = SONG_FONT_24;
	style.font_color = MMI_WHITE_COLOR;
	
	GUISTR_DrawTextToLCDInRect(&dev_info,
		&box,
		&box,
		&str,
		&style,
		state,
		0);
}

void hzwTimerCb(uint32 input) {
	BLOCK_ID tid = SCI_IdentifyThread();
	char name[SCI_MAX_NAME_SIZE] = {0};
	BOOLEAN active = FALSE;
	uint32 remaining_time;
	uint32 reschedule_time;
	HZW_TIMER *timer = (HZW_TIMER*)input;
	HZW_TIMER *timer1 = NULL;
	uint32 tick = SCI_GetTickCount();
	MmiSignalS *sendSignal = NULL;

	SCI_GetTimerInfo(timer->timer, name, &active, &remaining_time, &reschedule_time);
	active = SCI_IsTimerActive(timer->timer);

	SCI_TRACE_LOW("hzw tid=%u, id=%d, name=%s, active=%d, remaining_time=%d, reschedule_time=%d, period=%d",
		tid, timer->id, name, active, remaining_time, reschedule_time, tick - timer->tick);

	timer->tick = tick;

	MmiCreateSignal(HZW_SIGNALCODE_TIMER, sizeof(HZW_TIMER), &sendSignal);
	timer1 = (HZW_TIMER*)&sendSignal->sig;
	timer1->id = timer->id;
	timer1->tick =  timer->tick;
	timer1->timer = timer->timer;
	sendSignal->send = P_APP;

	MmiSendSignal(TASK_FL_ID, sendSignal);
}

void hzwCreatePeriodTimer(HZW_TIMER *timer, int id, BOOLEAN active) {
	timer->id = id;
	timer->tick = SCI_GetTickCount();
	timer->timer = SCI_CreatePeriodTimer("hzw period timer",
										hzwTimerCb,
										(uint32)timer,
										1000,
										active);
	if (!timer->timer) {
		SCI_TRACE_LOW("hzw: create timer %d failed", id);
	}
}

void hzwCreatePeriodTimers() {
	int i = 0;
	int period = 3000;

	for (i = 0; i < HZW_MAX_TIMER; i++) {
		hzwCreatePeriodTimer(&hzwTimers[i], i, TRUE);
	}
}

void hzwDeletePeriodTimers() {
	int i = 0;

	for (i = 0; i < HZW_MAX_TIMER; i++) {
		if (hzwTimers[i].timer) {
			if (SCI_DeleteTimer(hzwTimers[i].timer) != SCI_SUCCESS) {
				SCI_TRACE_LOW("hzw delete timer %d failed", i);
			}
			hzwTimers[i].timer = NULL;
		}
	}
}

void hzwOnceTimerCb(uint8 timerId, uint32 param) {
	SCI_TRACE_LOW("timerId=%d", timerId);
	MMK_CreateTimerCallback(1000, hzwOnceTimerCb, NULL, FALSE);
}

MMI_RESULT_E hzwWinFunc(MMI_WIN_ID_T win_id, MMI_MESSAGE_ID_E msg_id, DPARAM param) {
	static uint32 num = 0;
	BLOCK_ID tid = SCI_IdentifyThread();
	HZW_TIMER *timer = NULL;

	switch (msg_id) {
		case MSG_FULL_PAINT:
			hzwPaint(&num);
			break;

		case MSG_KEYUP_1: {
				if (!hzwTimerId) {
					hzwTimerId = MMK_CreateTimer(1000, 1);
				} else {
					MMK_StopTimer(hzwTimerId);
					hzwTimerId = 0;
				}	
				break;
			}

		case MSG_KEYUP_2: {
				if (!hzwTimer1.timer) {
					hzwCreatePeriodTimer(&hzwTimer1, -1, FALSE);
				} else if (!hzwTimer2.timer) {
					hzwCreatePeriodTimer(&hzwTimer2, -2, FALSE);
				}
				break;
			}

		case MSG_KEYUP_3: {
				BOOL isActive1 = hzwTimer1.timer && SCI_IsTimerActive(hzwTimer1.timer);
				BOOL isActive2 = hzwTimer2.timer && SCI_IsTimerActive(hzwTimer2.timer);

				if (!isActive1) {
					SCI_ActiveTimer(hzwTimer1.timer);
					SCI_TRACE_LOW("hzw period timer %d active", hzwTimer1.id);
				} else if (!isActive2) {
					SCI_ActiveTimer(hzwTimer2.timer);
					SCI_TRACE_LOW("hzw period timer %d active", hzwTimer2.id);
				}
				break;
			}

		case MSG_KEYUP_4: {
				BOOL isActive1 = hzwTimer1.timer && SCI_IsTimerActive(hzwTimer1.timer);
				BOOL isActive2 = hzwTimer2.timer && SCI_IsTimerActive(hzwTimer2.timer);

				{
					char name[SCI_MAX_NAME_SIZE] = {0};
					BOOLEAN active = FALSE;
					uint32 remaining_time;
					uint32 reschedule_time;
					SCI_GetTimerInfo(hzwTimer1.timer, name, &active, &remaining_time, &reschedule_time);
				}

				if (isActive1) {
					SCI_DeactiveTimer(hzwTimer1.timer);
					SCI_TRACE_LOW("hzw period timer %d deactive", hzwTimer1.id);
				} else if (isActive2) {
					SCI_DeactiveTimer(hzwTimer2.timer);
					SCI_TRACE_LOW("hzw period timer %d deactive", hzwTimer2.id);
				}
				break;
			}

		case MSG_KEYUP_5: {
				hzwCreatePeriodTimers();
				break;
			}

		case MSG_KEYUP_6: {
				hzwDeletePeriodTimers();
				break;
			}

		case MSG_KEYUP_7: {
				MMK_CreateTimerCallback(1000, hzwOnceTimerCb, NULL, FALSE);
				break;
			}

		case MSG_TIMER:
			hzwPaint(&num);
			break;

		case 23:
			timer = (HZW_TIMER*)param;
			SCI_TRACE_LOW("hzwWinFunc msg=%d tid=%u, ptr=0x%x", 23, tid, timer);
			break;

		case 24:
			timer = (HZW_TIMER*)param;
			SCI_TRACE_LOW("hzwWinFunc msg=%d tid=%u, ptr=0x%x", 24, tid, timer);
			break;
	}
}

WINDOW_TABLE(hzwWinTable) = {
	WIN_ID(HZW_WIN_ID),
	WIN_FUNC(hzwWinFunc),
	WIN_HIDE_STATUS,
	END_WIN,
};

void hzwEntry(uint32 argc, void *argv) {
	HZW_TIMER timer = {1, 23, NULL};
	BLOCK_ID tid = SCI_IdentifyThread();

	MMK_SendMsg(hzwWinHandle, 23, &timer);
	SCI_TRACE_LOW("hzw MMK_SendMsg msg=%d tid=%u, ptr=0x%x", 23, tid, &timer);

	SCI_Sleep(3000);

	MMK_PostMsg(hzwWinHandle, 24, &timer, sizeof(HZW_TIMER));
	SCI_TRACE_LOW("hzw MMK_PostMsg msg=%d tid=%u, ptr=0x%x", 24, tid, &timer);
}

void HzwCreateWindow() {
	hzwWinHandle = MMK_CreateWin(hzwWinTable, NULL);
	hzwTid = SCI_CreateThread("THZW",
		"QHZW",
		hzwEntry,
		0,
		NULL,
		2 * 1024,
		24,
		20, //SCI_PRIORITY_NORMAL,
		SCI_PREEMPT,
		SCI_AUTO_START);
}
```
