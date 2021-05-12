#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <sys/epoll.h>
#include <fcntl.h>

#define LISTEN_BACKLOG 50
#define FDNUM 1024

#define FD_DATA_BUF_LEN 100
struct fd_data {
    int fd;
    int write;
    int read;
    int len;
    uint8_t buf[FD_DATA_BUF_LEN];
};

int main(int argc, char *argv[]) {
    int efd, sfd, cfd, fdnum;
    struct epoll_event event, events[FDNUM];
    struct sockaddr_in saddr, caddr;
    socklen_t caddr_len = sizeof(caddr);
    struct fd_data *fdd;
    int flag;
    int len;

    efd = epoll_create(FDNUM);
    if (efd == -1) {
        fprintf(stderr, "%s\n", strerror(errno));
        exit(EXIT_FAILURE);
    }

    sfd = socket(AF_INET, SOCK_STREAM | SOCK_NONBLOCK, 0);
    if (sfd == -1) {
        fprintf(stderr, "%s\n", strerror(errno));
        exit(EXIT_FAILURE);
    }
    /*
    flag = fcntl(sfd, F_GETFL);
    if (flag & O_NONBLOCK) {
        fprintf(stdout, "%s\n", "O_NONBLOCK");
    } else {
        fprintf(stdout, "%s\n", "NO O_NONBLOCK");
    }
    */

    saddr.sin_family = AF_INET;
    saddr.sin_addr.s_addr = htonl(INADDR_ANY);
    saddr.sin_port = htons(8888);
    if (bind(sfd, (struct sockaddr*)&saddr, sizeof(saddr)) == -1) {
        fprintf(stderr, "bind: %s\n", strerror(errno));
        exit(EXIT_FAILURE);
    }

    if (listen(sfd, LISTEN_BACKLOG) == -1) {
        fprintf(stderr, "%s\n", strerror(errno));
        exit(EXIT_FAILURE);
    }

    event.events = EPOLLIN;

    event.data.ptr = malloc(sizeof(struct fd_data));
    if (event.data.ptr == NULL) {
        fprintf(stderr, "%s\n", strerror(errno));
        exit(EXIT_FAILURE);
    }
    // fprintf(stdout, "%p\n", event.data.ptr);
    memset(event.data.ptr, 0, sizeof(struct fd_data));

    fdd = event.data.ptr;
    fdd->fd = sfd;
    fdd->len = FD_DATA_BUF_LEN;

    if (epoll_ctl(efd, EPOLL_CTL_ADD, sfd, &event) == -1) {
        fprintf(stderr, "%s\n", strerror(errno));
        exit(EXIT_FAILURE);
    }

    while (1) {
        fdnum = epoll_wait(efd, events, FDNUM, -1);
        if (fdnum == -1) {
            fprintf(stderr, "%s\n", strerror(errno));
            exit(EXIT_FAILURE);
        }
        for (int i = 0; i< fdnum; i++) {
            fdd = events[i].data.ptr;

            if (sfd == fdd->fd) {
                // fprintf(stdout, "%p\n", fdd);
                cfd = accept(sfd, (struct sockaddr*)&caddr, &caddr_len);
                fcntl(cfd, F_SETFL, O_NONBLOCK);
                /*
                fprintf(stdout, "%x\n", O_NONBLOCK);
                flag = fcntl(cfd, F_GETFL);
                fprintf(stdout, "%x\n", flag);
                if (flag & O_NONBLOCK) {
                    fprintf(stdout, "%s\n", "O_NONBLOCK");
                } else {
                    fprintf(stdout, "%s\n", "NO O_NONBLOCK");
                }
                */
                fprintf(stdout, "%s:%d\n", inet_ntoa(caddr.sin_addr), ntohs(caddr.sin_port));

                event.events = EPOLLIN | EPOLLOUT;

                event.data.ptr = malloc(sizeof(struct fd_data));
                if (event.data.ptr == NULL) {
                    fprintf(stderr, "%s\n", strerror(errno));
                    exit(EXIT_FAILURE);
                }
                // fprintf(stdout, "%p\n", event.data.ptr);
                memset(event.data.ptr, 0, sizeof(struct fd_data));

                fdd = event.data.ptr;
                fdd->fd = cfd;
                fdd->len = FD_DATA_BUF_LEN;

                if (epoll_ctl(efd, EPOLL_CTL_ADD, cfd, &event) == -1) {
                    fprintf(stderr, "%s\n", strerror(errno));
                    exit(EXIT_FAILURE);
                }
            } else {
                if ((events[i].events & EPOLLIN) && (events[i].events & EPOLLOUT)) {
                    fprintf(stdout, "EPOLLIN && EPOLLOUT\n");
                }

                if (events[i].events & EPOLLIN) {
                    if (fdd->write >= fdd->read) {
                        if (fdd->read == 0) {
                            len = FD_DATA_BUF_LEN - fdd->write - 1;
                        } else {
                            len = FD_DATA_BUF_LEN - fdd->write;
                        }
                    } else {
                        len = fdd->read - fdd->write - 1;
                    }
                    fprintf(stdout, "buffer len: %d\n", len);

                    if (len != 0) {
                        len = recv(fdd->fd, fdd->buf + fdd->write, len, 0);
                        
                        if (len == -1) {
                            fprintf(stderr, "recv: %s:%d\n", strerror(errno), errno);
                            if (errno == ECONNRESET) { // recv: Connection reset by peer
                                fprintf(stderr, "continue\n");
                                // let it trigger EPOLLIN and len == 0, end-of-file.
                                // if (epoll_ctl(efd, EPOLL_CTL_DEL, events[i].data.fd, NULL) == -1) {
                                //     fprintf(stderr, "%s\n", strerror(errno));
                                //     exit(EXIT_FAILURE);
                                // }
                            } else {
                                exit(EXIT_FAILURE);
                            }
                        } else if (len == 0) {
                            fprintf(stdout, "%s\n", "end-of-file");
                            // end of file need del fd, otherwise EPOLLIN triggers again and again and len == 0.
                            //fprintf(stderr, "%d, %d, %d\n", fdd->fd, fdd->write, fdd->read);
                            if (epoll_ctl(efd, EPOLL_CTL_DEL, fdd->fd, NULL) == -1) {
                                fprintf(stderr, "epoll_ctl: %s:%d\n", strerror(errno), errno);
                                exit(EXIT_FAILURE);
                            }
                            //fprintf(stderr, "%d, %d, %d\n", fdd->fd, fdd->write, fdd->read);
                            free(fdd);
                            //fprintf(stderr, "%d, %d, %d\n", fdd->fd, fdd->write, fdd->read);
                            fdd = NULL;
                        } else {
                            fprintf(stdout, "recv len: %d\n", len);
                            fdd->write = (fdd->write + len) % FD_DATA_BUF_LEN;
                        }
                    } else {
                        fprintf(stdout, "buffer full.\n");
                        // exit(0);
                    }
                }
                
                if (events[i].events & EPOLLOUT && fdd != NULL) {
                    if (fdd->read > fdd->write) {
                        len = FD_DATA_BUF_LEN - fdd->read;
                    } else {
                        len = fdd->write - fdd->read;
                    }

                    if (len != 0) {
                        //fprintf(stderr, "%d, %d, %d\n", fdd->fd, fdd->write, fdd->read);
                        len = send(fdd->fd, fdd->buf + fdd->read, len, 0);
                        if (len == -1) {
                            fprintf(stderr, "send: %s:%d\n", strerror(errno), errno);
                            exit(EXIT_FAILURE);
                        }
                        fprintf(stdout, "send len: %d\n", len);
                        fdd->read = (fdd->read + len) % FD_DATA_BUF_LEN;
                    } else {
                        // fprintf(stdout, "buffer empty.\n");
                    }
                }
            }
        }
    }
}

// GET / HTTP/1.1
// Host: localhost:8888
// User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:58.0) Gecko/20100101 Firefox/58.0
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
// Accept-Language: zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2
// Accept-Encoding: gzip, deflate
// Sec-WebSocket-Version: 13
// Origin: null
// Sec-WebSocket-Extensions: permessage-deflate
// Sec-WebSocket-Key: blfXtud+H8awzQKZ9hYm0g==
// Connection: keep-alive, Upgrade
// Pragma: no-cache
// Cache-Control: no-cache
// Upgrade: websocket
//
//
