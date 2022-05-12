#include "config.h"

#define LOG_LEVEL_FATAL 0
#define LOG_LEVEL_ERROR 1
#define LOG_LEVEL_WARN 2
#define LOG_LEVEL_INFO 3
#define LOG_LEVEL_DEBUG 4

#define LOG_LEVEL_MAP \
    XXX(FATAL) \
    XXX(ERROR) \
    XXX(WARN) \
    XXX(INFO) \
    XXX(DEBUG)

#define LOG_MODULE_UI       0x01
#define LOG_MODULE_TCP   0x02
#define LOG_MODULE_HTTP 0x04
#define LOG_MODULE_UDP   0x08
#define LOG_MODULE_QUIC 0x10

#define LOG_MODULE_MAP(XXX) \
    XXX(UI) \
    XXX(TCP) \
    XXX(HTTP) \
    XXX(UDP) \
    XXX(QUIC)

extern void log(const char *file, int line, int module, int level, const char *fmt, ...);

#if LOG_LEVEL >= LOG_LEVEL_FATAL
#define LOG_FATAL(module, fmt, ...) log(__FILE__, __LINE__, module, LOG_LEVEL_FATAL, fmt, ##__VA_ARGS__)
#else
#define LOG_FATAL
#endif

#if LOG_LEVEL >= LOG_LEVEL_ERROR
#define LOG_ERROR(module, fmt, ...) log(__FILE__, __LINE__, module, LOG_LEVEL_ERROR, fmt, ##__VA_ARGS__)
#else
#define LOG_ERROR
#endif

#if LOG_LEVEL >= LOG_LEVEL_WARN
#define LOG_WARN(module, fmt, ...) log(__FILE__, __LINE__, module, LOG_LEVEL_WARN, fmt, ##__VA_ARGS__)
#else
#define LOG_WARN
#endif

#if LOG_LEVEL >= LOG_LEVEL_INFO
#define LOG_INFO(module, fmt, ...) log(__FILE__, __LINE__, module, LOG_LEVEL_INFO, fmt, ##__VA_ARGS__)
#else
#define LOG_INFO
#endif

#if LOG_LEVEL >= LOG_LEVEL_DEBUG
#define LOG_DEBUG(module, fmt, ...) log(__FILE__, __LINE__, module, LOG_LEVEL_DEBUG, fmt, ##__VA_ARGS__)
#else
#define LOG_DEBUG
#endif
