#include "ll.h"
#include "../utils.h"

static void test1() {
#if 0
    int w, h, max_w, max_h;

    ll_item items[] = {
        {0, 0, 20, 20, 0, -1, 0, NULL, NULL},
        
    };

    ll_measure(items, ARRAY_SIZE(items), &w, &h, &max_w, &max_h);
    assert(w == 90);
    assert(h == 60);
    assert(max_w == );
#endif
}

void ll_test() {
    test1();
}