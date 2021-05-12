#include <stdio.h>
#include <signal.h>
#include <unistd.h>

int main(int argc, char *argv[]) {
    sigset_t set, oldset;
    int sig, ret;

    sigemptyset(&set);
    sigaddset(&set, SIGINT);
    sigaddset(&set, SIGTERM);
    fprintf(stdout, "SIGINT: %d\n", sigismember(&set, SIGINT));
    fprintf(stdout, "SIGTERM: %d\n", sigismember(&set, SIGTERM));
    fprintf(stdout, "SIGUSR1: %d\n", sigismember(&set, SIGUSR1));

    sigprocmask(SIG_SETMASK, &set, &oldset);

    ret = sleep(20);
    fprintf(stdout, "%d\n", ret);

    sigpending(&set);
    fprintf(stdout, "SIGINT: %d\n", sigismember(&set, SIGINT));
    fprintf(stdout, "SIGTERM: %d\n", sigismember(&set, SIGTERM));
    fprintf(stdout, "SIGUSR1: %d\n", sigismember(&set, SIGUSR1));

    sigemptyset(&set);
    sigaddset(&set, SIGINT);
    sigaddset(&set, SIGTERM);
    sigwait(&set, &sig);
    fprintf(stdout, "sig: %d\n", sig);

    sigpending(&set);
    fprintf(stdout, "SIGINT: %d\n", sigismember(&set, SIGINT));
    fprintf(stdout, "SIGTERM: %d\n", sigismember(&set, SIGTERM));
    fprintf(stdout, "SIGUSR1: %d\n", sigismember(&set, SIGUSR1));

    sigemptyset(&set);
    sigaddset(&set, SIGINT);
    sigaddset(&set, SIGTERM);
    sigwait(&set, &sig);
    fprintf(stdout, "sig: %d\n", sig);

    sigpending(&set);
    fprintf(stdout, "SIGINT: %d\n", sigismember(&set, SIGINT));
    fprintf(stdout, "SIGTERM: %d\n", sigismember(&set, SIGTERM));
    fprintf(stdout, "SIGUSR1: %d\n", sigismember(&set, SIGUSR1));
}
