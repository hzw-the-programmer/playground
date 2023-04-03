#define LOG_BUF_LEN 1024
#define LOG_PREFIX "hzw:"

static FS_HANDLE g_log = FS_INVALID_FILE_HANDLE;

#define LOG(fmt, ...) log(__FILE__, __LINE__, __func__, fmt, ##__VA_ARGS__)
static void log(const char *file, int line, const char *func, const char *fmt, ...) {
	char buf[LOG_BUF_LEN];
	char *p, *file_p;
	int len, wlen, ret;
	va_list ap;

	if (g_log < 0) {
		g_log = MMI_FS_Open(L"D:/hzw.log", FS_CREATE_ALWAYS | FS_READ_WRITE);
		if (g_log < 0) {			
			mmi_trace(MMI_TRACE_LEVEL_1, LOG_PREFIX "create log file failed,g_log=%d", g_log);
			return;
		}
	}

	file_p = strrchr(file, '/');
	if (file_p) {
		file_p += 1;
	} else {
		file_p = file;
	}

	p = buf;
	len = sprintf(p, LOG_PREFIX "%s:%d:%s:", file_p, line, func);
	p += len;

	va_start(ap, fmt);
	len += vsnprintf(p, LOG_BUF_LEN, fmt, ap);
	va_end(ap);
	buf[len++] = '\n';
	buf[len] = '\0';
	ret = MMI_FS_Write(g_log, buf, len, &wlen);
	mmi_trace(MMI_TRACE_LEVEL_1, "%s", buf);
}

static void exit_setting()
{
}

static void enter_setting()
{
    EntryNewScreen(777, exit_setting, enter_setting, NULL);
    gdi_layer_clear(GDI_COLOR_RED);
    SetRightSoftkeyFunction(GoBackHistory, KEY_EVENT_UP);
    gdi_layer_blt_previous(0, 0, UI_DEVICE_WIDTH-1, UI_DEVICE_HEIGHT-1);
}

static void lsk() {
    enter_setting();
}

static void exit_app() {
}

#include "med_main.h"

#define AUDIO_FILE_NAME L"D:/test.amr"

static void start_record_cb(mdi_result ret) {
	// MDI_AUDIO_TERMINATED = 6
	LOG("ret=%d", ret);
}

static void start_record(U16 *fn) {
	mdi_handle handle = 0;
	mdi_result ret;

	ret = mdi_audio_start_record(fn, MED_TYPE_AMR, &handle, start_record_cb);
	LOG("ret=%d,handle=%d", ret, handle);
}

static void stop_record() {
	mdi_result ret;

	ret = mdi_audio_stop_record();
	LOG("ret=%d", ret);
}

static void play_record_cb(mdi_result ret) {
	LOG("ret=%d", ret);
}

static void play_record(U16 *fn) {
	mdi_handle handle;
	U8 volume = 6;
	mdi_result ret;

	ret = mdi_audio_play_file_with_vol_path(fn, DEVICE_AUDIO_PLAY_ONCE,
		&handle, play_record_cb, volume, MDI_DEVICE_SPEAKER, 0);
	LOG("ret=%d,handle=%d", ret, handle);
}

static void key_1() {
}

static void key_2() {
	U16 *fn = AUDIO_FILE_NAME;
	int ret;

	//ret = MMI_FS_Delete(fn);
	//LOG("MMI_FS_Delete=%d", ret);

	start_record(fn);
}

static void key_3() {
	stop_record();
}

static void key_4() {
}

static void key_5() {
	U16 *fn = AUDIO_FILE_NAME;

	play_record(fn);
}

static void enter_app() {
    EntryNewScreen(666, exit_app, enter_app, NULL);
    // no history
    // ExecuteCurrExitHandler_Ext -> GenericExitScreen
    //EntryNewScreen(666, exit_app, NULL, NULL);
    SetLeftSoftkeyFunction(lsk, KEY_EVENT_UP);
    SetRightSoftkeyFunction(GoBackHistory, KEY_EVENT_UP);
	SetKeyHandler(key_1, KEY_1, KEY_EVENT_DOWN);
	SetKeyHandler(key_2, KEY_2, KEY_EVENT_DOWN);
	SetKeyHandler(key_3, KEY_3, KEY_EVENT_DOWN);
	SetKeyHandler(key_4, KEY_4, KEY_EVENT_DOWN);
	SetKeyHandler(key_5, KEY_5, KEY_EVENT_DOWN);

    gdi_layer_clear(GDI_COLOR_WHITE);

    gui_set_font(&MMI_small_font);
    gui_set_text_color(gui_color(0, 0, 0));

    gdi_layer_blt_previous(0, 0, UI_DEVICE_WIDTH-1, UI_DEVICE_HEIGHT-1);
}
