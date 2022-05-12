#include "udp.h"

void udp_test() {
    LOG_UDP_FATAL("hi %s, %d m", "hzw", 1);
    LOG_UDP_ERROR("haha");
    LOG_UDP_WARN("haha");
    LOG_UDP_INFO("haha");
    LOG_UDP_DEBUG("haha");
}