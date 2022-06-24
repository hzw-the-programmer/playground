#include <assert.h>
#include "canvas.h"

void canvas_test() {
    canvas_t canvas;
    int x, y;

    canvas = canvas_new();
    canvas_translate(&canvas, 0, -10);
    x = 0, y = 0;
    canvas_transform(&canvas, &x, &y);
    assert(x == 0 && y == -10);

    canvas_translate(&canvas, 0, 10);
    x = 0, y = 0;
    canvas_transform(&canvas, &x, &y);
    assert(x == 0 && y == 0);
}