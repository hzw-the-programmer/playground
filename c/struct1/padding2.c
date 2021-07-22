#include <stdio.h>

typedef struct sshort {
    /*
    char d1;
    short d2;
    */
    short d2;
    char d1;
} sshort_t;

typedef struct sint {
    /*
    int d1;
    char d2;
    */
    char d2;
    int d1;
} sint_t;

typedef struct ssi {
    long l;
    sshort_t s;
    sint_t i;
} ssi_t;

void pbuf(char *buf, size_t len) {
    for (int i = 0; i < len; i++) {
        printf("%02x", buf[i]);
    }
}

int main(int argc, char **argv) {
    sshort_t ss = {};
    ss.d1 = 0x01;
    ss.d2 = 0x0102;
    pbuf((char*)&ss, sizeof(ss));
    printf("\n");

    sint_t si = {};
    si.d1 = 0x01020304;
    si.d2 = 0x05;
    pbuf((char*)&si, sizeof(si));
    printf("\n");

    ssi_t ssi = {};
    ssi.l = 0x0102030405060708;
    ssi.s.d1 = 0x01;
    ssi.s.d2 = 0x0102;
    ssi.i.d1 = 0x01020304;
    ssi.i.d2 = 0x05;
    pbuf((char*)&ssi, sizeof(ssi));
    printf("\n");

    ssi_t ssis[2] = {};
    for (int i = 0; i < 2; i++) {
        ssis[i].l = 0x0102030405060708;
        ssis[i].s.d1 = 0x01;
        ssis[i].s.d2 = 0x0102;
        ssis[i].i.d1 = 0x01020304;
        ssis[i].i.d2 = 0x05;
    }
    pbuf((char*)ssis, sizeof(ssis));
    printf("\n");

    return 0;
}
