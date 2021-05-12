#include <netinet/in.h>

#define UDP_PKT_DATA_LEN 512

typedef struct udp_pkt {
    struct sockaddr_in addr;
    size_t len;
    uint8_t data[UDP_PKT_DATA_LEN];
} udp_pkt_t;
