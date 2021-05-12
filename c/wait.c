#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>
#include <sys/types.h>
#include <unistd.h>
#include <wait.h>
#include <signal.h>

void sig_handler(int sig, siginfo_t *info, void *ucontext) {
    fprintf(stdout, "signo: %d\n", sig);
}

int main(int argc, char *argv[]) {
    switch (fork()) {
        case -1:
            fprintf(stderr, "%s:%d %d[%s]\n", __FILE__, __LINE__, errno, strerror(errno));
            exit(1);
        case 0:
            //sleep(20);
            sleep(10);
            exit(0);
        default: {
            //sleep(10);

            struct sigaction act, oldact;
            act.sa_flags = SA_SIGINFO;
            act.sa_sigaction = sig_handler;
            if (sigaction(SIGCHLD, &act, &oldact) == -1) {
                fprintf(stderr, "%s:%d %d[%s]\n", __FILE__, __LINE__, errno, strerror(errno));
                exit(1);
            }
            if (sigaction(SIGTERM, &act, &oldact) == -1) {
                fprintf(stderr, "%s:%d %d[%s]\n", __FILE__, __LINE__, errno, strerror(errno));
                exit(1);
            }

            int wstatus;
            pid_t pid = wait(&wstatus);
            if (pid == -1) {
                fprintf(stderr, "%s:%d %d[%s]\n", __FILE__, __LINE__, errno, strerror(errno));
                exit(1);
            }
            fprintf(stdout, "child pid: %d\n", pid);
            fprintf(stdout, "wstatus: %d\n", wstatus);
            fprintf(stdout, "exited: %s\n", WIFEXITED(wstatus) ? "TRUE" : "FALSE");
            fprintf(stdout, "exited status: %d\n", WEXITSTATUS(wstatus));
            fprintf(stdout, "signaled: %s\n", WIFSIGNALED(wstatus) ? "TRUE" : "FALSE");
            fprintf(stdout, "signal: %d\n", WTERMSIG(wstatus));

            int left = sleep(20);
            fprintf(stdout, "sleep left: %d\n", left);
            exit(0);
        }
    }
}
