#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <errno.h>
#include <string.h>
#include <netinet/in.h>
#include <signal.h>
#include <arpa/inet.h>
#include <unistd.h>

int main(int argc, char *argv[]) {
    int local_port = atoi(argv[1]);
    int remote_port = atoi(argv[2]);

    int sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd == -1) {
        printf("%s:%d %d(%s)\n", __FILE__, __LINE__, errno, strerror(errno));
        exit(1);
    }
    int buflen = 0;
    int len = sizeof(buflen);

    getsockopt(sockfd, SOL_SOCKET, SO_RCVBUF, &buflen, &len);
    printf("%d\n", buflen);
    getsockopt(sockfd, SOL_SOCKET, SO_SNDBUF, &buflen, &len);
    printf("%d\n", buflen);

    struct sockaddr_in addr;
    socklen_t addrlen = sizeof(addr);
    //bzero(&addr, addrlen);
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = INADDR_ANY;
    addr.sin_port = htons(local_port);
    if (bind(sockfd, (struct sockaddr*)&addr, sizeof(addr)) == -1) {
        printf("%s:%d %d(%s)\n", __FILE__, __LINE__, errno, strerror(errno));
        exit(1);
    }

    inet_aton("127.0.0.1", &addr.sin_addr);
    addr.sin_port = htons(remote_port);
    if (connect(sockfd, (struct sockaddr*)&addr, sizeof(addr)) == -1) {
        printf("%s:%d %d(%s)\n", __FILE__, __LINE__, errno, strerror(errno));
        exit(1);
    }
#if 0
    if (getsockname(sockfd, (struct sockaddr*)&addr, &addrlen) == -1) {
        printf("%s:%d %d(%s)\n", __FILE__, __LINE__, errno, strerror(errno));
        exit(1);
    }
    printf("%s:%d\n", inet_ntoa(addr.sin_addr), ntohs(addr.sin_port));
#endif
    close(sockfd);
}