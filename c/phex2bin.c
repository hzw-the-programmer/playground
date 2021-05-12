#include <stdio.h>
#include <string.h>

void print_hex2bin(char *src) {
    for (int i = 0; i < strlen(src); i+=2) {
        printf("\\x%c%c", src[i], src[i+1]);
    }
}

int main(int argc, char* argv[]) {
    if (argc != 2) {
        printf("Usage: %s hex\n", argv[0]);
        return 1;
    }
    printf("%d\n", (int)strlen(argv[1]));
    print_hex2bin(argv[1]); printf("\n");
}
