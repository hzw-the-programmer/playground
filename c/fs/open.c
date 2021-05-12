#include <stdlib.h>
#include <stdio.h>
#include <errno.h>
#include <string.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>

int main(int argc, char *argv[]) {
#if 1
    //test with:
    //1. file not exit
    //2. echo abcd > opentest then execute.
#if 0
    int fd = open("./opentest", O_WRONLY);
#elif 0
    //access modes, file status flag
    int fd = open("./opentest", O_WRONLY | O_APPEND);
#else
    //access modes, file status flag, file creation flag
#if 0
    mode_t mode = umask(0);
    printf("umask=%x\n", mode);
#endif
    int fd = open("./opentest", O_WRONLY | O_APPEND | O_CREAT, 0777);
#endif
    if (fd == -1) {
        //perror("open");
        fprintf(stderr, "%s:%d %s[%d]\n", __FILE__, __LINE__, strerror(errno), errno); //ENOENT
        exit(EXIT_FAILURE);
    }
    char *buf = "AB\n";
    ssize_t len = write(fd, buf, strlen(buf));
    printf("write len=%ld\n", len);
    close(fd);
#endif
}
