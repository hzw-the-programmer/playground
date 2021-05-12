#include <stdio.h>
#include <unistd.h>
#include <signal.h>
#include <stdlib.h>
#include <stdarg.h>

#define PRINTF(format, ...) do {printf(format, ##__VA_ARGS__);fflush(stdout);} while (0);

void sighandler(int signum, siginfo_t *siginfo, void *ucontext)
{
    static int _count = 0;
    int count = _count++;

    PRINTF("%d: sleep in sighandler: %d\n", count, signum);

    int left = sleep(10);
    if (left == 0) {
        PRINTF("%d: sleep finish\n", count);
    } else {
        PRINTF("%d: sleep interupted, left %d\n", count, left);
    }
}

int main(int argc, char* argv[])
{
    struct sigaction act, oldact;

    act.sa_sigaction = sighandler;
    //act.sa_flags = SA_NODEFER;
    sigemptyset(&act.sa_mask);
    sigaddset(&act.sa_mask, SIGTERM);


    sigaction(SIGINT, &act, &oldact); //ctrl-c
    sigaction(SIGTERM, &act, &oldact); //kill
    sigaction(SIGHUP, &act, &oldact); //terminal exit
    sigaction(SIGSTOP, &act, &oldact); //ctrl-z, can not caught
    sigaction(SIGCONT, &act, &oldact); //fg
    sigaction(SIGKILL, &act, &oldact); //kill -9, can not caught
    sigaction(SIGQUIT, &act, &oldact); //left-ctrl-\

    while (1) {
        int left = sleep(10);
        if (left == 0) {
            PRINTF("sleep finish\n");
        } else {
            PRINTF("sleep interupted, left %d\n", left);
        }
    }
}
