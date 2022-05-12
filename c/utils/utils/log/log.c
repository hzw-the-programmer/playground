#include <stdio.h>
#include <stdarg.h>

#include "log.h"

static char* log_level_str(int log_level) {
#define XXX(LEVEL) \
case LOG_LEVEL_##LEVEL: \
return #LEVEL;

    switch (log_level) {
        LOG_LEVEL_MAP
        default:
            return "DEBUG";
    }

#undef XXX
}

static char* log_module_str(int log_level) {
#define YYY(MODULE) \
case LOG_MODULE_##MODULE: \
return #MODULE;

    switch (log_level) {
        LOG_MODULE_MAP(YYY)
        default:
            return "DEBUG";
    }

#undef YYY
}

void log(const char *file, int line, int module, int level, const char *fmt, ...) {
    va_list args;

    va_start(args, fmt);
    printf("%s:%d:%s:%s:", file, line, log_module_str(module), log_level_str(level));
    vprintf(fmt, args);
    printf("\n");
    va_end(args);
}
