#include "log.h"

#define LOG_HTTP_FATAL
#define LOG_HTTP_ERROR
#define LOG_HTTP_WARN
#define LOG_HTTP_INFO
#define LOG_HTTP_DEBUG

#if (LOG_MODULE & LOG_MODULE_HTTP)
#if (LOG_LEVEL >= LOG_LEVEL_FATAL)
#undef LOG_HTTP_FATAL
#define LOG_HTTP_FATAL(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_HTTP, LOG_LEVEL_FATAL, fmt, ##__VA_ARGS__)
#endif

#if (LOG_LEVEL >= LOG_LEVEL_ERROR)
#undef LOG_HTTP_ERROR
#define LOG_HTTP_ERROR(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_HTTP, LOG_LEVEL_ERROR, fmt, ##__VA_ARGS__)
#endif

#if (LOG_LEVEL >= LOG_LEVEL_WARN)
#undef LOG_HTTP_WARN
#define LOG_HTTP_WARN(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_HTTP, LOG_LEVEL_WARN, fmt, ##__VA_ARGS__)
#endif

#if (LOG_LEVEL >= LOG_LEVEL_INFO)
#undef LOG_HTTP_INFO
#define LOG_HTTP_INFO(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_HTTP, LOG_LEVEL_INFO, fmt, ##__VA_ARGS__)
#endif

#if (LOG_LEVEL >= LOG_LEVEL_DEBUG)
#undef LOG_HTTP_DEBUG
#define LOG_HTTP_DEBUG(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_HTTP, LOG_LEVEL_DEBUG, fmt, ##__VA_ARGS__)
#endif
#endif