#include <stdio.h>
#include "mem/mem.h"

void slice_test();
void split_test();
void buffer_test();
void len_reader_writer_test();
void sep_reader_writer_test();
void http_test();

void main() {
    slice_test();
    split_test();
    buffer_test();
    len_reader_writer_test();
    sep_reader_writer_test();
    http_test();
    
    {
        char c;
        hmcheck();
        printf("success\n");
        scanf("%c", &c);
    }
}