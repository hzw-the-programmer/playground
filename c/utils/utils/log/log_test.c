#include "log.h"

void log_test() {
    LOG_FATAL(LOG_MODULE_UI, "code=%d, content_length=%d", 200, 1024);
    LOG_ERROR(LOG_MODULE_TCP, "code=%d, content_length=%d", 200, 1024);
    LOG_WARN(LOG_MODULE_HTTP, "code=%d, content_length=%d", 200, 1024);
    LOG_INFO(LOG_MODULE_UDP, "code=%d, content_length=%d", 200, 1024);
    LOG_DEBUG(LOG_MODULE_QUIC, "code=%d, content_length=%d", 200, 1024);
}
