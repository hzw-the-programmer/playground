#include <stdio.h>
#include "mem/mem.h"

void slice_test();
void split_test();
void buffer_test();
void writer_test();
void reader_test();

void main() {
    slice_test();
    split_test();
    buffer_test();
    writer_test();
    reader_test();
    
    {
        char c;
        hmcheck();
        printf("success\n");
        scanf("%c", &c);
    }
}