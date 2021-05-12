#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/wait.h>

int main(int argc, char* argv[]) {
    char temp[] = "/tmp/hzw_XXXXXX";
    int fd = mkstemp(temp);
    printf("fn=%s\n", temp);
    printf("fd=%d\n", fd);
    int nfd = dup(fd);
    printf("nfd=%d\n", nfd);

    int p = lseek(fd, 123, SEEK_SET);
    printf("p=%d\n", p);
    close(fd);
    p = lseek(fd, 0, SEEK_CUR);
    printf("fd: p=%d\n", p);
    p = lseek(nfd, 0, SEEK_CUR);
    printf("nfd: p=%d\n", p);

    switch (fork()) {
        case 0: {
            int status;
            wait(&status);
            printf("status=%d\n", status);
            p = lseek(fd, 0, SEEK_CUR);
            printf("fd: p=%d\n", p);
            p = lseek(nfd, 0, SEEK_CUR);
            printf("nfd: p=%d\n", p);
            break;
        }
        default:
            printf("c: fd=%d\n", fd);
            printf("c: nfd=%d\n", nfd);
            p = lseek(fd, 0, SEEK_CUR);
            printf("c: fd: p=%d\n", p);
            p = lseek(nfd, 0, SEEK_CUR);
            printf("c: nfd: p=%d\n", p);
            close(nfd);
            break;
    }

    return 2;
}
