#include <stdio.h>
#include <stdlib.h>
#include <netdb.h>
#include <arpa/inet.h>

char *family_str(int family) {
    switch (family) {
        case AF_INET: // 2
            return "AF_INET";
        default:
            return "UNKNKOW";
    }
}

char *socktype_str(int socktype) {
    switch (socktype) {
        case SOCK_STREAM: // 1
            return "SOCK_STREAM";
        case SOCK_DGRAM: // 2
            return "SOCK_DGRAM";
        case SOCK_RAW: // 3
            return "SOCK_RAW";
        case SOCK_SEQPACKET:
            return "SOCK_SEQPACKET";
        default:
            return "UNKNKOW";
    }
}

char *addr_str(struct sockaddr *addr) {
    static char str[50];
    struct sockaddr_in *addr_in;

    switch (addr->sa_family) {
        case AF_INET:
            addr_in = (struct sockaddr_in*)addr;
            sprintf(str, "%s:%d", inet_ntoa(addr_in->sin_addr), addr_in->sin_port);
            break;
        default:
            sprintf(str, "unknown");
            break;
    }

    return str;
}

int main(int argc, char *argv[]) {
    struct addrinfo *res, *r;
    int s;
    
    if (argc != 2) {
        fprintf(stderr, "Usage: %s url\n", argv[0]);
        exit(EXIT_FAILURE);
    }

    s = getaddrinfo(argv[1], NULL, NULL, &res);
    if (s != 0) {
        fprintf(stderr, "getaddrinfo: %s\n", gai_strerror(s));
        exit(EXIT_FAILURE);
    }

    for (r = res; r != NULL; r = r->ai_next) {
        fprintf(stdout, "***\n");
        fprintf(stdout, "family: %s(%d)\n", family_str(r->ai_family), r->ai_family);
        fprintf(stdout, "socktype: %s(%d)\n", socktype_str(r->ai_socktype), r->ai_socktype);
        fprintf(stdout, "addr: %s\n", addr_str(r->ai_addr));
        fprintf(stdout, "***\n");
    }

    freeaddrinfo(res);
}

/*
struct addrinfo {
    int              ai_flags;
    int              ai_family;
    int              ai_socktype;
    int              ai_protocol;
    socklen_t        ai_addrlen;
    struct sockaddr *ai_addr;
    char            *ai_canonname;
    struct addrinfo *ai_next;
};

struct sockaddr {
    sa_family_t sa_family;
    char        sa_data[14];
};

struct sockaddr_in {
    sa_family_t    sin_family;
    in_port_t      sin_port;
    struct in_addr sin_addr;
};

struct in_addr {
    uint32_t       s_addr;
};
*/
