#include "log.h"

#define LOG_UDP_FATAL
#define LOG_UDP_ERROR
#define LOG_UDP_WARN
#define LOG_UDP_INFO
#define LOG_UDP_DEBUG

#if (LOG_MODULE & LOG_MODULE_UDP)
#if (LOG_LEVEL >= LOG_LEVEL_FATAL)
#undef LOG_UDP_FATAL
#define LOG_UDP_FATAL(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_UDP, LOG_LEVEL_FATAL, fmt, ##__VA_ARGS__)
#endif

#if (LOG_LEVEL >= LOG_LEVEL_ERROR)
#undef LOG_UDP_ERROR
#define LOG_UDP_ERROR(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_UDP, LOG_LEVEL_ERROR, fmt, ##__VA_ARGS__)
#endif

#if (LOG_LEVEL >= LOG_LEVEL_WARN)
#undef LOG_UDP_WARN
#define LOG_UDP_WARN(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_UDP, LOG_LEVEL_WARN, fmt, ##__VA_ARGS__)
#endif

#if (LOG_LEVEL >= LOG_LEVEL_INFO)
#undef LOG_UDP_INFO
#define LOG_UDP_INFO(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_UDP, LOG_LEVEL_INFO, fmt, ##__VA_ARGS__)
#endif

#if (LOG_LEVEL >= LOG_LEVEL_DEBUG)
#undef LOG_UDP_DEBUG
#define LOG_UDP_DEBUG(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_UDP, LOG_LEVEL_DEBUG, fmt, ##__VA_ARGS__)
#endif
#endif
