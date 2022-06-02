#if !defined(__GRID_H__)
#define __GRID_H__

#include "rect.h"

typedef struct {
    rect_t rect;
} grid_item_t;

typedef struct {
    int rows, columns;
    grid_item_t *items;
    int len;
    int start, cur;
} grid_t;

void grid_measure(grid_t *grid, const rect_t *rect, int *width, int *height);
void grid_layout(grid_t *grid, const rect_t *rect);
void grid_displayed_range(const grid_t *grid, int *start, int *end);
void grid_down(grid_t *grid);
void grid_up(grid_t *grid);

#endif // __GRID_H__
