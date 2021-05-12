#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

#include "utils.h"

//#define TEST

void printerr(char *file, int line, int err) {
    fprintf(stderr, "%s:%d %s(%d)\n", file, line, strerror(err), err);
}

void die(char *file, int line, int err) {
    printerr(file, line, err);
    exit(EXIT_FAILURE);
}

int hex_to_num(int hex) {
    if (hex >= 0x30/*0*/ && hex <= 0x39/*9*/) {
        return hex - 0x30;
    } else if (hex >= 0x41/*A*/ && hex <= 0x46/*F*/) {
        return hex - 0x41 + 10;
    } else if (hex >= 0x61/*a*/ && hex <= 0x66/*f*/) {
        return hex - 0x61 + 10;
    } else {
        return -1;
    }
}

int num_to_hex(int num, int lower) {
    if (num >= 0 && num <= 9) {
        return num + 0x30/*0*/;
    } else if (num >= 10 && num <= 15) {
        num -= 10;
        return lower ? num + 0x61/*a*/ : num + 0x41/*A*/;
    } else {
        return -1;
    }
}

size_t unhexlify(const char *str, size_t slen, uint8_t *data, size_t dlen) {
    size_t total = 0;
    int num1, num2;

    while (slen > 1) {
        num1 = hex_to_num(*str);
        if (num1 == -1) {
            break;
        }
        str++;
        slen--;

        num2 = hex_to_num(*str);
        if (num2 == -1) {
            break;
        }
        str++;
        slen--;

        if (dlen == 0) {
            break;
        }

        *data = num1 * 16 + num2;
        data++;
        dlen--;
        total++;
    }

    return total;
}

size_t hexlify(const uint8_t *data, size_t dlen, char *str, size_t slen, int lower) {
    size_t total = 0;
    int num1, num2;

    while (dlen != 0) {
        num1 = num_to_hex(*data / 16, lower);
        if (num1 == -1) {
            break;
        }

        num2 = num_to_hex(*data % 16, lower);
        if (num2 == -1) {
            break;
        }

        data++;
        dlen--;

        if (slen < 2) {
            break;
        }

        *str = num1;
        *(str + 1) = num2;
        str += 2;
        slen -= 2;
        total += 2;
    }

    return total;
}

#ifdef TEST

int main(int argc, char *argv[]) {
    char *strU = "5033001403F661FE0148000000000452120308101E06FE9A";
    char *strL = "5033001403f661fe0148000000000452120308101e06fe9a";
    uint8_t data[] = {
        0x50, 0x33,
        0x00, 0x14,
        0x03, 0xF6, 0x61, 0xFE, 0x01, 0x48, 0x00, 0x00,
        0x00, 0x00, 0x04, 0x52,
        0x12, 0x03, 0x08, 0x10, 0x1E, 0x06,
        0xFE,
        0x9A
    };

    char strout[1024];
    uint8_t dataout[512];

    size_t len;

    len = unhexlify(strU, strlen(strU), dataout, sizeof(dataout));
    assert(len == strlen(strU) / 2);
    assert(memcmp(data, dataout, len) == 0);

    len = hexlify(dataout, len, strout, sizeof(strout), 0);
    assert(len == sizeof(data) * 2);
    assert(strncmp(strU, strout, len) == 0);

    len = unhexlify(strL, strlen(strL), dataout, sizeof(dataout));
    assert(len == strlen(strL) / 2);
    assert(memcmp(data, dataout, len) == 0);

    len = hexlify(dataout, len, strout, sizeof(strout), 1);
    assert(len == sizeof(data) * 2);
    assert(strncmp(strL, strout, len) == 0);
}

#endif
