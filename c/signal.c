#include <signal.h>
#include <stdio.h>
#include <unistd.h>
#include <sys/time.h>

unsigned int sleeptime = 10;

void handler(int signum, siginfo_t *siginfo, void *ucontext) {
    struct timeval stv;
    struct timeval etv;
    struct timeval rtv;
    char *name;

    switch (signum) {
        case SIGINT:
            name = "SIGINT";
            break;
        case SIGTERM:
            name = "SIGTERM";
            break;
        default:
            name = "UNKNOWN";
            break;
    }

    printf("\n*** in %s ***\n", name);
    printf("\n  signum: %d\n", signum);

    printf("  sleep time: %d\n", sleeptime);
    gettimeofday(&stv, NULL);
    int r = sleep(sleeptime);
    gettimeofday(&etv, NULL);
    timersub(&etv, &stv, &rtv);
    printf("  unsleep time: %d\n", r);
    printf("  sleep execution time: %ld.%ld\n", rtv.tv_sec, rtv.tv_usec);

    printf("\n*** exit %s ***\n", name);
}

void sigint(int signum, siginfo_t *siginfo, void *ucontext) {
    handler(signum, siginfo, ucontext);
}

void sigterm(int signum, siginfo_t *siginfo, void *ucontext) {
    handler(signum, siginfo, ucontext);
}

int main(int argc, char *argv[]) {
    struct sigaction act;
    act.sa_flags = SA_SIGINFO;

    //act.sa_sigaction = sigint;
    act.sa_sigaction = handler;
    sigaction(SIGINT, &act, NULL); // CTRL-C

    //act.sa_sigaction = sigterm;
    sigaction(SIGTERM, &act, NULL); // kill pid

    while (1) {
        struct timeval stv;
        struct timeval etv;
        struct timeval rtv;

        sleep(sleeptime / 2);
        
        printf("\n--- sleep begin ---\n");

        printf("\n  sleep time: %d\n", sleeptime);
        gettimeofday(&stv, NULL);
        int r = sleep(sleeptime);
        gettimeofday(&etv, NULL);
        timersub(&etv, &stv, &rtv);
        printf("  unsleep time: %d\n", r);
        printf("  sleep execution time: %ld.%ld\n", rtv.tv_sec, rtv.tv_usec);

        printf("\n--- sleep end ---\n");
    }
}
