#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <fcntl.h>
#include <errno.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <signal.h>
#include <sys/epoll.h>

#include "buffer.h"

#define MAX_EPOLL_FD 1024

int die(char *file, int line, int err) {
    fprintf(stderr, "%s:%d: %d(%s)\n", file, line, err, strerror(err));
    exit(EXIT_FAILURE);
}

void sigint(int sig, siginfo_t *info, void *ucontext) {
    fprintf(stderr, "sigint: sig=%d, pid=%d, spid=%d\n", sig, getpid(), info->si_pid);
}

int main(int argc, char *argv[]) {
    fprintf(stdout, "pid=%d\n", getpid());

    struct sigaction act, oldact;
    act.sa_flags = SA_SIGINFO;
    act.sa_sigaction = sigint;
    if (sigaction(SIGINT, &act, &oldact) == -1) {
        die(__FILE__, __LINE__, errno);
    }

    int efd = epoll_create(MAX_EPOLL_FD);
    if (efd == -1) {
        die(__FILE__, __LINE__, errno);
    }

    // create client socket
    int csfd = socket(AF_INET, SOCK_STREAM, 0);
    if (csfd == -1) {
        die(__FILE__, __LINE__, errno);
    }

    // optionally bind client socket
    struct sockaddr_in laddr;
    laddr.sin_family = AF_INET;
    laddr.sin_addr.s_addr = INADDR_ANY;
    laddr.sin_port = htons(4321);
    if (bind(csfd, (struct sockaddr*)&laddr, sizeof(laddr)) == -1) {
        die(__FILE__, __LINE__, errno);
    }

    // connect client socket to server
    /*
    struct sockaddr_in raddr;
    raddr.sin_family = AF_INET;
    if (inet_aton("127.0.0.1", &raddr.sin_addr) == 0) {
        die(__FILE__, __LINE__, errno);
    }
    raddr.sin_port = htons(8888);
    if (connect(csfd, (struct sockaddr*)&raddr, sizeof(raddr)) == -1) {
        die(__FILE__, __LINE__, errno);
    }
    */

    // add client socket to epoll
    struct epoll_event event;
    /*
    event.events = EPOLLIN | EPOLLOUT;
    event.data.fd = csfd;
    if (epoll_ctl(efd, EPOLL_CTL_ADD, csfd, &event) == -1) {
        die(__FILE__, __LINE__, errno);
    }
    */

    buffer_t *buf = createBuffer(5);

    fcntl(STDIN_FILENO, F_SETFL, O_NONBLOCK);
    event.events = EPOLLIN;
    event.data.fd = STDIN_FILENO;
    if (epoll_ctl(efd, EPOLL_CTL_ADD, STDIN_FILENO, &event) == -1) {
        die(__FILE__, __LINE__, errno);
    }

    event.events = EPOLLOUT;
    event.data.fd = STDOUT_FILENO;
    if (epoll_ctl(efd, EPOLL_CTL_ADD, STDOUT_FILENO, &event) == -1) {
        die(__FILE__, __LINE__, errno);
    }

    while (1) {
        struct epoll_event events[MAX_EPOLL_FD];
        int fdnum = epoll_wait(efd, events, sizeof(events), -1);
        if (fdnum == -1) {
            if (errno == EINTR/*4*/) {
                continue;
            }
            die(__FILE__, __LINE__, errno);
        }

        for (int i = 0; i < fdnum; i++) {
            if (STDIN_FILENO == events[i].data.fd) {
                size_t len;

                while ((len = _getBufferWriteLen(buf)) != 0) {
                    len = read(STDIN_FILENO, getBufferWritePtr(buf), len);

                    if (len == -1) {
                        if (errno == EINTR/*4*/) {
                            continue;
                        } else if (errno == EAGAIN/*11*/) {
                            break;
                        }
                        die(__FILE__, __LINE__, errno);
                    } else if (len == 0) {
                        die(__FILE__, __LINE__, errno);
                    } else {
                        advanceBufferWrite(buf, len);
                    }
                }

            } else if (STDOUT_FILENO == events[i].data.fd) {
                size_t len;
                
                while ((len = _getBufferReadLen(buf)) != 0) {
                    len = write(STDOUT_FILENO, getBufferReadPtr(buf), len);
                    
                    if (len == -1) {
                        die(__FILE__, __LINE__, errno);
                    } else if (len == 0) {
                        die(__FILE__, __LINE__, errno);
                    } else {
                        advanceBufferRead(buf, len);
                    }
                }

            }
        }
    }
}

/*
gcc echo_client.c buffer.c
*/
