#include "log.h"

#define LOG_UI_FATAL
#define LOG_UI_ERROR
#define LOG_UI_WARN
#define LOG_UI_INFO
#define LOG_UI_DEBUG

#if (LOG_MODULE & LOG_MODULE_UI)
#if (LOG_LEVEL >= LOG_LEVEL_FATAL)
#undef LOG_UI_FATAL
#define LOG_UI_FATAL(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_UI, LOG_LEVEL_FATAL, fmt, ##__VA_ARGS__)
#endif

#if (LOG_LEVEL >= LOG_LEVEL_ERROR)
#undef LOG_UI_ERROR
#define LOG_UI_ERROR(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_UI, LOG_LEVEL_ERROR, fmt, ##__VA_ARGS__)
#endif

#if (LOG_LEVEL >= LOG_LEVEL_WARN)
#undef LOG_UI_WARN
#define LOG_UI_WARN(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_UI, LOG_LEVEL_WARN, fmt, ##__VA_ARGS__)
#endif

#if (LOG_LEVEL >= LOG_LEVEL_INFO)
#undef LOG_UI_INFO
#define LOG_UI_INFO(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_UI, LOG_LEVEL_INFO, fmt, ##__VA_ARGS__)
#endif

#if (LOG_LEVEL >= LOG_LEVEL_DEBUG)
#undef LOG_UI_DEBUG
#define LOG_UI_DEBUG(fmt, ...) log(__FILE__, __LINE__, LOG_MODULE_UI, LOG_LEVEL_DEBUG, fmt, ##__VA_ARGS__)
#endif
#endif
