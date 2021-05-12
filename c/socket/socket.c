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

//void sigint(int signum) {
//}

void sigint(int signum, siginfo_t *info, void *ucontext) {

}

int main(int argc, char *argv) {
    struct sigaction act, oldact;
    //act.sa_handler = sigint;
    act.sa_flags = SA_SIGINFO;
    act.sa_sigaction = sigint;
    if (sigaction(SIGINT, &act, &oldact) == -1) {
        printf("%s:%d %d(%s)\n", __FILE__, __LINE__, errno, strerror(errno));
        exit(1);
    }

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
#if 0
    //bzero(&addr, addrlen);
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = INADDR_ANY;
    addr.sin_port = htons(9999);
    if (bind(sockfd, (struct sockaddr*)&addr, sizeof(addr)) == -1) {
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
#endif

    if (listen(sockfd, 1024) == -1) {
        printf("%s:%d %d(%s)\n", __FILE__, __LINE__, errno, strerror(errno));
        exit(1);
    }
#if 1
    if (getsockname(sockfd, (struct sockaddr*)&addr, &addrlen) == -1) {
        printf("%s:%d %d(%s)\n", __FILE__, __LINE__, errno, strerror(errno));
        exit(1);
    }
    printf("%s:%d\n", inet_ntoa(addr.sin_addr), ntohs(addr.sin_port));
#endif

    int sockfd2 = accept(sockfd, (struct sockaddr*)&addr, &addrlen);
    if (sockfd2 == -1) {
        printf("%s:%d %d(%s)\n", __FILE__, __LINE__, errno, strerror(errno));
        exit(1);
    }
    printf("%s:%d\n", inet_ntoa(addr.sin_addr), ntohs(addr.sin_port));
    sleep(10);
    char buf[256];
#if 1
    //int num = send(sockfd2, buf, 256, 0);
    int num = write(sockfd2, buf, 256);
    if (num == -1) {
        printf("%s:%d %d(%s)\n", __FILE__, __LINE__, errno, strerror(errno));
        exit(1);
    } else {
        printf("%d\n", num);
    }
#else
    int num = recv(sockfd2, buf, 256, 0);
    if (num == -1) {
        printf("%s:%d %d(%s)\n", __FILE__, __LINE__, errno, strerror(errno));
        exit(1);
    } else if (num == 0) {
        printf("peer closed\n");
    }
#endif
}
