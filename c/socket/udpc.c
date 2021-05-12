#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>

#include <arpa/inet.h>

#include <unistd.h>

#define SERVER "10.0.37.139"
#define PORT 19268

int main() {
    int sock;

    if ((sock = socket(AF_INET, SOCK_DGRAM, 0)) == -1) {
        fprintf(stderr, "%s:%d: %d(%s)", __FILE__, __LINE__, errno, strerror(errno));
        exit(1);
    }

    char buff[] = "abc";
    struct sockaddr_in addr_other;
    addr_other.sin_family = AF_INET;
    addr_other.sin_port = htons(PORT);
    if (inet_aton(SERVER, &addr_other.sin_addr) == 0) {
        fprintf(stderr, "%s:%d: %d(%s)", __FILE__, __LINE__, errno, strerror(errno));
        exit(1);
    }
    while (1) {
        if (sendto(sock, buff, sizeof(buff), 0, (struct sockaddr*)&addr_other, sizeof(addr_other)) == -1) {
            fprintf(stderr, "%s:%d: %d(%s)", __FILE__, __LINE__, errno, strerror(errno));
            exit(1);
        }
        sleep(1);
    }
}
