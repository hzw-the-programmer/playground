#include <stdio.h>
#include "mem/mem.h"

void slice_test();
void split_test();

void main() {
    slice_test();
    split_test();
    
    {
        char c;
        hmcheck();
        printf("success\n");
        scanf("%c", &c);
    }
}