```
#define HZW_WIN_ID
#define HZW_TIME_LEN 50
#define HZW_MAX_SMS 10

typedef struct {
	int32 recordId;
	uint8 storage;
	uint8 dualSys;	
} hzwSmsId;

static MMI_HANDLE_T hzwWinHandle;
static BOOLEAN hzwReqSent = FALSE;
static uint32 hzwIndex = 0;
static wchar hzwName[MMISMS_PBNAME_MAX_LEN + 2];
static wchar hzwTime[HZW_TIME_LEN];
static wchar hzwSms[MMISMS_R8_MESSAGE_LEN + 1];
static hzwSmsId hzwSmsIds[HZW_MAX_SMS] = {0};

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

	text.wstr_ptr = hzwName;
	text.wstr_len = MMIAPICOM_Wstrlen(text.wstr_ptr);

	GUISTR_DrawTextToLCDInRect(&dev,
		&rect,
		&rect,
		&text,
		&style,
		state,
		GUISTR_TEXT_DIR_AUTO);

	text.wstr_ptr = hzwTime;
	text.wstr_len = MMIAPICOM_Wstrlen(text.wstr_ptr);
	rect.top += 30;

	GUISTR_DrawTextToLCDInRect(&dev,
		&rect,
		&rect,
		&text,
		&style,
		state,
		GUISTR_TEXT_DIR_AUTO);

	text.wstr_ptr = hzwSms;
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
	MMK_CloseWin(hzwWinHandle);
}

static void hzwClear() {
	hzwName[0] = 0;
	hzwTime[0] = 0;
	hzwSms[0] = 0;
}

static void hzwReadSmsByOrderId(MMISMS_BOX_TYPE_E boxType, MMISMS_ORDER_ID_T orderId) {
	MMISMS_ClearOperInfo();
	MMISMS_ReadyReadSms();
	MMISMS_SetSMSUpdateInfo(orderId);
	MMISMS_SetGlobalOperationOrderId(orderId);
	MMISMS_SetCurType(boxType);

	if (!MMISMS_AppReadSms(orderId)) {
		SCI_TRACE_LOW("hzwReadSmsReqByOrderId: send req failed");
		return;
	}

	hzwReqSent = TRUE;
}

static void hzwReadSmsByIndex() {
	MMISMS_BOX_TYPE_E boxType = MMISMS_BOX_MT;
	MMISMS_ORDER_ID_T orderId = NULL;

	orderId = MMISMS_GetLinkedArrayPtrIdInList(boxType, hzwIndex++);
	if (!orderId) {
		SCI_TRACE_LOW("hzwReadSmsReqByIndex: orderId is NULL");
		hzwIndex = 0;
		hzwClear();
		MMK_PostMsg(HZW_WIN_ID, MSG_FULL_PAINT, NULL, 0);
		return;
	}

	hzwReadSmsByOrderId(boxType, orderId);
}

static void hzwReadSmsByIds() {
	MMISMS_ORDER_ID_T orderId = NULL;
	MMISMS_BOX_TYPE_E boxType = MMISMS_BOX_MT;
	MMISMS_MSG_TYPE_E msgType = MMISMS_TYPE_SMS;
	int32 recordId = hzwSmsIds[0].recordId;
	uint8 storage = hzwSmsIds[0].storage;
	uint8 dualSys = hzwSmsIds[0].dualSys;

	SCI_TRACE_LOW("hzwReadSmsByIds: recordId=%d, storage=%d, dualSys=%d",
					recordId, storage, dualSys);

	if (!recordId && !storage && !dualSys) {
		return;
	}

	if (!MMISMS_GetOrderIdByStorage(
			&orderId,
			msgType,
			dualSys,
			storage,
			recordId)) {
		SCI_TRACE_LOW("hzwReadSmsByIds: orderId is NULL");
		return;
	}

	SCI_MEMCPY(hzwSmsIds, hzwSmsIds + 1, sizeof(hzwSmsIds[0]) * (HZW_MAX_SMS - 1));
	hzwSmsIds[HZW_MAX_SMS - 1].recordId = 0;
	hzwSmsIds[HZW_MAX_SMS - 1].storage = 0;
	hzwSmsIds[HZW_MAX_SMS - 1].dualSys = 0;

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
void HzwReadSmsRes(MMISMS_READ_MSG_T *readMsg) {
	MMI_STRING_T name = {0};
	wchar nameTemp[MMISMS_PBNAME_MAX_LEN + 2] = {0};
	MMI_STRING_T time = {0};
	wchar timeTemp[HZW_TIME_LEN] = {0};
	MMI_STRING_T sms = {0};
	wchar smsTemp[MMISMS_R8_MESSAGE_LEN + 1] = {0};

	if (!hzwReqSent) {
		return;
	}
	hzwReqSent = FALSE;

	if (!readMsg) {
		SCI_TRACE_LOW("HzwReadSmsRes: res failed");
		return;
	}

	name.wstr_len = 0;
	name.wstr_ptr = nameTemp;
	hzwAddressToStr(&readMsg->address, &name);

	time.wstr_len = 0;
	time.wstr_ptr = timeTemp;
	hzwTimestampToStr(&readMsg->time_stamp, &time);

	sms.wstr_len = 0;
	sms.wstr_ptr = smsTemp;
	hzwSmsToStr(readMsg, &sms);

	MMIAPICOM_Wstrncpy(hzwName, name.wstr_ptr, name.wstr_len);
	hzwName[name.wstr_len] = 0;
	MMIAPICOM_Wstrncpy(hzwTime, time.wstr_ptr, time.wstr_len);
	hzwTime[time.wstr_len] = 0;
	MMIAPICOM_Wstrncpy(hzwSms, sms.wstr_ptr, sms.wstr_len);
	hzwSms[sms.wstr_len] = 0;

	MMK_PostMsg(HZW_WIN_ID, MSG_FULL_PAINT, NULL, 0);
}

void HzwNewSms(uint16 recordId, uint8 storage, uint8 dualSys) {
	int i = 0;

	SCI_TRACE_LOW("HzwNewSms: recordId=%d, storage=%d, dualSys=%d", recordId, storage, dualSys);

	for (i = 0; i < HZW_MAX_SMS; i++) {
		if (!hzwSmsIds[i].recordId && !hzwSmsIds[i].storage && !hzwSmsIds[i].dualSys) {
			SCI_TRACE_LOW("HzwNewSms: find at %d", i);
			hzwSmsIds[i].recordId = recordId;
			hzwSmsIds[i].storage = storage;
			hzwSmsIds[i].dualSys = dualSys;
			break;
		}
	}
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
			hzwReadSmsByIds();
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
	hzwClear();
	hzwWinHandle = MMK_CreateWin(HZW_WIN_TAB, NULL);
	SCI_TRACE_LOW("HzwLaunch: after MMK_CreateWin");
}
```

将HzwReadSmsRes添加到如下位置
```
PUBLIC void MMISMS_HandleUserReadSMSCnf(DPARAM param)
{
    ...
    if (g_mmisms_global.operation.cur_order_index < MMISMS_SPLIT_MAX_NUM - 1)
    {
        g_mmisms_global.operation.cur_order_index ++;
        // 获取短信的下一个部分
        next_order_id = MMISMS_GetCurOperationOrderId();  //必须用MMISMS_GetCurOperationOrderId
    }
    ...
    // 如果存在短信的下一个部分
    if ( next_order_id != PNULL )
    {
        if ( is_exist_read_err)
        {
           ...
        }
        ...
        else
        {
            // 发起请求读取下一部分
            MMISMS_AppReadSms( next_order_id );
        }
    }
    ...
    if (is_exist_read_err && IsExistContent())
    {
        // 读取失败
        HzwReadSmsRes(NULL);
    }
    ...
    else
    {
        // 读取成功，这里得到是完整短信
        HzwReadSmsRes(&g_mmisms_global.read_msg);
    }
}
```

日志
```
000000340039		34-39:HzwLaunch: after MMK_CreateWin
000000350009		35-9:hzwWinHandleMsg: MSG_OPEN_WINDOW
000000350013		35-13:hzwWinHandleMsg: MSG_PRE_FULL_PAINT
000000350016		35-16:hzwWinHandleMsg: MSG_FULL_PAINT
000000350019		35-19:hzwWinHandleMsg: MSG_END_FULL_PAINT
000000650013		65-13:hzwWinHandleMsg: MSG_LOSE_FOCUS
000001040041		104-41:hzwWinHandleMsg: MSG_GET_FOCUS
000001040048		104-48:hzwWinHandleMsg: MSG_PRE_FULL_PAINT
000001040051		104-51:hzwWinHandleMsg: MSG_FULL_PAINT
000001050002		105-2:hzwWinHandleMsg: MSG_END_FULL_PAINT
000001060005		106-5:hzwWinHandleMsg: KEY_CANCEL KEY_RELEASED
000001300059		130-59:hzwWinHandleMsg: KEY_1 KEY_PRESSED
000001310038		131-38:hzwWinHandleMsg: KEY_1 KEY_RELEASED
000001390005		139-5:hzwSmsToStr: dataLen=278, alphabet=2, splitCount=3, language=0, singleShift=0, lockShift=0
000001390006		139-6:hzwSmsToStr: i=0, splitLen=134, splitAlphabet=2
000001390007		139-7:hzwSmsToStr: i=1, splitLen=134, splitAlphabet=2
000001390008		139-8:hzwSmsToStr: i=2, splitLen=10, splitAlphabet=2
000001390011		139-11:hzwWinHandleMsg: MSG_PRE_FULL_PAINT
000001390014		139-14:hzwWinHandleMsg: MSG_FULL_PAINT
000001400015		140-15:hzwWinHandleMsg: MSG_END_FULL_PAINT
000001680016		168-16:hzwWinHandleMsg: KEY_2 KEY_PRESSED
000001680031		168-31:hzwWinHandleMsg: KEY_2 KEY_RELEASED
000001680032		168-32:hzwReadSmsByIds: recordId=0, storage=0, dualSys=0
000001980052		198-52:hzwWinHandleMsg: MSG_LOSE_FOCUS
000002560031		256-31:hzwWinHandleMsg: MSG_GET_FOCUS
000002560038		256-38:hzwWinHandleMsg: MSG_PRE_FULL_PAINT
000002560041		256-41:hzwWinHandleMsg: MSG_FULL_PAINT
000002570025		257-25:hzwWinHandleMsg: MSG_END_FULL_PAINT
000002580011		258-11:hzwWinHandleMsg: KEY_CANCEL KEY_RELEASED
000002910012		291-12:hzwWinHandleMsg: MSG_LOSE_FOCUS
000003780023		378-23:HzwNewSms: recordId=0, storage=1, dualSys=0
000003780024		378-24:HzwNewSms: find at 0
000005700047		570-47:HzwNewSms: recordId=4, storage=1, dualSys=0
000005700053		570-53:HzwNewSms: find at 1
000007260014		726-14:hzwWinHandleMsg: MSG_GET_FOCUS
000007300026		730-26:hzwWinHandleMsg: MSG_LOSE_FOCUS
000007300041		730-41:hzwWinHandleMsg: MSG_PRE_FULL_PAINT
000007300044		730-44:hzwWinHandleMsg: MSG_FULL_PAINT
000007300049		730-49:hzwWinHandleMsg: MSG_END_FULL_PAINT
000007500033		750-33:hzwWinHandleMsg: MSG_GET_FOCUS
000007500040		750-40:hzwWinHandleMsg: MSG_PRE_FULL_PAINT
000007500043		750-43:hzwWinHandleMsg: MSG_FULL_PAINT
000007500049		750-49:hzwWinHandleMsg: MSG_END_FULL_PAINT
000007510052		751-52:hzwWinHandleMsg: KEY_CANCEL KEY_RELEASED
000007880013		788-13:hzwWinHandleMsg: KEY_2 KEY_PRESSED
000007880029		788-29:hzwWinHandleMsg: KEY_2 KEY_RELEASED
000007880030		788-30:hzwReadSmsByIds: recordId=0, storage=1, dualSys=0
000007950044		795-44:hzwSmsToStr: dataLen=612, alphabet=0, splitCount=4, language=0, singleShift=0, lockShift=0
000007950045		795-45:hzwSmsToStr: i=0, splitLen=153, splitAlphabet=0
000007950046		795-46:hzwSmsToStr: len=153, splitLen=153
000007950047		795-47:hzwSmsToStr: i=1, splitLen=153, splitAlphabet=0
000007950048		795-48:hzwSmsToStr: len=153, splitLen=153
000007950049		795-49:hzwSmsToStr: i=2, splitLen=153, splitAlphabet=0
000007950050		795-50:hzwSmsToStr: len=153, splitLen=153
000007950051		795-51:hzwSmsToStr: i=3, splitLen=153, splitAlphabet=0
000007960001		796-1:hzwSmsToStr: len=153, splitLen=153
000007960004		796-4:hzwWinHandleMsg: MSG_PRE_FULL_PAINT
000007960007		796-7:hzwWinHandleMsg: MSG_FULL_PAINT
000007960010		796-10:hzwWinHandleMsg: MSG_END_FULL_PAINT
000008150042		815-42:hzwWinHandleMsg: KEY_2 KEY_PRESSED
000008160029		816-29:hzwWinHandleMsg: KEY_2 KEY_RELEASED
000008160030		816-30:hzwReadSmsByIds: recordId=4, storage=1, dualSys=0
000008220048		822-48:hzwSmsToStr: dataLen=388, alphabet=0, splitCount=3, language=0, singleShift=0, lockShift=0
000008220049		822-49:hzwSmsToStr: i=0, splitLen=153, splitAlphabet=0
000008220050		822-50:hzwSmsToStr: len=153, splitLen=153
000008220051		822-51:hzwSmsToStr: i=1, splitLen=153, splitAlphabet=0
000008230001		823-1:hzwSmsToStr: len=153, splitLen=153
000008230002		823-2:hzwSmsToStr: i=2, splitLen=82, splitAlphabet=0
000008230003		823-3:hzwSmsToStr: len=82, splitLen=82
000008230008		823-8:hzwWinHandleMsg: MSG_PRE_FULL_PAINT
000008230011		823-11:hzwWinHandleMsg: MSG_FULL_PAINT
000008230030		823-30:hzwWinHandleMsg: MSG_END_FULL_PAINT
000008380015		838-15:hzwWinHandleMsg: KEY_2 KEY_PRESSED
000008390010		839-10:hzwWinHandleMsg: KEY_2 KEY_RELEASED
000008390011		839-11:hzwReadSmsByIds: recordId=0, storage=0, dualSys=0
000008580004		858-4:hzwWinHandleMsg: KEY_2 KEY_PRESSED
000008580054		858-54:hzwWinHandleMsg: KEY_2 KEY_RELEASED
000008580055		858-55:hzwReadSmsByIds: recordId=0, storage=0, dualSys=0
000008970002		897-2:hzwWinHandleMsg: MSG_LOSE_FOCUS
```
