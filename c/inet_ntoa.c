#include <stdio.h>
#include <stdlib.h>
#include <arpa/inet.h>

void dumpnumber(uint8_t *num, size_t len) {
    for (int i = 0; i < len; i++) {
        fprintf(stdout, "%d ", num[i]);
    }
    fprintf(stdout, "\n");
}

int main(int argc, char *argv[]) {
    struct in_addr sin_addr;
    char *cp;

    if (argc != 2) {
        fprintf(stderr, "Usage: %s ip\n", argv[0]);
        exit(EXIT_FAILURE);
    }

    sin_addr.s_addr = atoi(argv[1]);
    fprintf(stderr, "%x\n", sin_addr.s_addr);
    
    dumpnumber((uint8_t*)&sin_addr.s_addr, sizeof(sin_addr.s_addr));
    cp = inet_ntoa(sin_addr);

    fprintf(stdout, "%s\n", cp);
}
