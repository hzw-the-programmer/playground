```
#define HZW_WIN_ID
#define HZW_TIME_LEN 50
#define HZW_MAX_SMS_ID 10

typedef enum {
	HZW_ACTION_SET_INDEX,
	HZW_ACTION_PUSH_ID,
	HZW_ACTION_SHIFT_ID,
	HZW_ACTION_SET_READ_REQ_SENT,
	HZW_ACTION_SET_NAME,
	HZW_ACTION_SET_TIME,
	HZW_ACTION_SET_CONTENT,
	HZW_ACTION_SET_NAME_TIME_CONTENT,
} hzwActionCode;

typedef struct {
	hzwActionCode code;
	void *payload;
} hzwAction;

typedef struct {
	int32 recordId;
	uint8 storage;
	uint8 dualSys;	
} hzwSmsId;

typedef struct {
	int index;
	hzwSmsId ids[HZW_MAX_SMS_ID];
	BOOLEAN readReqSent;
	wchar name[MMISMS_PBNAME_MAX_LEN + 2];
	wchar time[HZW_TIME_LEN];
	wchar content[MMISMS_R8_MESSAGE_LEN + 1];
} hzwState;

typedef BOOLEAN hzwReducer(hzwAction *action, hzwState *state);

extern MMISMS_GLOBAL_T g_mmisms_global;

static char* hzwActionCodeStr(hzwActionCode code) {
	switch (code) {
		case HZW_ACTION_SET_INDEX:
			return "HZW_ACTION_SET_INDEX";
		case HZW_ACTION_PUSH_ID:
			return "HZW_ACTION_PUSH_ID";
		case HZW_ACTION_SHIFT_ID:
			return "HZW_ACTION_SHIFT_ID";
		case HZW_ACTION_SET_READ_REQ_SENT:
			return "HZW_ACTION_SET_READ_REQ_SENT";
		case HZW_ACTION_SET_NAME:
			return "HZW_ACTION_SET_NAME";
		case HZW_ACTION_SET_TIME:
			return "HZW_ACTION_SET_TIME";
		case HZW_ACTION_SET_CONTENT:
			return "HZW_ACTION_SET_CONTENT";
		case HZW_ACTION_SET_NAME_TIME_CONTENT:
			return "HZW_ACTION_SET_NAME_TIME_CONTENT";
		default:
			return "DEFAULT";
	}
}

static hzwState gState = {0};

static int hzwIndex() {
	return gState.index;
}

static hzwSmsId* hzwIds() {
	return gState.ids;
}

static BOOLEAN hzwReadReqSent() {
	return gState.readReqSent;
}

static wchar* hzwName() {
	return gState.name;
}

static wchar* hzwTime() {
	return gState.time;
}

static wchar* hzwContent() {
	return gState.content;
}

static BOOLEAN hzwSetIndex(hzwAction *action, hzwState *state) {
	int oldIndex = state->index;
	int newIndex = (int)action->payload;

	SCI_TRACE_LOW("hzwSetIndex: oldIndex=%d, newIndex=%d", oldIndex, newIndex);

	state->index = newIndex;

	return state->index != oldIndex;
}

static BOOLEAN hzwPushId(hzwAction *action, hzwState *state) {
	hzwSmsId *id = (hzwSmsId*)action->payload;
	int i = 0;
	int32 recordId = 0;
	uint8 storage = 0, dualSys = 0;

	for (i = 0; i < HZW_MAX_SMS_ID; i++) {
		int32 *recordIdP = &state->ids[i].recordId;
		uint8 *storageP = &state->ids[i].storage;
		uint8 *dualSysP = &state->ids[i].dualSys;

		if (!*recordIdP && !*storageP && !*dualSysP) {
			*recordIdP = recordId = id->recordId;
			*storageP = storage = id->storage;
			*dualSysP = dualSys = id->dualSys;
			break;
		}
	}

	SCI_TRACE_LOW("hzwPushId: i=%d, recordId=%d, storage=%d, dualSys=%d",
		i, recordId, storage, dualSys);

	return i != HZW_MAX_SMS_ID;
}

static BOOLEAN hzwShiftId(hzwAction *action, hzwState *state) {
	SCI_MEMCPY(state->ids, state->ids + 1, sizeof(hzwSmsId) * (HZW_MAX_SMS_ID - 1));
	state->ids[HZW_MAX_SMS_ID - 1].recordId = 0;
	state->ids[HZW_MAX_SMS_ID - 1].storage = 0;
	state->ids[HZW_MAX_SMS_ID - 1].dualSys = 0;

	return TRUE;
}

static char hzwHexChar(uint8 i) {
	if (i < 10) return '0' + i;
	return 'a' + (i - 10);
}

static char* hzwHexWStr(wchar *buf) {
	static char hex[40 + 3 + 1] = {0};
	int i = 0;

	if (!buf) {
		hex[0] = 0;
		return hex;
	}

	while (buf[i] && (i * 4 + 3) < 40) {
		hex[i * 4] = hzwHexChar((buf[i] & 0xf000) >> 12);
		hex[i * 4 + 1] = hzwHexChar((buf[i] & 0x0f00) >> 8);
		hex[i * 4 + 2] = hzwHexChar((buf[i] & 0xf0) >> 4);
		hex[i * 4 + 3] = hzwHexChar(buf[i] & 0x0f);
		i++;
	}

	if (!buf[i]) {
		hex[i * 4] = 0;
		return hex;
	}

	for (i *= 4; i < sizeof(hex) - 1; i++) {
		hex[i] = '.';
	}
	hex[i] = 0;

	return hex;
}

static BOOLEAN hzwSetName(wchar *name) {
	SCI_TRACE_LOW("hzwSetName: %s", hzwHexWStr(name));

	if (!MMIAPICOM_Wstrcmp(gState.name, name)) {
		return FALSE;
	}

	if (name) {
		MMIAPICOM_Wstrcpy(gState.name, name);
	} else {
		gState.name[0] = 0;
	}

	return TRUE;
}

static BOOLEAN hzwSetTime(wchar *time) {
	SCI_TRACE_LOW("hzwSetTime: %s", hzwHexWStr(time));

	if (!MMIAPICOM_Wstrcmp(gState.time, time)) {
		return FALSE;
	}

	if (time) {
		MMIAPICOM_Wstrcpy(gState.time, time);
	} else {
		gState.time[0] = 0;
	}

	return TRUE;
}

static BOOLEAN hzwSetContent(wchar *content) {
	SCI_TRACE_LOW("hzwSetContent: %s", hzwHexWStr(content));

	if (!MMIAPICOM_Wstrcmp(gState.content, content)) {
		return FALSE;
	}

	if (content) {
		MMIAPICOM_Wstrcpy(gState.content, content);
	} else {
		gState.content[0] = 0;
	}

	return TRUE;
}

typedef struct {
	wchar *name;
	wchar *time;
	wchar *content;
} hzwNameTimeContent;

static BOOLEAN hzwSetNameTimeContent(hzwNameTimeContent *nameTimeContent) {
	BOOLEAN changed = FALSE;

	changed = hzwSetName(nameTimeContent->name) || changed;
	changed = hzwSetTime(nameTimeContent->time) || changed;
	changed = hzwSetContent(nameTimeContent->content) || changed;

	return changed;
}

static BOOLEAN hzwReducer1(hzwAction *action, hzwState *state) {
	switch (action->code) {
		case HZW_ACTION_SET_INDEX:
			return hzwSetIndex(action, state);
		case HZW_ACTION_PUSH_ID:
			return hzwPushId(action, state);
		case HZW_ACTION_SHIFT_ID:
			return hzwShiftId(action, state);
		case HZW_ACTION_SET_READ_REQ_SENT:
			state->readReqSent = (BOOLEAN)action->payload;
			return TRUE;
		case HZW_ACTION_SET_NAME: {
			wchar *name = (wchar*)action->payload;
			return hzwSetName(name);
		}
		case HZW_ACTION_SET_TIME: {
			wchar *time = (wchar*)action->payload;
			return hzwSetTime(time);
		}
		case HZW_ACTION_SET_CONTENT: {
			wchar *content = (wchar*)action->payload;
			return hzwSetContent(content);
		}
		case HZW_ACTION_SET_NAME_TIME_CONTENT: {
			hzwNameTimeContent *nameTimeContent = (hzwNameTimeContent*)action->payload;
			return hzwSetNameTimeContent(nameTimeContent);
		}
		default:
			return FALSE;
	}
}

static hzwReducer* hzwReducers[] = {
	hzwReducer1,
};

#define HZW_MAX_REDUCERS (sizeof(hzwReducers)/sizeof(hzwReducers[0]))

static void hzwDispatch(hzwAction *action, hzwState *state) {
	int i = 0;
	BOOLEAN changed = FALSE;

	for (i = 0; i < HZW_MAX_REDUCERS; i++) {
		if (hzwReducers[i](action, state)) {
			changed = TRUE;
		}
	}

	SCI_TRACE_LOW("hzwDispatch: code=%s, changed=%d", hzwActionCodeStr(action->code), changed);

	if (!changed) {
		return;
	}

	if (!MMK_IsFocusWin(HZW_WIN_ID)) {
		return;
	}
	MMK_PostMsg(HZW_WIN_ID, MSG_FULL_PAINT, NULL, 0);
}

void HzwDispatch(hzwActionCode code, void *payload) {
	hzwAction action = {0};

	action.code = code;
	action.payload = payload;

	hzwDispatch(&action, &gState);
}

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

	text.wstr_ptr = hzwName();
	text.wstr_len = MMIAPICOM_Wstrlen(text.wstr_ptr);

	GUISTR_DrawTextToLCDInRect(&dev,
		&rect,
		&rect,
		&text,
		&style,
		state,
		GUISTR_TEXT_DIR_AUTO);

	text.wstr_ptr = hzwTime();
	text.wstr_len = MMIAPICOM_Wstrlen(text.wstr_ptr);
	rect.top += 30;

	GUISTR_DrawTextToLCDInRect(&dev,
		&rect,
		&rect,
		&text,
		&style,
		state,
		GUISTR_TEXT_DIR_AUTO);

	text.wstr_ptr = hzwContent();
	text.wstr_len = MMIAPICOM_Wstrlen(text.wstr_ptr);
	rect.top += 30;
	state = 0;

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

static void hzwReadSmsByOrderId(MMISMS_BOX_TYPE_E boxType, MMISMS_ORDER_ID_T orderId) {
	boxType = MMISMS_GetSmsStateBoxtype(orderId);
	// Inbox: MMISMS_BOX_MT
	// Outbox: MMISMS_BOX_SENDFAIL
	// Drafts: MMISMS_BOX_NOSEND
	// Sentbox: MMISMS_BOX_SENDSUCC
	// Security inbox: MMISMS_BOX_SECURITY

	// Inbox: mo_mt_type
	// MMISMS_MT_SR_HAVE_READ
	// MMISMS_MT_SR_TO_BE_READ
	// MMISMS_MT_HAVE_READ
	// MMISMS_MT_TO_BE_READ
	// MMISMS_MT_NOT_DOWNLOAD
	// MMISMS_MT_NOT_NOTIFY
	// MMISMS_MT_NEED_NOT_DOWNLOAD

	// Outbox: mo_mt_type
	// MMISMS_MO_SEND_FAIL

	// Drafts: mo_mt_type
	// MMISMS_MO_DRAFT

	// Sentbox: mo_mt_type
	// MMISMS_MO_SEND_SUCC

	// MMISMS_HandleUserReadSMSCnf

	MMISMS_ClearOperInfo();
	MMISMS_ReadyReadSms();
	MMISMS_SetSMSUpdateInfo(orderId);
	MMISMS_SetGlobalOperationOrderId(orderId);
	MMISMS_SetCurType(boxType);

	if (!MMISMS_AppReadSms(orderId)) {
		SCI_TRACE_LOW("hzwReadSmsByOrderId: failed");
		return;
	}

	HzwDispatch(HZW_ACTION_SET_READ_REQ_SENT, TRUE);
}

static void hzwReadSmsByIndex() {
	MMISMS_ORDER_ID_T orderId = NULL;
	MMISMS_BOX_TYPE_E boxType = MMISMS_BOX_MT;
	int index = hzwIndex();
	hzwAction action = {0};
	hzwNameTimeContent nameTimeContent = {0};

	HzwDispatch(HZW_ACTION_SET_NAME_TIME_CONTENT, (void*)&nameTimeContent);

	orderId = MMISMS_GetLinkedArrayPtrIdInList(boxType, index);
	if (!orderId) {
		SCI_TRACE_LOW("hzwReadSmsByIndex: orderId is NULL");
		HzwDispatch(HZW_ACTION_SET_INDEX, (void*)0);
		return;
	}

	HzwDispatch(HZW_ACTION_SET_INDEX, (void*)(index + 1));
	hzwReadSmsByOrderId(boxType, orderId);
}

static void hzwReadSmsById() {
	MMISMS_ORDER_ID_T orderId = NULL;
	MMISMS_BOX_TYPE_E boxType = MMISMS_BOX_MT;
	MMISMS_MSG_TYPE_E msgType = MMISMS_TYPE_SMS;
	hzwSmsId *ids = hzwIds();
	int32 recordId = ids[0].recordId;
	uint8 storage = ids[0].storage;
	uint8 dualSys = ids[0].dualSys;
	hzwNameTimeContent nameTimeContent = {0};

	SCI_TRACE_LOW("hzwReadSmsById: recordId=%d, storage=%d, dualSys=%d",
					recordId, storage, dualSys);
					
	HzwDispatch(HZW_ACTION_SET_NAME_TIME_CONTENT, (void*)&nameTimeContent);

	if (!recordId && !storage && !dualSys) {
		return;
	}

	if (!MMISMS_GetOrderIdByStorage(
			&orderId,
			msgType,
			dualSys, // 0为卡1，1为卡2
			storage, // 1为手机；2为sim卡，dualSys表明是哪张卡
			recordId)) { // 卡1从0开始，递增；卡2从199开始，递减？
		SCI_TRACE_LOW("hzwReadSmsById: orderId is NULL");
		return;
	}

	HzwDispatch(HZW_ACTION_SHIFT_ID, NULL);
	hzwReadSmsByOrderId(boxType, orderId);
}

static void hzwAddressToStr(MN_CALLED_NUMBER_T *address, MMI_STRING_T *str) {
	uint8 number[MMISMS_PBNAME_MAX_LEN + 2] = {0};
	uint8 numberLen = 0;
	uint8 numberTemp[MMISMS_PBNAME_MAX_LEN + 2] = {0};
	uint8 i = 0, j = 0;
	MMI_STRING_T name = {0};
	wchar nameTemp[MMISMS_PBNAME_MAX_LEN + 2] = {0};
	BOOLEAN find = FALSE;

	if (address->num_len < 1) {
		str->wstr_len = 0;
		return;
	}

	numberLen = MMIAPICOM_GenNetDispNumber((MN_NUMBER_TYPE_E)(address->number_type),
		(uint8)MIN((MMISMS_PBNUM_MAX_LEN >> 1), address->num_len),
		address->party_num,
		number,
		(uint8)(MMISMS_PBNUM_MAX_LEN + 2)
	);

	name.wstr_len = 0;
	name.wstr_ptr = nameTemp;

	find = MMISMS_GetNameByNumberFromPB(number, &name, MMISMS_PBNAME_MAX_LEN);

	if (!find || name.wstr_len == 0) {
		if (address->number_type == 0 && address->number_plan == 1) {
			while (number[i] == 0x30 && i < numberLen) {
				i++;
			}

			for (j = 0; j < numberLen + 1; j++, i++) {
				numberTemp[i] = number[i];
			}
		} else {
			for (i = 0; i < numberLen + 1; i++) {
				numberTemp[i] = number[i];
			}
		}

		name.wstr_len = MIN(MMISMS_PBNAME_MAX_LEN, strlen(numberTemp));
		MMI_STRNTOWSTR(name.wstr_ptr,
			name.wstr_len,
			numberTemp,
			name.wstr_len,
			name.wstr_len
		);
	}

	str->wstr_len  = name.wstr_len;

	MMI_WSTRNCPY(
		str->wstr_ptr,
		MMISMS_PBNAME_MAX_LEN,
		name.wstr_ptr,
		MMISMS_PBNAME_MAX_LEN,
		str->wstr_len
	);
}

static void hzwTimestampToStr(MN_SMS_TIME_STAMP_T *timestamp, MMI_STRING_T *str) {
	uint8 date[MMISET_DATE_STR_LEN] = {0};
	uint8 time[MMISET_TIME_STR_12HOURS_LEN + 1] = {0};
	uint8 dateTime[HZW_TIME_LEN] = {0};

	MMIAPISET_FormatDateStrByDateStyle(
		timestamp->year + MMISMS_MT_OFFSERT_YEAR,
		timestamp->month,
		timestamp->day,
		'-',
		date,
		MMISET_DATE_STR_LEN);

	MMIAPISET_FormatTimeStrByTime(
		timestamp->hour,
		timestamp->minute,
		time,
		MMISET_TIME_STR_12HOURS_LEN + 1);

	sprintf(dateTime, "%s %s", date, time);
	str->wstr_len = strlen(dateTime);

	MMI_STRNTOWSTR(
		str->wstr_ptr,
		str->wstr_len,
		dateTime,
		str->wstr_len,
		str->wstr_len
	);
}

static void hzwSmsToStr(MMISMS_READ_MSG_T *readMsg, MMI_STRING_T *str) {
	uint32 i = 0, offset = 0, dataOffset = 0;
	MMISMS_CONTENT_T *readContent = &readMsg->read_content;
	uint8 *data = readContent->data;
	uint16 dataLen = readContent->length;
	MN_SMS_ALPHABET_TYPE_E alphabet = readContent->alphabet;
	MMISMS_R8_LANGUAGE_E language = readMsg->language;
	uint8 splitCount = readContent->sms_num;
	BOOLEAN singleShift = readMsg->is_single_shift;
	BOOLEAN lockShift = readMsg->is_lock_shift;
	uint16 splitLen = 0;
	MN_SMS_ALPHABET_TYPE_E splitAlphabet = MN_SMS_DEFAULT_ALPHABET;
	wchar r8[MMISMS_R8_MESSAGE_LEN + 1] = {0};
	uint16 r8Len = 0;

	SCI_TRACE_LOW("hzwSmsToStr: dataLen=%d, alphabet=%d, splitCount=%d, "
		"language=%d, singleShift=%d, lockShift=%d",
		dataLen, alphabet, splitCount, language, singleShift, lockShift);

	if (MMISMS_IsR8Language(language)
		&& alphabet != MN_SMS_UCS2_ALPHABET
		&& alphabet != MN_SMS_8_BIT_ALPHBET) {
		r8Len = MMISMS_Default2NationalR8(
			data,
			r8,
			dataLen,
			language,
			lockShift,
			singleShift
		);
		MMI_WSTRNCPY(
			str->wstr_ptr,
			MMISMS_R8_MESSAGE_LEN,
			r8,
			r8Len,
			r8Len);
	}

	for (i = 0; i < MIN(splitCount, MMISMS_SPLIT_MAX_NUM); i++) {
		splitLen = readContent->split_sms_length[i];
		splitAlphabet = readContent->alphabet_ori[i];
		SCI_TRACE_LOW("hzwSmsToStr: i=%d, splitLen=%d, splitAlphabet=%d", i, splitLen, splitAlphabet);

		if (splitLen != 0) {
			if (splitAlphabet == MN_SMS_UCS2_ALPHABET) {
				GUI_UCS2L2UCS2B((uint8*)(str->wstr_ptr + offset),
					splitLen,
					data + dataOffset,
					splitLen);
				offset += splitLen / sizeof(wchar);
				dataOffset += splitLen;
			} else {
				uint16 len = splitLen;
				if (alphabet == MN_SMS_8_BIT_ALPHBET) {
					MMI_STRNTOWSTR(
						str->wstr_ptr + offset,
						len,
						data + dataOffset,
						len,
						len
					);
				} else {
					len = MMIAPICOM_Default2Wchar(
						data + dataOffset,
						str->wstr_ptr + offset,
						len
					);
				}
				SCI_TRACE_LOW("hzwSmsToStr: len=%d, splitLen=%d", len, splitLen);

				offset += len;
				dataOffset += splitLen;
				if((data[(i + 1) * MMISMS_DEFAULT_CHAR_LEN - 1] == 0x1b)
					&& (data[(i + 1) * MMISMS_DEFAULT_CHAR_LEN ] != 0x00)
					&& (data[(i + 1) * MMISMS_DEFAULT_CHAR_LEN ] < 0x80)) {
					dataOffset += 1;
				}
			}
		}
	}

	str->wstr_len = MMIAPICOM_Wstrlen(str->wstr_ptr);
}

//MMISMS_HandleUserReadSMSCnf
BOOLEAN HzwReadSmsRes(MMISMS_READ_MSG_T *readMsg) {
	MMI_STRING_T name = {0};
	wchar nameTemp[MMISMS_PBNAME_MAX_LEN + 2] = {0};
	MMI_STRING_T time = {0};
	wchar timeTemp[HZW_TIME_LEN] = {0};
	MMI_STRING_T content = {0};
	wchar contentTemp[MMISMS_R8_MESSAGE_LEN + 1] = {0};
	hzwNameTimeContent nameTimeContent = {0};

	if (!hzwReadReqSent()) {
		return FALSE;
	}
	HzwDispatch(HZW_ACTION_SET_READ_REQ_SENT, FALSE);

	if (!readMsg) {
		SCI_TRACE_LOW("HzwReadSmsRes: res failed");
		return TRUE;
	}

	name.wstr_len = 0;
	name.wstr_ptr = nameTemp;
	hzwAddressToStr(&readMsg->address, &name);
	name.wstr_ptr[name.wstr_len] = 0;

	time.wstr_len = 0;
	time.wstr_ptr = timeTemp;
	hzwTimestampToStr(&readMsg->time_stamp, &time);
	time.wstr_ptr[time.wstr_len] = 0;

#if 1
	MMISMS_GetSMSContent(&content);
#else
	content.wstr_len = 0;
	content.wstr_ptr = contentTemp;
	hzwSmsToStr(readMsg, &content);
	content.wstr_ptr[content.wstr_len] = 0;
#endif

	nameTimeContent.name = name.wstr_ptr;
	nameTimeContent.time = time.wstr_ptr;
	nameTimeContent.content = content.wstr_ptr;
	HzwDispatch(HZW_ACTION_SET_NAME_TIME_CONTENT, (void*)&nameTimeContent);

#if 1
	SCI_FREE(content.wstr_ptr);
#endif

	MMISMS_UpdateUnreadOrder();

	return TRUE;
}

void HzwNewSms(uint16 recordId, uint8 storage, uint8 dualSys) {
	int i = 0;
	hzwSmsId id = {0};

	SCI_TRACE_LOW("HzwNewSms: recordId=%d, storage=%d, dualSys=%d", recordId, storage, dualSys);

	id.recordId = recordId;
	id.storage = storage;
	id.dualSys = dualSys;
	HzwDispatch(HZW_ACTION_PUSH_ID, &id);
}

static BOOLEAN hzwSending = FALSE;
#define HZW_NUMBER
#define HZW_CONTENT L"abcdefghijklmnopqrstuvwxyz0123456789" \
	L"abcdefghijklmnopqrstuvwxyz0123456789" \
	L"abcdefghijklmnopqrstuvwxyz0123456789" \
	L"abcdefghijklmnopqrstuvwxyz0123456789" \
	L"abcdefghijklmnopqrstuvwxyz0123456789" \
	L"abcdefghijklmnopqrstuvwxyz0123456789" \
	L"abcdefghijklmnopqrstuvwxyz0123456789" \
	L"abcdefghijklmnopqrstuvwxyz0123456789" \
	L"abcdefghijklmnopqrstuvwxyz0123456789" \
	L"abcdefghijklmnopqrstuvwxyz0123456789" \
	L"abcdefghijklmnopqrstuvwxyz0123456789" \
	L"abcdefghijklmnopqrstuvwxyz0123456789" \
	L"abcdefghijklmnopqrstuvwxyz0123456789" \
	L"abcdefghijklmnopqrstuvwxyz0123456789" \
	L"abcdefghijklmnopqrstuvwxyz0123456789" \
	L"abcdefghijklmnopqrstuvwxyz0123456789"

static void hzwSendSms(uint8 *number, wchar *content) {
	uint16 contentLen = MMIAPICOM_Wstrlen(content);
	uint8 numberLen = strlen(number);
	MN_DUAL_SYS_E dualSys = MN_DUAL_SYS_1;

	if (MMIAPISMS_IsSendingSMS()) {
		return;
	}

	MMISMS_SetMessageContent(contentLen, content, FALSE, &g_mmisms_global.edit_content);
	MMISMS_SetAddressToMessage(number, numberLen, &g_mmisms_global.edit_content.dest_info.dest_list);
	MMISMS_SetCurSendDualSys(dualSys);
	MMISMS_SetDestDualSys(dualSys);
	MMISMS_ClearDestId();
	hzwSending = MMISMS_WintabSendSMS(FALSE, FALSE) == MMISMS_NO_ERR;

	SCI_TRACE_LOW("hzwSendSms: sending=%d", hzwSending);
}

BOOLEAN HzwSendSmsRsp(BOOLEAN success) {
	SCI_TRACE_LOW("HzwSendSmsRsp: sending=%d, success=%d", hzwSending, success);

	if (!hzwSending) {
		return FALSE;
	}

	MMISMS_ClearEditContent();
	MMISMS_ClearDestNum();

	hzwSending = FALSE;
	return TRUE;
}

static BOOLEAN hzwSendCb (BOOLEAN success, void *parm) {
	SCI_TRACE_LOW("hzwSendCb: success=%d", success);
}

// this method has bug when sending long sms
static void hzwSendSms1(uint8 *number, wchar *text) {
	MMI_STRING_T content = {0};
	MMISMS_SEND_DATA_T sendData = {0};
	BOOLEAN success = FALSE;

	content.wstr_ptr = text;
	content.wstr_len = MMIAPICOM_Wstrlen(text);

	sendData.dest_addr_ptr = number;
	sendData.dest_addr_len = strlen(number);
	sendData.sms_content_ptr = &content;
	sendData.dual_sys = MN_DUAL_SYS_1;
	sendData.is_need_packet = FALSE;
	sendData.dst_port = 0;
	sendData.src_port = 0;
	sendData.send_callback = hzwSendCb;

	success = MMIAPISMS_SendSmsForOther(&sendData);

	SCI_TRACE_LOW("hzwSendSms1: success=%d", success);
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
			hzwReadSmsByIndex();
			break;
		case MSG_KEYUP_2:
			hzwReadSmsById();
			break;
		case MSG_KEYUP_3:
			hzwSendSms(HZW_NUMBER, L"1" HZW_CONTENT L"E");
			break;
		case MSG_KEYUP_4:
			hzwSendSms1(HZW_NUMBER, L"2" HZW_CONTENT L"E");
			break;
		case MSG_KEYUP_5:
			hzwSendSms(HZW_NUMBER, L"3hello worldE");
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
}
```

```
PUBLIC void MMISMS_ShowSendResult(uint8 fail_count, uint8 *fail_id_ptr)
{
	...
	{
		extern BOOLEAN HzwSendSmsRsp(BOOLEAN success);
		if (HzwSendSmsRsp(fail_count == 0)) {
			return;
		}
	}
	...
}
```

```
PUBLIC void MMISMS_HandleUserReadSMSCnf(DPARAM param)
{
	...
#ifdef MMISMS_CHATMODE_ONLY_SUPPORT
        else if(MMISMS_READ_SMS == MMISMS_GetCurrentOperStatus())
        {
			{
				extern BOOLEAN HzwReadSmsRes(MMISMS_READ_MSG_T *readMsg);
				if (HzwReadSmsRes(sig_ptr)) {
					return;
				}
			}
		}
	...
}
```
