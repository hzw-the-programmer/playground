#include "canvas.h"

canvas_t canvas_new() {
    canvas_t canvas = {0};

    return canvas;
}

void canvas_translate(canvas_t *canvas, int tx, int ty) {
    canvas->tx += tx;
    canvas->ty += ty;
}

void canvas_transform(const canvas_t *canvas, int *x, int *y) {
    *x += canvas->tx;
    *y += canvas->ty;
}