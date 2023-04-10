#define LOG_BUF_LEN 1024
#if defined(WIN32)
#define LOG_FILE_SEP_CHAR '\\'
#else
#define LOG_FILE_SEP_CHAR '/'
#endif
#define LOG_PREFIX "hzw"

static FS_HANDLE g_log = FS_INVALID_FILE_HANDLE;

#define LOG(fmt, ...) log(__FILE__, __LINE__, __FUNCTION__, fmt, ##__VA_ARGS__)

static void log(const char *file, int line, const char *func, const char *fmt, ...) {
	char buf[LOG_BUF_LEN];
	char *p, *file_p;
	int len, wlen, ret;
	va_list ap;
	MYTIME dt;

	if (g_log < 0) {
		g_log = MMI_FS_Open(L"D:/hzw.log", FS_CREATE_ALWAYS | FS_READ_WRITE);
		if (g_log < 0) {			
			mmi_trace(MMI_TRACE_LEVEL_1, LOG_PREFIX "create log file failed,g_log=%d", g_log);
			return;
		}
	}

	DTGetRTCTime(&dt);

	file_p = strrchr(file, LOG_FILE_SEP_CHAR);
	if (file_p) {
		file_p += 1;
	} else {
		file_p = file;
	}

	p = buf;
	len = sprintf(p, "[%d-%02d-%02d %02d:%02d:%02d][%s:%d:%s][%s]: ",
		dt.nYear, dt.nMonth, dt.nDay,
		dt.nHour, dt.nMin, dt.nSec,
		file_p, line, func,
		LOG_PREFIX);
	p += len;

	va_start(ap, fmt);
	len += vsnprintf(p, LOG_BUF_LEN, fmt, ap);
	va_end(ap);
	buf[len++] = '\n';
	buf[len] = '\0';
	ret = MMI_FS_Write(g_log, buf, len, &wlen);
	mmi_trace(MMI_TRACE_LEVEL_1, "%s", buf);
}

static void draw(const char *text) {
    gdi_layer_clear(GDI_COLOR_WHITE);

    gui_set_font(&MMI_small_font);
    gui_set_text_color(gui_color(0, 0, 0));
	gui_move_text_cursor(50, 50);
	gui_printf("%s", text);

    gdi_layer_blt_previous(0, 0, UI_DEVICE_WIDTH-1, UI_DEVICE_HEIGHT-1);
}

static UINT32 g_ticks;

static void timer_cb() {
	LOG("%d, %d", sxr_GetCurrentTaskId(), (csw_TMGetTick() - g_ticks) / (MMI_TICK1S/1000));
	draw("finish");
}

static void lsk() {
}

static void key_1() {
}

static void key_2() {
	LOG("%d", sxr_GetCurrentTaskId());
	draw("start");
	g_ticks = csw_TMGetTick();
	StartTimer(KEY_TIMER_ID69, 15000, timer_cb);
}

static void key_3() {
}

static void key_4() {
}

static void key_5() {
}

static void key_6() {
}

static void key_7() {
}

static void register_handlers() {
    SetLeftSoftkeyFunction(lsk, KEY_EVENT_UP);
    SetRightSoftkeyFunction(GoBackHistory, KEY_EVENT_UP);
	SetKeyHandler(key_1, KEY_1, KEY_EVENT_DOWN);
	SetKeyHandler(key_2, KEY_2, KEY_EVENT_DOWN);
	SetKeyHandler(key_3, KEY_3, KEY_EVENT_DOWN);
	SetKeyHandler(key_4, KEY_4, KEY_EVENT_DOWN);
	SetKeyHandler(key_5, KEY_5, KEY_EVENT_DOWN);
	SetKeyHandler(key_6, KEY_6, KEY_EVENT_DOWN);
	SetKeyHandler(key_7, KEY_7, KEY_EVENT_DOWN);
}

static void exit_app() {
}

static void enter_app() {
	LOG("%d", sxr_GetCurrentTaskId());

	EntryNewScreen(666, exit_app, enter_app, NULL);
    // no history
    // ExecuteCurrExitHandler_Ext -> GenericExitScreen
    //EntryNewScreen(666, exit_app, NULL, NULL);

	register_handlers();
	draw("");
}
