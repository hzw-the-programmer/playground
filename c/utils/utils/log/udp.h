#include "log.h"

#if (LOG_MODULE & LOG_MODULE_UDP) && (LOG_LEVEL >= LOG_LEVEL_FATAL)
#define LOG_UDP_FATAL(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_UDP, LOG_LEVEL_FATAL, fmt, ##__VA_ARGS__)
#else
#define LOG_UDP_FATAL
#endif

#if (LOG_MODULE & LOG_MODULE_UDP) && (LOG_LEVEL >= LOG_LEVEL_ERROR)
#define LOG_UDP_ERROR(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_UDP, LOG_LEVEL_ERROR, fmt, ##__VA_ARGS__)
#else
#define LOG_UDP_ERROR
#endif

#if (LOG_MODULE & LOG_MODULE_UDP) && (LOG_LEVEL >= LOG_LEVEL_WARN)
#define LOG_UDP_WARN(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_UDP, LOG_LEVEL_WARN, fmt, ##__VA_ARGS__)
#else
#define LOG_UDP_WARN
#endif

#if (LOG_MODULE & LOG_MODULE_UDP) && (LOG_LEVEL >= LOG_LEVEL_INFO)
#define LOG_UDP_INFO(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_UDP, LOG_LEVEL_INFO, fmt, ##__VA_ARGS__)
#else
#define LOG_UDP_INFO
#endif

#if (LOG_MODULE & LOG_MODULE_UDP) && (LOG_LEVEL >= LOG_LEVEL_DEBUG)
#define LOG_UDP_DEBUG(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_UDP, LOG_LEVEL_DEBUG, fmt, ##__VA_ARGS__)
#else
#define LOG_UDP_DEBUG
#endif