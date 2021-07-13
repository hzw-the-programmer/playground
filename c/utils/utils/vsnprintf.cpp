#include <assert.h> // assert
#include <stdio.h> // sprintf
#include <string.h> // strcmp

void test_sprintf() {
    char dst[256] = {0};
    
    assert(sprintf(dst, "%.2d", -1) == 3);
    assert(strcmp(dst, "-01") == 0);

    assert(sprintf(dst, "%6.2d", -1) == 6);
    assert(strcmp(dst, "   -01") == 0);

    assert(sprintf(dst, "%06d", 1) == 6);
    assert(strcmp(dst, "000001") == 0);

    assert(sprintf(dst, "%+06d", 1) == 6);
    assert(strcmp(dst, "+00001") == 0);

    assert(sprintf(dst, "% 06d", 1) == 6);
    assert(strcmp(dst, " 00001") == 0);

    assert(sprintf(dst, "%-6.2d", -1) == 6);
    assert(strcmp(dst, "-01   ") == 0);

    assert(sprintf(dst, "%-06.2d", -1) == 6);
    assert(strcmp(dst, "-01   ") == 0);

    assert(sprintf(dst, "%.2d", -123) == 4);
    assert(strcmp(dst, "-123") == 0);
}

void test_vsnprintf() {
    test_sprintf();
}
