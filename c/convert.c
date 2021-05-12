#include <stdio.h>

void printhex(char* p, int l) {
    for (int i = 0; i < l; i++) {
        printf("%02x", *p++);
    }
    printf("\n");
}

int main(int argc, char** argv) {
    int i = 600;
    printhex((char*)&i, sizeof(int));
    char c = (char)i;
    printhex(&c, sizeof(char));
}
