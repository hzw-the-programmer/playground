#include <string.h>

void test_array2d_1() {
    char strs[2][10];
    int i;

    strcpy(strs[0], "hello");
    strcpy(strs[1], "world");

    for (i = 0; i< 2; i++) {
        printf("%s\n", strs[i]);
    }
}

void test_array2d_2() {
    char strs[2][16];
    int i;
    char **ps;

    ps = strs;

    printf("%p\n", strs);
    printf("%p\n", ps);

    printf("%p\n", strs + 1);
    printf("%p\n", ps + 1);
}

void test_array2d() {
    test_array2d_1();
    test_array2d_2();
}