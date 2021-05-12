#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>
#include <sys/wait.h>
#include <signal.h>
#include <errno.h>

#define WRITE

void sig_handler(int signum, siginfo_t *siginfo, void *ucontext) {
    fprintf(stdout, "signum=%d\n", signum);
}

int main(int argc, char *argv[]) {
    int pipefd[2], len, wstatus;
    pid_t cid;
    char buf;

    if (argc != 2) {
        fprintf(stderr, "Usage: %s <string>\n", argv[0]);
        exit(EXIT_FAILURE); //1
    }

    if (pipe(pipefd) == -1) {
        perror("pipe");
        exit(EXIT_FAILURE);
    }

    cid = fork();
    if (cid == -1) {
        perror("fork");
        exit(EXIT_FAILURE);
    }

    if (cid == 0) {
        close(pipefd[1]); //write side

#if !defined(WRITE)
        sleep(1);
        fprintf(stdout, "begin read\n");
        while ((len = read(pipefd[0], &buf, 1)) > 0) {
            write(STDOUT_FILENO, &buf, 1);
        }
        write(STDOUT_FILENO, "\n", 1);

        fprintf(stdout, "last read len=%d\n", len);
#endif
        close(pipefd[0]);

        _exit(EXIT_SUCCESS);
    } else {
        close(pipefd[0]); //read side

#if defined(WRITE)
        struct sigaction act, oldact;
        act.sa_flags = SA_SIGINFO;
        act.sa_sigaction = sig_handler;
        if (sigaction(SIGPIPE, &act, &oldact) == -1) {
            perror("sigaction");
            exit(EXIT_FAILURE);
        }

        sleep(1);
        fprintf(stdout, "before write\n");
#endif

        len = write(pipefd[1], argv[1], strlen(argv[1]));
        fprintf(stdout, "write len=%d\n", len);
        if (len == -1) {
            fprintf(stdout, "%s[%d]\n", strerror(errno), errno);
        }
        close(pipefd[1]); //if not close, read will block.

        fprintf(stdout, "wait\n");
        cid = wait(&wstatus);
        fprintf(stdout, "cid=%d, exit=%d, status=%d\n", cid, WIFEXITED(wstatus), WEXITSTATUS(wstatus));
        exit(EXIT_SUCCESS);
    }
}
