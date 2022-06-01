#include <assert.h>
#include "grid.h"

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

void grid_test() {
    grid_layout_test_1();
}