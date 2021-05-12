#include <stdio.h>
#include <time.h>
#include <sys/timerfd.h>
#include <stdint.h>
#include <unistd.h>

int main() {
    int fd = timerfd_create(CLOCK_REALTIME, 0);

    struct itimerspec new;
    new.it_value.tv_nsec = 0;
    new.it_value.tv_sec = 1;
    new.it_interval.tv_nsec = 0;
    new.it_interval.tv_sec = 1;
    timerfd_settime(fd, 0, &new, NULL);

    uint64_t exp;
    while (1) {
        read(fd, &exp, sizeof(exp));

        char buf[100];
        time_t t = time(NULL);
        ctime_r(&t, buf);
        printf("%s\n", buf);
    }
}
