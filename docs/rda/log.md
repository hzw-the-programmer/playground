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
