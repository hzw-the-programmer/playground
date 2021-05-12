#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include <errno.h>
#include <unistd.h>
#include <sys/timerfd.h>
#include <signal.h>
#include <time.h>

#include "utils.h"

void sigint(int sig, siginfo_t *info, void *ucontext) {
    fprintf(stderr, "%s:%d: sigint\n", __FILE__, __LINE__);
}

int main(int argc, char *argv[]) {
    struct sigaction act, oldact;
    int tfd;
    int rlen;

    /*
    act.sa_flags = SA_SIGINFO;
    act.sa_sigaction = sigint;
    if (sigaction(SIGINT, &act, &oldact) == -1) {
        die(__FILE__, __LINE__, errno);
    }
    */

    struct timespec res;
    struct timespec t;

    printf("CLOCK_REALTIME:\n");
    if (clock_getres(CLOCK_REALTIME, &res) == -1) {
        die(__FILE__, __LINE__, errno);
    }
    printf("  resolution(%ld, %ld)\n", res.tv_sec, res.tv_nsec);
    if (clock_gettime(CLOCK_REALTIME, &t) == -1) {
        die(__FILE__, __LINE__, errno);
    }
    printf("  time(%ld, %ld)\n", t.tv_sec, t.tv_nsec);

    printf("CLOCK_MONOTONIC:\n");
    if (clock_getres(CLOCK_MONOTONIC, &res) == -1) {
        die(__FILE__, __LINE__, errno);
    }
    printf("  resolution(%ld, %ld)\n", res.tv_sec, res.tv_nsec);
    if (clock_gettime(CLOCK_MONOTONIC, &t) == -1) {
        die(__FILE__, __LINE__, errno);
    }
    printf("  time(%ld, %ld)\n", t.tv_sec, t.tv_nsec);

    tfd = timerfd_create(CLOCK_REALTIME, 0);
    //tfd = timerfd_create(CLOCK_REALTIME, TFD_NONBLOCK);
    if (tfd == -1) {
        die(__FILE__, __LINE__, errno);
    }

    struct itimerspec new_value;
    struct itimerspec old_value;
    new_value.it_value.tv_sec = 1;
    new_value.it_value.tv_nsec = 0;
    new_value.it_interval.tv_sec = 2;
    new_value.it_interval.tv_nsec = 0;
    if (timerfd_settime(tfd, 0, &new_value, &old_value) == -1) {
        die(__FILE__, __LINE__, errno);
    }

    while (1) {
        uint64_t expiration_count;

        if (clock_gettime(CLOCK_REALTIME, &t) == -1) {
            die(__FILE__, __LINE__, errno);
        }
        printf("  time(%ld, %ld)\n", t.tv_sec, t.tv_nsec);

        rlen = read(tfd, &expiration_count, sizeof(expiration_count));
        if (rlen == -1) {
            printerr(__FILE__, __LINE__, errno);
            if (errno == EINTR/*4*/ ||
                errno == EAGAIN/*11*/) {
                continue;
            }
            exit(1);
        }
    }
}

//gcc timerfd.c utils.c
