#include "log.h"

#if (LOG_MODULE & LOG_MODULE_QUIC) && (LOG_LEVEL >= LOG_LEVEL_FATAL)
#define LOG_QUIC_FATAL(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_QUIC, LOG_LEVEL_FATAL, fmt, ##__VA_ARGS__)
#else
#define LOG_QUIC_FATAL
#endif

#if (LOG_MODULE & LOG_MODULE_QUIC) && (LOG_LEVEL >= LOG_LEVEL_ERROR)
#define LOG_QUIC_ERROR(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_QUIC, LOG_LEVEL_ERROR, fmt, ##__VA_ARGS__)
#else
#define LOG_QUIC_ERROR
#endif

#if (LOG_MODULE & LOG_MODULE_QUIC) && (LOG_LEVEL >= LOG_LEVEL_WARN)
#define LOG_QUIC_WARN(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_QUIC, LOG_LEVEL_WARN, fmt, ##__VA_ARGS__)
#else
#define LOG_QUIC_WARN
#endif

#if (LOG_MODULE & LOG_MODULE_QUIC) && (LOG_LEVEL >= LOG_LEVEL_INFO)
#define LOG_QUIC_INFO(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_QUIC, LOG_LEVEL_INFO, fmt, ##__VA_ARGS__)
#else
#define LOG_QUIC_INFO
#endif

#if (LOG_MODULE & LOG_MODULE_QUIC) && (LOG_LEVEL >= LOG_LEVEL_DEBUG)
#define LOG_QUIC_DEBUG(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_QUIC, LOG_LEVEL_DEBUG, fmt, ##__VA_ARGS__)
#else
#define LOG_QUIC_DEBUG
#endif