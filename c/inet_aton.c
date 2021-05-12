#include <stdio.h>
#include <stdlib.h>
#include <arpa/inet.h>

int main(int argc, char *argv[]) {
    struct in_addr sin_addr;
    int res;

    if (argc != 2) {
        fprintf(stderr, "Usage: %s ip\n", argv[0]);
        exit(EXIT_FAILURE);
    }

    res = inet_aton(argv[1], &sin_addr);
    if (res != 1) {
        fprintf(stderr, "inet_aton: error\n");
        exit(EXIT_FAILURE);
    }

    fprintf(stdout, "0x%x\n", sin_addr.s_addr);
}
