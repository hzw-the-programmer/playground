#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <sys/epoll.h>
#include <unistd.h>

#include "utils.h"
#include "circular_que.h"

#define EPOLL_SIZE 1024

#define NUM_OF_UDP_PKT 10
#define UDP_PKT_DATA_LEN 512

typedef struct udp_pkt {
    struct sockaddr_in addr;
    size_t len;
    uint8_t data[UDP_PKT_DATA_LEN];
} udp_pkt_t;

udp_pkt_t udppkt_in, udppkt_out;
char udppktstr[UDP_PKT_DATA_LEN * 2];


int main(int argc, char *argv[]) {
    int efd, usfd, num, len;
    struct sockaddr_in laddr, raddr;
    socklen_t addrlen = sizeof(laddr);
    struct epoll_event event, events[EPOLL_SIZE];

    circular_que_t *log_que = create_circular_que(NUM_OF_UDP_PKT, sizeof(udp_pkt_t));
    circular_que_t *send_que = create_circular_que(NUM_OF_UDP_PKT, sizeof(udp_pkt_t));

    efd = epoll_create(EPOLL_SIZE);
    if (efd == -1) {
        die(__FILE__, __LINE__, errno);
    }

    usfd = socket(AF_INET, SOCK_DGRAM, 0);
    if (usfd == -1) {
        die(__FILE__, __LINE__, errno);
    }

    laddr.sin_family = AF_INET;
    laddr.sin_addr.s_addr = htonl(INADDR_ANY);
    laddr.sin_port = htons(19268);
    if (bind(usfd, (struct sockaddr*)&laddr, addrlen) == -1) {
        die(__FILE__, __LINE__, errno);
    }

    event.events = EPOLLIN | EPOLLOUT;
    event.data.fd = usfd;
    if (epoll_ctl(efd, EPOLL_CTL_ADD, usfd, &event) == -1) {
        die(__FILE__, __LINE__, errno);
    }

    event.events = EPOLLOUT;
    event.data.fd = STDOUT_FILENO;
    if (epoll_ctl(efd, EPOLL_CTL_ADD, STDOUT_FILENO, &event) == -1) {
        die(__FILE__, __LINE__, errno);
    }

    while (1) {
        num = epoll_wait(efd, events, EPOLL_SIZE, -1);
        if (num == -1) {
            die(__FILE__, __LINE__, errno);
        }
        //printf("%d\n", num);

        for (int i = 0; i < num; i++) {
            if (usfd == events[i].data.fd) {
                if (EPOLLIN & events[i].events) {
                    udppkt_in.len = recvfrom(usfd, udppkt_in.data, UDP_PKT_DATA_LEN, 0,
                        (struct sockaddr*)&udppkt_in.addr, &addrlen);
                    if (circular_que_push(log_que, &udppkt_in) == -1) {
                        die(__FILE__, __LINE__, errno);
                    }
                    if (circular_que_push(send_que, &udppkt_in) == -1) {
                        die(__FILE__, __LINE__, errno);
                    }
                }
                
                if (EPOLLOUT & events[i].events) {
                    //printf("usfd EPOLLOUT\n");
                    if (circular_que_pop(send_que, &udppkt_out) != -1) {
                        sendto(usfd, udppkt_out.data, udppkt_out.len, 0,
                            (struct sockaddr*)&udppkt_out.addr, sizeof(struct sockaddr_in));
                    }
                }
            } else if (STDOUT_FILENO == events[i].data.fd) {
                if (EPOLLOUT & events[i].events) {
                    //printf("STDOUT_FILENO EPOLLOUT\n");
                    //exit(0);
                    if (circular_que_pop(log_que, &udppkt_out) != -1) {
                        len = hexlify(udppkt_out.data, udppkt_out.len, udppktstr, sizeof(udppktstr), 0);
                        udppktstr[len] = '\0';
                        fprintf(stderr, "%s\n", udppktstr);
                    }
                }
            }
        }
    }
}

//gcc hserver.c circular_que.c utils.c -o hserver
