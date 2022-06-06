#include "grid.h"

void grid_measure(grid_t *grid, const rect_t *rect, int *width, int *height) {
    int i;

    for (i = 0; i < GRID_PAGE_CAP(grid); i++) {
        grid->rects[i].w = rect->w / grid->columns;
        grid->rects[i].h = rect->h / grid->rows;
    }

    if (width) *width = rect->w;
    if (height) *height = rect->h;
}

void grid_layout(grid_t *grid, const rect_t *rect) {
    int columns, x, y, i;

    columns = grid->columns;
    x = rect->x;
    y = rect->y;

   for (i = 0; i < GRID_PAGE_CAP(grid); i++) {
        grid->rects[i].x = x;
        grid->rects[i].y = y;
        if (i % columns == columns - 1) {
            x = rect->x;
            y += grid->rects[i].h;
        } else {
            x += grid->rects[i].w;
        }
    }
}

void grid_down(grid_t *grid) {
    int s, c, len;

    len = GRID_LEN(grid);
    if (len == 0) return;

    s = grid->start;
    c = grid->cur;

    c += grid->columns;
    if (c >= s + GRID_PAGE_CAP(grid)) {
        s += grid->columns;
    }
    if (c >= len) {
        s = 0;
        c = grid->cur % grid->columns;
        c++;
        c %= MIN(len, grid->columns);
    }
    
    grid->start = s;
    grid->cur = c;
}

void grid_up(grid_t *grid) {
    int s, c, len;

    len = GRID_LEN(grid);
    if (len == 0) return;

    s = grid->start;
    c = grid->cur;

    c -= grid->columns;
    if (c < s) {
        s -= grid->columns;
    }
    if (c < 0) {
            s = GRID_CAP(grid) - GRID_PAGE_CAP(grid);
            if (s < 0) s = 0;
            c = grid->cur % grid->columns;
            c--;
            if (c < 0) c = MIN(len, grid->columns) - 1;
            c = s + MIN(GRID_PAGE_CAP(grid), GRID_CAP(grid))  + c - grid->columns;
            if (c >= len) c -= grid->columns;
    }

    grid->start = s;
    grid->cur = c;
}

void grid_right(grid_t *grid) {
    int s, c, len;

    len = GRID_LEN(grid);
    if (len == 0) return;

    s = grid->start;
    c = grid->cur;

    c++;
    if (c >= s + GRID_PAGE_CAP(grid)) {
        s += grid->columns;
    }
    if (c >= len) {
        s = c = 0;
    }

    grid->start = s;
    grid->cur = c;
}

void grid_left(grid_t *grid) {
    int s, c, len;

    len = GRID_LEN(grid);
    if (len == 0) return;

    s = grid->start;
    c = grid->cur;

    c--;
    if (c < s) {
        s -= grid->columns;
    }
    if (c < 0) {
        s = GRID_CAP(grid) - GRID_PAGE_CAP(grid);
        if (s < 0) s = 0;
        c = len - 1;
    }

    grid->start = s;
    grid->cur = c;
}
