#include <string.h> // strcat
#include <stdio.h> // sprintf

#include "memory.h"

void print_buf(unsigned char *buf, int len, int columns) {
    int i = 0;
    char hex[4+1] = {0};
    char *row = NULL;

    row = (char*)HZW_MALLOC((4+2) * columns - 2 + 1);
    if (!row) {
        printf("print_buf: not enough memory\n");
        return;
    }
    row[0] = 0;

    for (i = 0; i < len; i++) {
        if (i % columns != 0) {
            strcat(row, ", ");
        }
        sprintf(hex, "0x%02x", buf[i]);
        strcat(row, hex);
        if (i % columns == columns - 1 || i + 1 == len) {
            printf("%s\n", row);
            row[0] = 0;
        }
    }

    HzwFree(row);
}

void test_print_buf() {
    {
        unsigned char buf[11] = {0};
        print_buf(buf, 11, 10);
    }
    
    {
        unsigned char buf[23] = {0};
        print_buf(buf, 23, 10);
    }

    {
        unsigned char buf[35] = {0};
        print_buf(buf, 35, 10);
    }

    {
        unsigned char buf[47] = {0};
        print_buf(buf, 47, 10);
    }

    {
        unsigned char buf[50] = {0};
        print_buf(buf, 50, 10);
    }

    {
        unsigned char buf[50] = {0};
        print_buf(buf, 50, 13);
    }
}
