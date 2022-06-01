#include "grid.h"
#include "../utils.h"

void grid_measure(grid_t *grid, const rect_t *rect, int *width, int *height) {
    int w, h, s, e, i;
    grid_item_t *item;

    w = rect->w / grid->columns;
    h = rect->h / grid->rows;
    grid_displayed_range(grid, &s, &e);
    for (i = s; i < e; i++) {
        item = &grid->items[i];
        item->rect.w = w;
        item->rect.h = h;
    }

    if (width) *width = w;
    if (height) *height = h;
}

void grid_layout(grid_t *grid, const rect_t *rect) {
    int columns, x, y, s, e, i;
    grid_item_t *item;

    columns = grid->columns;
    x = rect->x;
    y = rect->y;
    grid_displayed_range(grid, &s, &e);
    for (i = s; i < e; i++) {
        item = &grid->items[i];
        item->rect.x = x;
        item->rect.y = y;
        if (i % columns == columns - 1) {
            x = rect->x;
            y += item->rect.h;
        } else {
            x += item->rect.w;
        }
    }
}

void grid_displayed_range(const grid_t *grid, int *start, int *end) {
    if (start) *start = grid->start;
    if (end) {
        *end = grid->start + MIN(grid->rows * grid->columns, grid->len - grid->start);
    }
}

int grid_max_start(const grid_t *grid) {
    int lcs, nl;

    nl = grid->len;
    lcs = nl % grid->columns;
    if (lcs != 0) {
        nl += grid->columns - lcs;
    }

    return nl - (grid->rows * grid->columns);
}

int grid_next_column(int i, int columns) {
    int cl;

    cl = i % columns;
    cl++;
    cl %= columns;

    return cl;
}

void grid_down(grid_t *grid) {
    int ns, nc, s, e;

    ns = grid->start;
    nc = grid->cur + grid->columns;
    grid_displayed_range(grid, &s, &e);
    if (nc >= e) {
        ns += grid->columns;
        if (ns > grid_max_start(grid)
            || nc >= grid->len) {
            ns = 0;
            nc = grid_next_column(nc, grid->columns);
            if (nc >= grid->len) {
                nc = 0;
            }
        }
    }
    grid->start = ns;
    grid->cur = nc;
}
