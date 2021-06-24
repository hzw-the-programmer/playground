#include <assert.h>

#define unsigned_max(a) (0xffffffffffffffff >> (64 - 8 * sizeof(a)))

void test_max() {
    unsigned char uc;
    unsigned short us;
    unsigned int ui;
    unsigned long ul;
    unsigned long long ull;

    assert(1 == sizeof(char));
    assert(2 == sizeof(short));
    assert(4 == sizeof(int));
    assert(4 == sizeof(long));
    assert(8 == sizeof(long long));
    assert(0xff == unsigned_max(uc));
    assert(0xffff == unsigned_max(us));
    assert(0xffffffff == unsigned_max(ui));
    assert(0xffffffffffffffff == unsigned_max(ull));
}
