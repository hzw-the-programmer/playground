#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>

#define FN "./test"

void wr(int fd) {
    ssize_t len;
    char *c = "abcd";
    size_t clen = strlen(c);
    while (clen != 0) {
        len = write(fd, c, clen);
        if (len == -1) {
            if (errno == EINTR) {
                continue;
            }
            fprintf(stderr, "%s:%d: %s[%d]\n",
                __FILE__, __LINE__, strerror(errno), errno);
            exit(EXIT_FAILURE);
        }
        c += len;
        clen -= len;
    }
}

int main(int argc, char *argv[]) {
    ssize_t len;
    if (access(FN, F_OK) == 0) {
        if (unlink(FN) == -1) {
            fprintf(stderr, "%s:%d: %s[%d]\n",
                __FILE__, __LINE__, strerror(errno), errno);
            exit(EXIT_FAILURE);
        }
    }

    int fd = open(FN, O_RDWR | O_CREAT, 0777);
    if (fd == -1) {
        fprintf(stderr, "%s:%d: %s[%d]\n",
            __FILE__, __LINE__, strerror(errno), errno);
        exit(EXIT_FAILURE);
    }

    wr(fd);

    lseek(fd, 0, SEEK_SET);
    char buf[5] = {0};
    len = read(fd, buf, 2);
    printf("%s\n", buf);

    wr(fd);
}