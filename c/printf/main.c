#include <stdio.h>
#include "printf.h"

int main() {
    printf("%06.2d\n", -1);
    printf("%064d\n", 1);
}

void _putchar(char character) {
    putchar(character);
}
