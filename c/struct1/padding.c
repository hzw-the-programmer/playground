#include <stdio.h>
//http://www.catb.org/esr/structure-packing/

//typedef struct __attribute__ ((packed)) {
typedef struct {
    /*
    char d1;
    int d2;
    short d3;
    char d4;
    */
    /*
    char d1;
    short d3;
    char d4;
    int d2;
    */
    char d1;
    char d4;
    short d3;
    int d2;
} data;

int main(int argc, char **argv) {
    data d;
    d.d1 = 0x01;
    d.d2 = 0x02030405;
    d.d3 = 0x0607;
    d.d4 = 0x08;
    char *p = (char*)&d;
    int i;
    for (i = 0; i < sizeof(data); i++) {
        printf("%02x", *(p++));
    }
    printf("\n");
}
