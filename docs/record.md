## file
MS_Code\MS_MMI\source\mmi_app\app\record\c\mmirecord_wintab.c
## func
MMIAPIRECORD_OpenMainWin
HandleRecordMainPlayWinMsg

## file
MS_Code\MS_MMI\source\mmi_app\app\record\c\mmirecord.c
## func
MMIRECORD_StartRecordNormal

## example
typedef enum {
	AUDIO_EVENT_PAUSE,
	AUDIO_EVENT_RESUME,
	AUDIO_EVENT_STOP,
	AUDIO_EVENT_END,
	AUDIO_EVENT_DISK_FULL,
} AUDIO_EVENT;

typedef void (*AUDIO_CB)(AUDIO_EVENT event);

static MMISRV_HANDLE_T audio_play_handle = INVALID_HANDLE;
static AUDIO_CB audio_play_cb = NULL;

void audio_set_play_handle(MMISRV_HANDLE_T handle) {
	audio_play_handle = handle;
}

MMISRV_HANDLE_T audio_get_play_handle() {
	return audio_play_handle;
}

void audio_set_play_cb(AUDIO_CB cb) {
	audio_play_cb = cb;
}

AUDIO_CB audio_get_play_cb() {
	return audio_play_cb;
}

void audio_call_play_cb(AUDIO_EVENT event) {
	AUDIO_CB cb = audio_get_play_cb();

	if (!cb) {
		return;
	}

	cb(event);
}

void audio_play_pause() {
	MMISRV_HANDLE_T handle = audio_get_play_handle();

	SCI_TRACE_LOW("hzw audio_play_pause handle=0x%x", handle);

	if (handle <= INVALID_HANDLE) {
		return;
	}

	MMISRVAUD_Pause(handle);

	audio_call_play_cb(AUDIO_EVENT_PAUSE);
}

void audio_play_resume() {
	MMISRV_HANDLE_T handle = audio_get_play_handle();

	SCI_TRACE_LOW("hzw audio_play_resume handle=0x%x", handle);

	if (handle <= INVALID_HANDLE) {
		return;
	}

	MMISRVAUD_Resume(handle);

	audio_call_play_cb(AUDIO_EVENT_RESUME);
}

void audio_play_stop() {
	MMISRV_HANDLE_T handle = audio_get_play_handle();

	SCI_TRACE_LOW("hzw audio_play_stop handle=0x%x", handle);

	if (handle <= INVALID_HANDLE) {
		return;
	}

	MMISRVAUD_Stop(handle);

	MMISRVMGR_Free(handle);

	audio_set_play_handle(INVALID_HANDLE);

	audio_call_play_cb(AUDIO_EVENT_STOP);
	audio_set_play_cb(NULL);
}

static BOOLEAN audio_play_notify(MMISRV_HANDLE_T handle, MMISRVMGR_NOTIFY_PARAM_T *param) {
	MMISRVAUD_REPORT_T *report_ptr = NULL;

	SCI_TRACE_LOW("hzw audio_play_notify");

	if (handle <= INVALID_HANDLE || param == NULL) {
		return FALSE;
	}

	report_ptr = (MMISRVAUD_REPORT_T*)param->data;
	if (report_ptr == NULL) {
		return FALSE;
	}

	SCI_TRACE_LOW("hzw audio_play_notify report_ptr->report=%d", report_ptr->report);

	switch (report_ptr->report) {
	case MMISRVAUD_REPORT_END:
		MMISRVAUD_Stop(handle);

		MMISRVMGR_Free(handle);

		audio_set_play_handle(INVALID_HANDLE);

		audio_call_play_cb(AUDIO_EVENT_END);
		audio_set_play_cb(NULL);
		break;

	default:
		break;
	}
}

void audio_play_data(uint8 *data, uint32 len, AUDIO_CB cb) {
	MMISRVMGR_SERVICE_REQ_T req = {0};
	MMISRVAUD_TYPE_T srv = {0};
	MMISRV_HANDLE_T handle = INVALID_HANDLE;

	SCI_TRACE_LOW("hzw audio_play_data");

	audio_play_stop();

	req.notify = audio_play_notify;
	req.pri = MMISRVAUD_PRI_NORMAL;

	srv.volume = MMIAPISET_GetMultimVolume();
	srv.play_times = 1;
	srv.info.type = MMISRVAUD_TYPE_RING_BUF;
	srv.info.ring_buf.fmt = MMISRVAUD_RING_FMT_MP3;
	srv.info.ring_buf.data = data;
	srv.info.ring_buf.data_len = len;
	srv.all_support_route = MMISRVAUD_ROUTE_NONE;

	handle = MMISRVMGR_Request(STR_SRV_AUD_NAME, &req, &srv);
	if (handle <= INVALID_HANDLE) {
		return;
	}
	if (!MMISRVAUD_Play(handle, 0)) {
		MMISRVMGR_Free(handle);
		return;
	}
	SCI_TRACE_LOW("hzw audio_play_data handle=0x%x", handle);

	audio_set_play_handle(handle);
	audio_set_play_cb(cb);
}

void audio_play_file(const wchar *filename, AUDIO_CB cb) {
	MMISRVMGR_SERVICE_REQ_T req = {0};
	MMISRVAUD_TYPE_T srv = {0};
	MMISRV_HANDLE_T handle = INVALID_HANDLE;

	SCI_TRACE_LOW("hzw audio_play_file");

	{
		MMIFILE_HANDLE filehandle = 0;
		uint32 size = 0;

		filehandle = MMIAPIFMM_CreateFile(filename, SFS_MODE_READ | SFS_MODE_WRITE | SFS_MODE_OPEN_ALWAYS, NULL, NULL);
		size = MMIAPIFMM_GetFileSize(filehandle);
		MMIAPIFMM_CloseFile(filehandle);
		SCI_TRACE_LOW("hzw audio_play_file size=%d", size);
	}

	audio_play_stop();

	req.ind_data = 0;
	req.is_async = FALSE;
	req.is_auto_resume_off = FALSE;
	req.notify = audio_play_notify;
	req.pri = MMISRVAUD_PRI_NORMAL;

	srv.duation = 0;
	srv.eq_mode = 0;
	srv.is_mixing_enable = FALSE;
	srv.play_times = 1;
	srv.volume = MMIAPISET_GetMultimVolume();
	srv.all_support_route = MMISRVAUD_ROUTE_AUTO;
	srv.info.type = MMISRVAUD_TYPE_RING_FILE;
	srv.info.ring_file.fmt = MMISRVAUD_RING_FMT_AMR;
	srv.info.ring_file.name = filename;
	srv.info.ring_file.name_len = MMIAPICOM_Wstrlen(filename);

	handle = MMISRVMGR_Request(STR_SRV_AUD_NAME, &req, &srv);
	if (handle <= INVALID_HANDLE) {
		return;
	}
	if (!MMISRVAUD_Play(handle, 0)) {
		MMISRVMGR_Free(handle);
		return;
	}
	SCI_TRACE_LOW("hzw audio_play_file handle=0x%x", handle);

	audio_set_play_handle(handle);
	audio_set_play_cb(cb);
}

static MMISRV_HANDLE_T audio_record_handle = INVALID_HANDLE;
static AUDIO_CB audio_record_cb = NULL;

void audio_set_record_handle(MMISRV_HANDLE_T handle) {
	audio_record_handle = handle;
}

MMISRV_HANDLE_T audio_get_record_handle() {
	return audio_record_handle;
}

void audio_set_record_cb(AUDIO_CB cb) {
	audio_record_cb = cb;
}

AUDIO_CB audio_get_record_cb() {
	return audio_record_cb;
}

void audio_call_record_cb(AUDIO_EVENT event) {
	AUDIO_CB cb = audio_get_record_cb();

	if (!cb) {
		return;
	}

	cb(event);
}

void audio_record_pause() {
	MMISRV_HANDLE_T handle = audio_get_record_handle();
	
	SCI_TRACE_LOW("hzw audio_record_pause handle=0x%x", handle);

	if (handle <= INVALID_HANDLE) {
		return;
	}

	MMISRVAUD_Pause(handle);

	audio_call_record_cb(AUDIO_EVENT_PAUSE);
}

void audio_record_resume() {
	MMISRV_HANDLE_T handle = audio_get_record_handle();
	
	SCI_TRACE_LOW("hzw audio_record_resume handle=0x%x", handle);

	if (handle <= INVALID_HANDLE) {
		return;
	}

	MMISRVAUD_Resume(handle);

	audio_call_record_cb(AUDIO_EVENT_RESUME);
}

void audio_record_stop() {
	MMISRV_HANDLE_T handle = audio_get_record_handle();

	SCI_TRACE_LOW("hzw audio_record_stop handle=0x%x", handle);

	if (handle <= INVALID_HANDLE) {
		return;
	}

	MMISRVAUD_Stop(handle);

	MMISRVMGR_Free(handle);

	audio_set_record_handle(INVALID_HANDLE);

	audio_call_record_cb(AUDIO_EVENT_STOP);
	audio_set_record_cb(NULL);
}

static BOOLEAN audio_record_notify(MMISRV_HANDLE_T handle, MMISRVMGR_NOTIFY_PARAM_T *param) {
	MMISRVAUD_REPORT_T *report_ptr = NULL;
	uint32 freeh = 0, freel = 0;

	SCI_TRACE_LOW("hzw audio_record_notify");

	if (handle <= INVALID_HANDLE || param == NULL) {
		return FALSE;
	}

	report_ptr = (MMISRVAUD_REPORT_T*)param->data;
	if (report_ptr == NULL) {
		return FALSE;
	}

	SCI_TRACE_LOW("hzw audio_record_notify report_ptr->report=%d", report_ptr->report);

	switch (report_ptr->report) {
	case MMISRVAUD_REPORT_END:
		MMISRVAUD_Stop(handle);

		MMISRVMGR_Free(handle);

		audio_set_record_handle(INVALID_HANDLE);

		SCI_TRACE_LOW("hzw audio_record_notify report_ptr->data1=%d", report_ptr->data1);

		MMIAPIFMM_GetDeviceFreeSpace(L"D", 1, &freeh, &freel);
		SCI_TRACE_LOW("hzw audio_record_notify freeh=%d, freel=%d", freeh, freel);

		if (!MMIAPIFMM_IsEnoughSpace(MMIFMM_STORE_TYPE_FIXED , MMI_DEVICE_UDISK, 20000, NULL)) {
			SCI_TRACE_LOW("hzw audio_record_notify not enough space");
			audio_call_record_cb(AUDIO_EVENT_DISK_FULL);
		} else {
			audio_call_record_cb(AUDIO_EVENT_END);
		}
		audio_set_record_cb(NULL);
		break;

	default:
		break;
	}
}

void audio_record_start(const wchar *filename, AUDIO_CB cb) {
	MMISRVMGR_SERVICE_REQ_T req = {0};
	MMISRVAUD_TYPE_T srv = {0};
	MMISRV_HANDLE_T handle = INVALID_HANDLE;
	uint32 len = 0;

	SCI_TRACE_LOW("hzw audio_record_start");

	audio_record_stop();

	len = MMIAPICOM_Wstrlen(filename);

	if (MMIAPIFMM_IsFileExist(filename, len)) {
		MMIAPIFMM_DeleteFile(filename, NULL);
	}

	req.notify = audio_record_notify;
	req.pri = MMISRVAUD_PRI_NORMAL;

	srv.duation = 0;
	srv.eq_mode = 0;
	srv.is_mixing_enable = FALSE;
	srv.play_times = 1;
	srv.all_support_route = MMISRVAUD_ROUTE_NONE;
	srv.volume = AUD_MAX_SPEAKER_VOLUME;

	srv.info.type = MMISRVAUD_TYPE_RECORD_FILE;		 
	srv.info.record_file.fmt = MMISRVAUD_RECORD_FMT_AMR;
	srv.info.record_file.name = filename;
	srv.info.record_file.name_len = len;	  
	srv.info.record_file.source = MMISRVAUD_RECORD_SOURCE_NORMAL;
	srv.info.record_file.frame_len= MMISRVAUD_RECORD_FRAME_LEN_DEFAULT;

	handle = MMISRVMGR_Request(STR_SRV_AUD_NAME, &req, &srv);
	if (handle <= INVALID_HANDLE) {
		return;
	}
	if (!MMISRVAUD_Play(handle, 0)) {
		MMISRVMGR_Free(handle);
		return;
	}
	SCI_TRACE_LOW("hzw audio_record_start handle=0x%x", handle);

	audio_set_record_handle(handle);
	audio_set_record_cb(cb);
}

void audio_cb_test(AUDIO_EVENT event) {
	char *str = NULL;
	switch (event) {
	case AUDIO_EVENT_PAUSE:
		str = "PAUSE";
		break;
	case AUDIO_EVENT_RESUME:
		str = "RESUME";
		break;
	case AUDIO_EVENT_STOP:
		str = "STOP";
		break;
	case AUDIO_EVENT_END:
		str = "END";
		break;
	case AUDIO_EVENT_DISK_FULL:
		str = "DISK_FULL";
		break;
	default:
		str = "UNKNOWN";
		break;
	}
	SCI_TRACE_LOW("hzw audio_cb_test event=%s", str);
}

## log
000001700051		170-51:hzw audio_record_start
000001700052		170-52:hzw audio_record_stop handle=0x0
000001820017		182-17:hzw audio_record_start handle=0x10c0001
000002410029		241-29:hzw audio_record_pause handle=0x10c0001
000002430008		243-8:hzw audio_cb_test event=PAUSE
000003330006		333-6:hzw audio_record_resume handle=0x10c0001
000003390014		339-14:hzw audio_cb_test event=RESUME
000003710016		371-16:hzw audio_record_pause handle=0x10c0001
000003720042		372-42:hzw audio_cb_test event=PAUSE
000003890012		389-12:hzw audio_record_resume handle=0x10c0001
000003950023		395-23:hzw audio_cb_test event=RESUME
000004140057		414-57:hzw audio_record_stop handle=0x10c0001
000004170047		417-47:hzw audio_cb_test event=STOP
000004370013		437-13:hzw audio_play_file
000004380015		438-15:hzw audio_play_file size=17446
000004380016		438-16:hzw audio_play_stop handle=0x0
000004480052		448-52:hzw audio_play_file handle=0x1140001
000004760052		476-52:hzw audio_play_notify
000004760053		476-53:hzw audio_play_notify report_ptr->report=1
000004770044		477-44:hzw audio_cb_test event=END
000006190046		619-46:hzw audio_play_data
000006190047		619-47:hzw audio_play_stop handle=0x0
000006290045		629-45:hzw audio_play_data handle=0x11b0001
000006380010		638-10:hzw audio_play_notify
000006380011		638-11:hzw audio_play_notify report_ptr->report=1
000006380051		638-51:hzw audio_cb_test event=END
000006530037		653-37:hzw audio_play_data
000006530038		653-38:hzw audio_play_stop handle=0x0
000006630007		663-7:hzw audio_play_data handle=0x11d0001
000006680034		668-34:hzw audio_play_notify
000006680035		668-35:hzw audio_play_notify report_ptr->report=1
000006690010		669-10:hzw audio_cb_test event=END
000006870011		687-11:hzw audio_play_data
000006870012		687-12:hzw audio_play_stop handle=0x0
000006960048		696-48:hzw audio_play_data handle=0x11f0001
000007030027		703-27:hzw audio_play_notify
000007030028		703-28:hzw audio_play_notify report_ptr->report=1
000007030068		703-68:hzw audio_cb_test event=END
000007170023		717-23:hzw audio_play_data
000007170024		717-24:hzw audio_play_stop handle=0x0
000007260049		726-49:hzw audio_play_data handle=0x1210001
000007330019		733-19:hzw audio_play_notify
000007330020		733-20:hzw audio_play_notify report_ptr->report=1
000007330060		733-60:hzw audio_cb_test event=END
000007430052		743-52:hzw audio_play_data
000007430056		743-56:hzw audio_play_stop handle=0x0
000007540010		754-10:hzw audio_play_data handle=0x1230001
000007580019		758-19:hzw audio_play_notify
000007580020		758-20:hzw audio_play_notify report_ptr->report=1
000007580060		758-60:hzw audio_cb_test event=END
000007780002		778-2:hzw audio_play_file
000007790003		779-3:hzw audio_play_file size=17446
000007790004		779-4:hzw audio_play_stop handle=0x0
000007900005		790-5:hzw audio_play_file handle=0x1250001
000007940035		794-35:hzw audio_play_data
000007940036		794-36:hzw audio_play_stop handle=0x1250001
000007960029		796-29:hzw audio_cb_test event=STOP
000008030017		803-17:hzw audio_play_data handle=0x1260000
000008060052		806-52:hzw audio_play_notify
000008060053		806-53:hzw audio_play_notify report_ptr->report=1
000008070037		807-37:hzw audio_cb_test event=END
000008240002		824-2:hzw audio_play_file
000008250003		825-3:hzw audio_play_file size=17446
000008250004		825-4:hzw audio_play_stop handle=0x0
000008360005		836-5:hzw audio_play_file handle=0x1280001
000008420017		842-17:hzw audio_play_data
000008420018		842-18:hzw audio_play_stop handle=0x1280001
000008440006		844-6:hzw audio_cb_test event=STOP
000008510002		851-2:hzw audio_play_data handle=0x1290000
000008540038		854-38:hzw audio_play_notify
000008540039		854-39:hzw audio_play_notify report_ptr->report=1
000008550015		855-15:hzw audio_cb_test event=END
000008730029		873-29:hzw audio_play_data
000008730030		873-30:hzw audio_play_stop handle=0x0
000008830003		883-3:hzw audio_play_data handle=0x12b0001
000008870021		887-21:hzw audio_play_file
000008880019		888-19:hzw audio_play_file size=17446
000008880020		888-20:hzw audio_play_stop handle=0x12b0001
000008900003		890-3:hzw audio_cb_test event=STOP
000008960049		896-49:hzw audio_play_file handle=0x12c0000
000009040052		904-52:hzw audio_play_data
000009040053		904-53:hzw audio_play_stop handle=0x12c0000
000009060052		906-52:hzw audio_cb_test event=STOP
000009130041		913-41:hzw audio_play_data handle=0x12d0000
000009170020		917-20:hzw audio_play_notify
000009170021		917-21:hzw audio_play_notify report_ptr->report=1
000009170061		917-61:hzw audio_cb_test event=END
000009480017		948-17:hzw audio_play_file
000009490016		949-16:hzw audio_play_file size=17446
000009490017		949-17:hzw audio_play_stop handle=0x0
000009580030		958-30:hzw audio_play_file handle=0x12f0001
000009630005		963-5:hzw audio_play_pause handle=0x12f0001
000009650021		965-21:hzw audio_cb_test event=PAUSE
000009790027		979-27:hzw audio_play_resume handle=0x12f0001
000009860003		986-3:hzw audio_cb_test event=RESUME
000009920031		992-31:hzw audio_play_pause handle=0x12f0001
000009940028		994-28:hzw audio_cb_test event=PAUSE
000010090014		1009-14:hzw audio_play_resume handle=0x12f0001
000010150042		1015-42:hzw audio_cb_test event=RESUME
000010240005		1024-5:hzw audio_play_pause handle=0x12f0001
000010260033		1026-33:hzw audio_cb_test event=PAUSE
000010360058		1036-58:hzw audio_play_resume handle=0x12f0001
000010440024		1044-24:hzw audio_cb_test event=RESUME
000010490023		1049-23:hzw audio_play_stop handle=0x12f0001
000010510040		1051-40:hzw audio_cb_test event=STOP
000010660045		1066-45:hzw audio_play_file
000010670043		1067-43:hzw audio_play_file size=17446
000010670044		1067-44:hzw audio_play_stop handle=0x0
000010780037		1078-37:hzw audio_play_file handle=0x1340001
000010850052		1085-52:hzw audio_play_data
000010850053		1085-53:hzw audio_play_stop handle=0x1340001
000010870052		1087-52:hzw audio_cb_test event=STOP
000010940043		1094-43:hzw audio_play_data handle=0x1350000
000010980026		1098-26:hzw audio_play_notify
000010980027		1098-27:hzw audio_play_notify report_ptr->report=1
000010980067		1098-67:hzw audio_cb_test event=END
000011090007		1109-7:hzw audio_play_file
000011100053		1110-53:hzw audio_play_file size=17446
000011100054		1110-54:hzw audio_play_stop handle=0x0
000011210050		1121-50:hzw audio_play_file handle=0x1370001
000011370023		1137-23:hzw audio_play_pause handle=0x1370001
000011390038		1139-38:hzw audio_cb_test event=PAUSE
000011530056		1153-56:hzw audio_play_resume handle=0x1370001
000011600020		1160-20:hzw audio_cb_test event=RESUME
000011710023		1171-23:hzw audio_play_notify
000011710024		1171-24:hzw audio_play_notify report_ptr->report=1
000011710071		1171-71:hzw audio_cb_test event=END
