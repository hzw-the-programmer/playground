#include "log.h"

#if (LOG_MODULE & LOG_MODULE_HTTP) && (LOG_LEVEL >= LOG_LEVEL_FATAL)
#define LOG_HTTP_FATAL(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_HTTP, LOG_LEVEL_FATAL, fmt, ##__VA_ARGS__)
#else
#define LOG_HTTP_FATAL
#endif

#if (LOG_MODULE & LOG_MODULE_HTTP) && (LOG_LEVEL >= LOG_LEVEL_ERROR)
#define LOG_HTTP_ERROR(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_HTTP, LOG_LEVEL_ERROR, fmt, ##__VA_ARGS__)
#else
#define LOG_HTTP_ERROR
#endif

#if (LOG_MODULE & LOG_MODULE_HTTP) && (LOG_LEVEL >= LOG_LEVEL_WARN)
#define LOG_HTTP_WARN(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_HTTP, LOG_LEVEL_WARN, fmt, ##__VA_ARGS__)
#else
#define LOG_HTTP_WARN
#endif

#if (LOG_MODULE & LOG_MODULE_HTTP) && (LOG_LEVEL >= LOG_LEVEL_INFO)
#define LOG_HTTP_INFO(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_HTTP, LOG_LEVEL_INFO, fmt, ##__VA_ARGS__)
#else
#define LOG_HTTP_INFO
#endif

#if (LOG_MODULE & LOG_MODULE_HTTP) && (LOG_LEVEL >= LOG_LEVEL_DEBUG)
#define LOG_HTTP_DEBUG(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_HTTP, LOG_LEVEL_DEBUG, fmt, ##__VA_ARGS__)
#else
#define LOG_HTTP_DEBUG
#endif