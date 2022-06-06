#if !defined(__GRID_H__)
#define __GRID_H__

#include "rect.h"
#include "../types.h"
#include "../utils.h"

typedef struct {
    void *data;
    int (*len)(void *data);
} grid_adapter_t;

typedef struct {
    rect_t *rects;
    int rows, columns;
    int start, cur;
    grid_adapter_t adapter;
} grid_t;

#define GRID_LEN(grid) ((grid)->adapter.len != NULL ? (grid)->adapter.len((grid)->adapter.data) : 0)
#define GRID_CAP(grid) ((GRID_LEN(grid) + ((grid)->columns - 1)) / grid->columns * grid->columns)
#define GRID_PAGE_CAP(grid) ((grid)->rows * (grid)->columns)
#define GRID_PAGE_LEN(grid) (MIN(GRID_PAGE_CAP(grid), GRID_LEN(grid) - (grid)->start))

void grid_measure(grid_t *grid, const rect_t *rect, int *width, int *height);
void grid_layout(grid_t *grid, const rect_t *rect);
void grid_down(grid_t *grid);
void grid_up(grid_t *grid);
void grid_right(grid_t *grid);
void grid_left(grid_t *grid);

#endif // __GRID_H__
