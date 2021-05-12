#include <stdio.h>
#include "hzw.h"

// gcc -c -o main.o main.c
// gcc -o main main.o -lhzw -L.

// LD_LIBRARY_PATH=. ./main

int main() {
    printf("my name is %s.", get_name());
}
