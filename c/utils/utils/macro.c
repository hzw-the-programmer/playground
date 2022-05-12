#include <string.h>
#include <assert.h>

#define C1 0x01
#define C2 0x02
#define C3 0x04
#define C4 0x08
#define C5 0x10
#define C6 0x20
#define C7 0x40
#define C8 0x80

#define C (C7 | C8)

void macro_test() {
#if C & C1
    printf("C & C1\n");
#elif C & C2
    printf("C & C2\n");
#elif C & C3
    printf("C & C3\n");
#elif C & C4
    printf("C & C4\n");
#elif C & C5
    printf("C & C5\n");
#elif C & C6
    printf("C & C6\n");
#elif C & C7
    printf("C & C7\n");
#elif C & C8
    printf("C & C8\n");
#else
    printf("no match\n");
#endif
}
