#include <stdio.h>
#include <stdlib.h>

void bin2hex(char *bin, int length, char *hex) {
    int i;
    for (i = 0; i < length; i++) {
        sprintf(hex + i * 2, "%02x", bin[i]);
    }
    *(hex + i * 2) = '\0';
}

int main(int argc, char **argv) {
    //char bin[] = "\x00\x01\x02\x03\x04";
    //int length = sizeof(bin);
    int data = 0x01020304;
    char *bin = (char*)&data;
    int length = sizeof(int);
    char *hex = malloc(length + 1);
    bin2hex(bin, length, hex);
    printf("0x%s\n", hex);
    free(hex);
    hex = NULL;
    return 0;
}