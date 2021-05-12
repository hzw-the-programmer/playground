#include <sys/epoll.h>
#include <sys/timerfd.h>
#include <errno.h>
#include <unistd.h>
#include <stdio.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <stdlib.h>
#include <arpa/inet.h>

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

char udppktdemo[] = "5033001403F661FE0148000000000452120308101E06FE9A";

int main(int argc, char *argv[]) {
    int efd, tfd, usfd, len;
    socklen_t addrlen = sizeof(struct sockaddr_in);
    struct epoll_event event;

    circular_que_t *log_que = create_circular_que(NUM_OF_UDP_PKT, sizeof(udp_pkt_t));
    circular_que_t *send_que = create_circular_que(NUM_OF_UDP_PKT, sizeof(udp_pkt_t));

    if (argc != 3) {
        printf("Usage: %s remote_addr remote_port\n", argv[0]);
        exit(EXIT_FAILURE);
    }

    struct sockaddr_in addr;
    addr.sin_family = AF_INET;
    if (inet_aton(argv[1], &addr.sin_addr) == -1) {
        die(__FILE__, __LINE__, errno);
    }
    addr.sin_port = htons(atoi(argv[2]));

    efd = epoll_create(EPOLL_SIZE);
    if (efd == -1) {
        die(__FILE__, __LINE__, errno);
    }

    usfd = socket(AF_INET, SOCK_DGRAM, 0);
    if (usfd == -1) {
        die(__FILE__, __LINE__, errno);
    }

    event.events = EPOLLIN | EPOLLOUT;
    event.data.fd = usfd;
    if (epoll_ctl(efd, EPOLL_CTL_ADD, usfd, &event) == -1) {
        die(__FILE__, __LINE__, errno);
    }

    tfd = timerfd_create(CLOCK_MONOTONIC, 0);
    if (tfd == -1) {
        die(__FILE__, __LINE__, errno);
    }

    struct itimerspec new_value, old_value;
    new_value.it_value.tv_sec = 1;
    new_value.it_value.tv_nsec = 0;
    new_value.it_interval.tv_sec = 2;
    new_value.it_interval.tv_nsec = 0;
    if (timerfd_settime(tfd, 0, &new_value, &old_value) == -1) {
        die(__FILE__, __LINE__, errno);
    }

    event.events = EPOLLIN;
    event.data.fd = tfd;
    if (epoll_ctl(efd, EPOLL_CTL_ADD, tfd, &event) == -1) {
        die(__FILE__, __LINE__, errno);
    }

    event.events = EPOLLOUT;
    event.data.fd = STDOUT_FILENO;
    if (epoll_ctl(efd, EPOLL_CTL_ADD, STDOUT_FILENO, &event) == -1) {
        die(__FILE__, __LINE__, errno);
    }

    while (1) {
        /*
        struct timespec tp;
        if (clock_gettime(CLOCK_MONOTONIC, &tp) == -1) {
            die(__FILE__, __LINE__, errno);
        }
        printf("time: (%ld, %ld)\n", tp.tv_sec, tp.tv_nsec);
        */

        struct epoll_event events[EPOLL_SIZE];
        int num = epoll_wait(efd, events, EPOLL_SIZE, -1);
        if (num == -1) {
            die(__FILE__, __LINE__, errno);
        }

        for (int i = 0; i < num; i++) {
            if (tfd == events[i].data.fd) {
                uint64_t expiration;
                read(tfd, &expiration, sizeof(expiration));

                udppkt_in.addr = addr;
                udppkt_in.len = unhexlify(udppktdemo, sizeof(udppktdemo), udppkt_in.data, UDP_PKT_DATA_LEN);
                if (circular_que_push(send_que, &udppkt_in) == -1) {
                    die(__FILE__, __LINE__, errno);
                }
            } else if (usfd == events[i].data.fd) {
                if (EPOLLIN & events[i].events) {
                    udppkt_in.len = recvfrom(usfd, udppkt_in.data, UDP_PKT_DATA_LEN, 0,
                        (struct sockaddr*)&udppkt_in.addr, &addrlen);
                    if (circular_que_push(log_que, &udppkt_in) == -1) {
                        die(__FILE__, __LINE__, errno);
                    }
                }

                if (EPOLLOUT & events[i].events) {
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

    if (close(efd) == -1) {
        die(__FILE__, __LINE__, errno);
    }
}

//gcc hclient.c circular_que.c utils.c -o hclient
