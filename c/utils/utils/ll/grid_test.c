#include <string.h>
#include <assert.h>
#include "grid.h"
#include "../utils.h"

void grid_layout_test_helper(grid_t *grid, const rect_t *rect, const grid_item_t *wanted) {
    int i, s, e;
    grid_item_t item;

    grid_measure(grid, rect, 0, 0);
    grid_layout(grid, rect);
    grid_displayed_range(grid, &s, &e);
    for (i = s; i < e; i++) {
        item = grid->items[i];
        assert(item.rect.x == wanted[i - s].rect.x);
        assert(item.rect.y == wanted[i - s].rect.y);
        assert(item.rect.w == wanted[i - s].rect.w);
        assert(item.rect.h == wanted[i - s].rect.h);
    }
}

void grid_test_helper(int len, int *wanted, int count, void (*func)(grid_t*)) {
    grid_t grid = {3, 3, 0, 0, 0, 0};
    int i, j;

    if (!wanted) return;

    grid.len = len;
    for (i = 0; i < count; i++) {
        j = 0;
        for (; wanted[j] != -1 && wanted[j + 1] != -1; j += 2) {
            func(&grid);
            assert(grid.start == wanted[j]);
            assert(grid.cur == wanted[j + 1]);
        }
    }
}

void grid_layout_test_1() {
    grid_item_t items[20];
    grid_t grid = {3, 3, items, 20, 0, 0};
    rect_t rect = {0, 0, 30, 30};
    grid_item_t wanted[9] = {
        {{0, 0, 10, 10}}, {{10, 0, 10, 10}}, {{20, 0, 10, 10}},
        {{0, 10, 10, 10}}, {{10, 10, 10, 10}}, {{20, 10, 10, 10}},
        {{0, 20, 10, 10}}, {{10, 20, 10, 10}}, {{20, 20, 10, 10}},
    };
    int i, s, e, size;

    for (i = 0; i < 13; i += 3) {
        grid.start = i;
        grid_displayed_range(&grid, &s, &e);
        size = grid.rows * grid.columns;
        if (i == 12) {
            size -= 1;
        }
        assert(s == i && e == (s + size));
        grid_layout_test_helper(&grid, &rect, wanted);
    }
}

void grid_down_test() {
    int wanted_0[] = {
        0, 0,
        -1, -1,
    };
    int wanted_1[] = {
        0, 0,
        -1, -1,
    };
    int wanted_2[] = {
        0, 1,
        0, 0,
        -1, -1,
    };
    int wanted_3[] = {
        0, 1,
        0, 2,
        0, 0,
        -1, -1,
    };
    int wanted_7[] = {
        0, 3,
        0, 6,

        0, 1,
        0, 4,

        0, 2,
        0, 5,

        0, 0,
        -1, -1,
    };
    int wanted_20[] = {
        0, 3,
        0, 6,
        3, 9,
        6, 12,
        9, 15,
        12, 18,
        
        0, 1,
        0, 4,
        0, 7,
        3, 10,
        6, 13,
        9, 16,
        12, 19,

        0, 2,
        0, 5,
        0, 8,
        3, 11,
        6, 14,
        9, 17,

        0, 0,
        -1, -1,
    };
    int* tests[100];
    int i;
    
    memset(tests, 0, sizeof(tests));
    tests[0] = wanted_0;
    tests[1] = wanted_1;
    tests[2] = wanted_2;
    tests[3] = wanted_3;
    tests[7] = wanted_7;
    tests[20] = wanted_20;

    for (i = 0; i < ARRAY_SIZE(tests); i++) {
        grid_test_helper(i, tests[i], 2, grid_down);
    }
}

void grid_up_test() {
    int wanted_0[] = {
        0, 0,
        -1, -1,
    };
    int wanted_1[] = {
        0, 0,
        -1, -1,
    };
    int wanted_2[] = {
        0, 1,
        0, 0,
        -1, -1,
    };
    int wanted_3[] = {
        0, 2,
        0, 1,
        
        0, 0,
        
        -1, -1,
    };
    int wanted_4[] = {
        0, 2,
        
        0, 1,
        
        0, 3,
        0, 0,
        
        -1, -1,
    };
    int wanted_5[] = {
        0, 2,
        
        0, 4,
        0, 1,

        0, 3,
        0, 0,
        
        -1, -1,
    };
    int wanted_6[] = {
        0, 5,
        0, 2,
        
        0, 4,
        0, 1,
        
        0, 3,
        0, 0,
        
        -1, -1,
    };
    int wanted_7[] = {
        0, 5,
        0, 2,

        0, 4,
        0, 1,

        0, 6,
        0, 3,
        0, 0,

        -1, -1,
    };
    int wanted_8[] = {
        0, 5,
        0, 2,

        0, 7,
        0, 4,
        0, 1,

        0, 6,
        0, 3,
        0, 0,
        
        -1, -1,
    };
    int wanted_9[] = {
        0, 8,
        0, 5,
        0, 2,

        0, 7,
        0, 4,
        0, 1,

        0, 6,
        0, 3,
        0, 0,

        -1, -1,
    };
    int wanted_20[] = {
        12, 17,
        12, 14,
        9, 11,
        6, 8,
        3, 5,
        0, 2,
        
        12, 19,
        12, 16,
        12, 13,
        9, 10,
        6, 7,
        3, 4,
        0, 1,

        12, 18,
        12, 15,
        12, 12,
        9, 9,
        6, 6,
        3, 3,

        0, 0,
        -1, -1,
    };
    int* tests[100];
    int i;
    
    memset(tests, 0, sizeof(tests));
    tests[0] = wanted_0;
    tests[1] = wanted_1;
    tests[2] = wanted_2;
    tests[3] = wanted_3;
    tests[4] = wanted_4;
    tests[5] = wanted_5;
    tests[6] = wanted_6;
    tests[7] = wanted_7;
    tests[8] = wanted_8;
    tests[9] = wanted_9;
    tests[20] = wanted_20;

    for (i = 0; i < ARRAY_SIZE(tests); i++) {
        grid_test_helper(i, tests[i], 2, grid_up);
    }
}

void grid_test() {
    grid_layout_test_1();
    grid_down_test();
    grid_up_test();
}