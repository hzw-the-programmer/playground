#include "log.h"

#define LOG_QUIC_FATAL
#define LOG_QUIC_ERROR
#define LOG_QUIC_WARN
#define LOG_QUIC_INFO
#define LOG_QUIC_DEBUG

#if (LOG_MODULE & LOG_MODULE_QUIC)
#if (LOG_LEVEL >= LOG_LEVEL_FATAL)
#undef LOG_QUIC_FATAL
#define LOG_QUIC_FATAL(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_QUIC, LOG_LEVEL_FATAL, fmt, ##__VA_ARGS__)
#endif

#if (LOG_LEVEL >= LOG_LEVEL_ERROR)
#undef LOG_QUIC_ERROR
#define LOG_QUIC_ERROR(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_QUIC, LOG_LEVEL_ERROR, fmt, ##__VA_ARGS__)
#endif

#if (LOG_LEVEL >= LOG_LEVEL_WARN)
#undef LOG_QUIC_WARN
#define LOG_QUIC_WARN(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_QUIC, LOG_LEVEL_WARN, fmt, ##__VA_ARGS__)
#endif

#if (LOG_LEVEL >= LOG_LEVEL_INFO)
#undef LOG_QUIC_INFO
#define LOG_QUIC_INFO(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_QUIC, LOG_LEVEL_INFO, fmt, ##__VA_ARGS__)
#endif

#if (LOG_LEVEL >= LOG_LEVEL_DEBUG)
#undef LOG_QUIC_DEBUG
#define LOG_QUIC_DEBUG(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_QUIC, LOG_LEVEL_DEBUG, fmt, ##__VA_ARGS__)
#endif
#endif
