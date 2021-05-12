#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>

#include <arpa/inet.h>

#define PORT 19267
#define BUFFLEN 1024

int main() {
    int sock;

    if ((sock = socket(AF_INET, SOCK_DGRAM, 0)) == -1) {
        fprintf(stderr, "%s:%d: %d(%s)", __FILE__, __LINE__, errno, strerror(errno));
        exit(1);
    }

    struct sockaddr_in addr_me;

    bzero(&addr_me, sizeof(addr_me));

    addr_me.sin_family = AF_INET;
    addr_me.sin_addr.s_addr = htonl(INADDR_ANY);
    addr_me.sin_port = htons(PORT);

    if (bind(sock, (struct sockaddr*)&addr_me, sizeof(addr_me)) == -1) {
        fprintf(stderr, "%s:%d: %d(%s)", __FILE__, __LINE__, errno, strerror(errno));
        exit(1);
    }

    ssize_t len;
    char buff[BUFFLEN];
    struct sockaddr_in addr_other;
    socklen_t addr_len = sizeof(addr_other);
    while (1) {
        if ((len = recvfrom(sock, buff, BUFFLEN, 0, (struct sockaddr*)&addr_other, &addr_len)) == -1) {
            fprintf(stderr, "%s:%d: %d(%s)", __FILE__, __LINE__, errno, strerror(errno));
            exit(1);
        }

        for (int i = 0; i < len; i++) {
            printf("%x", buff[i]);
        }
        printf("\n");
    }
}