#include <stdio.h>
#include <stdarg.h>

#define LOG_LEVEL_FATAL 0
#define LOG_LEVEL_ERROR 1
#define LOG_LEVEL_WARN 2
#define LOG_LEVEL_INFO 3
#define LOG_LEVEL_DEBUG 4

#define LOG_LEVEL LOG_LEVEL_WARN

char* log_level_str(int log_level) {
    switch (log_level) {
        case LOG_LEVEL_FATAL:
            return "FATAL";
        case LOG_LEVEL_ERROR:
            return "ERROR";
        case LOG_LEVEL_WARN:
            return "WARN";
        case LOG_LEVEL_INFO:
            return "INFO";
        case LOG_LEVEL_DEBUG:
        default:
            return "DEBUG";
    }
}

void log(const char *file, int line, int log_level, const char *module, const char *fmt, ...) {
    va_list args;

    va_start(args, fmt);
    printf("%s:%d:%s:%s:", file, line, log_level_str(log_level), module);
    vprintf(fmt, args);
    printf("\n");
    va_end(args);
}

#define LOG_FATAL
#define LOG_ERROR
#define LOG_WARN
#define LOG_INFO
#define LOG_DEBUG

#if LOG_LEVEL >= LOG_LEVEL_FATAL
#undef LOG_FATAL
#define LOG_FATAL(module, fmt, ...) log(__FILE__, __LINE__, LOG_LEVEL_FATAL, module, fmt, ##__VA_ARGS__)
#endif

#if LOG_LEVEL >= LOG_LEVEL_ERROR
#undef LOG_ERROR
#define LOG_ERROR(module, fmt, ...) log(__FILE__, __LINE__, LOG_LEVEL_ERROR, module, fmt, ##__VA_ARGS__)
#endif

#if LOG_LEVEL >= LOG_LEVEL_WARN
#undef LOG_WARN
#define LOG_WARN(module, fmt, ...) log(__FILE__, __LINE__, LOG_LEVEL_WARN, module, fmt, ##__VA_ARGS__)
#endif

#if LOG_LEVEL >= LOG_LEVEL_INFO
#undef LOG_INFO
#define LOG_INFO(module, fmt, ...) log(__FILE__, __LINE__, LOG_LEVEL_INFO, module, fmt, ##__VA_ARGS__)
#endif

#if LOG_LEVEL >= LOG_LEVEL_DEBUG
#undef LOG_DEBUG
#define LOG_DEBUG(module, fmt, ...) log(__FILE__, __LINE__, LOG_LEVEL_DEBUG, module, fmt, ##__VA_ARGS__)
#endif

void test_log() {
    LOG_FATAL("http", "code=%d, content_length=%d", 200, 1024);
    LOG_ERROR("http", "code=%d, content_length=%d", 200, 1024);
    LOG_WARN("http", "code=%d, content_length=%d", 200, 1024);
    LOG_INFO("http", "code=%d, content_length=%d", 200, 1024);
    LOG_DEBUG("http", "code=%d, content_length=%d", 200, 1024);
}
