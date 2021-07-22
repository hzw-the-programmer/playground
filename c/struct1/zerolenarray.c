#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct s1 {
    char d1;
    int d2;
    char d3;
    char buf[];
} s1_t;

void pbuf(char *buf, size_t len) {
    for (int i = 0; i < len; i++) {
        printf("%02x", buf[i]);
    }
}

int main(int argc, char **argv) {
    s1_t s;
    printf("%lu\n", sizeof(s));

    int size = sizeof(s) + 4;
    void *buf = malloc(size);
    memset(buf, 0, size);
    pbuf(buf, size);
    printf("\n");

    s1_t *ps1 = buf;
    ps1->d1 = 0x01;
    ps1->d2 = 0x02030405;
    ps1->d3 = 0x06;
    ps1->buf[0] = 0x07;
    ps1->buf[1] = 0x08;
    ps1->buf[2] = 0x09;
    ps1->buf[3] = 0x0a;
    pbuf(buf, size);
    printf("\n");

    s1_t s1ts[2] = {};
    for (int i = 0; i < 2; i++) {
        //s1ts[i].d1 = 0x01;
        s1ts[i].d2 = 0x02030405;
        s1ts[i].d3 = 0x06;
        s1ts[i].buf[0] = 0x07;
        s1ts[i].buf[1] = 0x08;
        s1ts[i].buf[2] = 0x09;
        if (i == 0)
            s1ts[i].buf[3] = 0x0a;
    }
    pbuf((char*)s1ts, sizeof(s1_t) * 2);
    printf("\n");

    return 0;
}
