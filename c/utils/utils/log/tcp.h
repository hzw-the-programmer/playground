#include "log.h"

#define LOG_TCP_FATAL
#define LOG_TCP_ERROR
#define LOG_TCP_WARN
#define LOG_TCP_INFO
#define LOG_TCP_DEBUG

#if (LOG_MODULE & LOG_MODULE_TCP)
#if (LOG_LEVEL >= LOG_LEVEL_FATAL)
#undef LOG_TCP_FATAL
#define LOG_TCP_FATAL(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_TCP, LOG_LEVEL_FATAL, fmt, ##__VA_ARGS__)
#endif

#if (LOG_LEVEL >= LOG_LEVEL_ERROR)
#undef LOG_TCP_ERROR
#define LOG_TCP_ERROR(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_TCP, LOG_LEVEL_ERROR, fmt, ##__VA_ARGS__)
#endif

#if (LOG_LEVEL >= LOG_LEVEL_WARN)
#undef LOG_TCP_WARN
#define LOG_TCP_WARN(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_TCP, LOG_LEVEL_WARN, fmt, ##__VA_ARGS__)
#endif

#if (LOG_LEVEL >= LOG_LEVEL_INFO)
#undef LOG_TCP_INFO
#define LOG_TCP_INFO(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_TCP, LOG_LEVEL_INFO, fmt, ##__VA_ARGS__)
#endif

#if (LOG_LEVEL >= LOG_LEVEL_DEBUG)
#undef LOG_TCP_DEBUG
#define LOG_TCP_DEBUG(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_TCP, LOG_LEVEL_DEBUG, fmt, ##__VA_ARGS__)
#endif
#endif
