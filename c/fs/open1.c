#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>
#include <unistd.h>
//#include <sys/types.h>
//#include <sys/stat.h>
#include <fcntl.h>

#define FN "./test"
#define OC "abcdefg\n"
#define NC "ABC"

void mktest() {
    char *oc = OC;
    size_t oc_len = strlen(oc);
    ssize_t len;
    
    if (access(FN, F_OK) == 0) {
        if (unlink(FN) == -1) {
            fprintf(stderr, "%s:%d: %s[%d]\n",
                __FILE__, __LINE__,
                strerror(errno), errno);
            exit(EXIT_FAILURE);
        }
    }

    int fd = open(FN, O_WRONLY | O_TRUNC | O_CREAT, 0777);
    if (fd == -1) {
        fprintf(stderr, "%s:%d: %s[%d]\n",
            __FILE__, __LINE__,
            strerror(errno), errno);
        exit(EXIT_FAILURE);
    }

    while (oc_len != 0) {
        len = write(fd, oc, oc_len);
        if (len == -1) {
            // The call was interrupted by a signal before any data was written.
            if (errno == EINTR) {
                continue;
            }
            fprintf(stderr, "%s:%d: %s[%d]\n",
                __FILE__, __LINE__,
                strerror(errno), errno);
            exit(EXIT_FAILURE);
        }
        oc += len;
        oc_len -= len;
    }
    
    close(fd);
}

void wronly(int flags) {
    char *nc = NC;
    size_t nc_len = strlen(nc);
    ssize_t len;
    int fd = open(FN, O_WRONLY | flags);
    if (fd == -1) {
        fprintf(stderr, "%s:%d: %s[%d]\n",
            __FILE__, __LINE__,
            strerror(errno), errno);
        exit(EXIT_FAILURE);
    }

    while (nc_len != 0) {
        len = write(fd, nc, nc_len);
        if (len == -1) {
            // The call was interrupted by a signal before any data was written.
            if (errno == EINTR) {
                continue;
            }
            fprintf(stderr, "%s:%d: %s[%d]\n",
                __FILE__, __LINE__,
                strerror(errno), errno);
            exit(EXIT_FAILURE);
        }
        nc += len;
        nc_len -= len;
    }
}

int main(int argc, char *argv[]) {
    mktest();
    wronly(0);
    //wronly(O_TRUNC);
    //wronly(O_APPEND);
}
