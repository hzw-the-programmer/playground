#include <stdio.h>
#include <stdlib.h>
#include "stdint.h"
#include "test1.h"

int get_content(const char *fn, uint8_t **data, long *fs) {
    FILE *f;
    size_t n;
    int result = 1;

    f = fopen(fn, "rb");
    if (!f) {
        perror("fopen");
        goto end;
    }

    if (fseek(f, 0, SEEK_END)) {
        perror("fseek");
        goto end;
    }

    *fs = ftell(f);
    if (*fs == -1) {
        perror("ftell");
        goto end;
    }

    if (fseek(f, 0, SEEK_SET)) {
        perror("fseek");
        goto end;
    }

    *data = (uint8_t*)malloc(*fs);
    if (!*data) {
        perror("malloc");
        goto end;
    }

    n = fread(*data, 1, *fs, f);
    if (n != *fs) {
        free(*data);
        printf("fread: %d!=%d", n, *fs);
        goto end;
    }

    result = 0;

end:
    if (f) {
        fclose(f);
    }
    return result;
}

int main(int argc, char *argv[]) {
    uint8_t *buf;
    size_t len, n;
    
    if (argc != 2) {
        printf("Usage: %s fn\n", argv[0]);
        return 1;
    }

    if (get_content(argv[1], &buf, &len)) {
        printf("get_content failed\n");
        return 1;
    }

    test1_process(buf, len);

    return 0;
}
