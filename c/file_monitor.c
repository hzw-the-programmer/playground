#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <errno.h>
#include <stdio.h>
#include <unistd.h>
#include <sys/epoll.h>

#include "utils.h"

#define EPOLL_SIZE 1024

int main(int argc, char *argv[]) {
    char pathname_ne[] = "./opentest";
    int fd, efd;
    struct epoll_event event;

    printf("STDIN_FILENO=%d\n", STDIN_FILENO);
    printf("STDOUT_FILENO=%d\n", STDOUT_FILENO);
    printf("STDERR_FILENO=%d\n", STDERR_FILENO);

    mode_t mode = umask(S_IXGRP | S_IWOTH | S_IXOTH);
    printf("default umask=%x\n", mode);

    fd = open(pathname_ne, O_WRONLY);
    if (fd == -1) {
        if (errno == ENOENT/*2*/) {
            printerr(__FILE__, __LINE__, errno);
        } else if (errno == EACCES/*13*/) {
            printerr(__FILE__, __LINE__, errno);
        } else {
            die(__FILE__, __LINE__, errno);
        }
    }
    if (close(fd) == -1) {
        printerr(__FILE__, __LINE__, errno);
    }

    //fd = open(pathname_ne, O_WRONLY | O_CREAT);
    fd = open(pathname_ne, O_WRONLY | O_CREAT, S_IRWXU | S_IRWXG | S_IRWXO);
    if (fd == -1) {
        die(__FILE__, __LINE__, errno);
    }
    //close(fd);
    printf("%d\n", fd);

    efd = epoll_create(EPOLL_SIZE);
    if (efd == -1) {
        die(__FILE__, __LINE__, errno);
    }

    event.events = EPOLLIN;
    event.data.fd = fd;
    if (epoll_ctl(efd, EPOLL_CTL_ADD, fd, &event) == -1) {
        if (errno == EPERM/*1*/) {
            printf("fd refers to a regular file or a directory.\n");
        }
        die(__FILE__, __LINE__, errno);
    }

    while(1) {
        struct epoll_event events[EPOLL_SIZE];
        int num = epoll_wait(efd, events, EPOLL_SIZE, -1);
        for (int i = 0; i < num; i++) {
            if (fd == events[i].data.fd) {
                if (EPOLLIN & events[i].events) {
                    printf("if not read it will execute repeatly.\n");
                }
            }
        }
    }
}

//gcc file_monitor.c utils.c
