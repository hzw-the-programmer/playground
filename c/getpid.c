#include <stdio.h>
#include <pthread.h>
#include <sys/syscall.h>
#include <sys/types.h>
#include <unistd.h>

pid_t gettid() {
    return syscall(SYS_gettid);
}

void *work(void* arg) {
    fprintf(
        stdout,
        "pid: %d, tid: %d, ppid: %d, sid: %d, pgid: %d\n",
        getpid(), gettid(), getppid(), getsid(0), getpgid(0));
}

int main(int argc, char *argv[]) {
    pthread_t thread;

    fork();
    fprintf(
        stdout,
        "pid: %d, tid: %d, ppid: %d, sid: %d, pgid: %d\n",
        getpid(), gettid(), getppid(), getsid(0), getpgid(0));
    pthread_create(&thread, NULL, work, NULL);
    pthread_join(thread, NULL);
}
