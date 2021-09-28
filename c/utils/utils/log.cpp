#include <stdio.h>
#include <stdarg.h>

void log(const char *file, int line, int level, const char *module, const char *fmt, ...) {
    va_list args;

    va_start(args, fmt);
    printf("%s:%d:%d:%s:", file, line, level, module);
    vprintf(fmt, args);
    printf("\n");
    va_end(args);
}

#define LOG(level, module, fmt, ...) log(__FILE__, __LINE__, level, module, fmt, ##__VA_ARGS__)

void test_log() {
    LOG(0, "http", "code=%d, content_length=%d", 200, 1024);
}
