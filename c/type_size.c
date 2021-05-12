#include <stdio.h>

int main(int argc, char *argv[]) {
    printf("int: %lu\n", sizeof(int));
    printf("short: %lu\n", sizeof(short));
    printf("long: %lu\n", sizeof(long));
    printf("long long: %lu\n", sizeof(long long));
    printf("float: %lu\n", sizeof(float));
    printf("char: %lu\n", sizeof(char));
    printf("void*: %lu\n", sizeof(void*));
    printf("int*: %lu\n", sizeof(int*));

    char *str = "building abstract mechanisms of increasing complexity from lower-level ones.";
    void *p1 = str;
    printf("%s\n", (char*)(p1 + 6));
    int *p2 = (int*)str;
    printf("%s\n", (char*)(p2 + 6));
    int ints[] = {0x12345678, 0x234F6781, 0x34567812};
    p1 = ints;
    printf("%X\n", *(char*)(p1 + 0));
    printf("%X\n", *(char*)(p1 + 1));
    printf("%X\n", *(char*)(p1 + 5));
}
