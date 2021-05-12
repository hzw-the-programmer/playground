#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
    char res[1024] = {0};

    if (argc != 2) {
        printf("Usage: %s str\n", argv[0]);
        exit(1);
    }

    int i = 0, j = 0;
    while (argv[1][i]) {
        if (argv[1][i] == ' ' || argv[1][i] == '\t') {
            if (j != 0) {
                if (res[j - 1] != ' ') {
                    res[j++] = ',';
                    res[j++] = ' ';
                }
            }
        } else {
            res[j++] = argv[1][i];
        }
        i++;
    }

    if (res[j - 1] == ' ') {
        res[j - 2] = '\0';
    }

    printf("%s\n", res);
}
