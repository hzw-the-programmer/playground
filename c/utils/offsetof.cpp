#include <stdio.h>
#include <stddef.h> // offsetof

typedef struct {
    char c;
    int i;
} ST1;

void test_offsetof() {
    printf("offsetof(ST1, i)=%d\n", offsetof(ST1, i));
    printf("sizeof(ST1)=%d\n", sizeof(ST1));
}
