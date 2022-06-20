#if !defined(__CANVAS_H__)
#define __CANVAS_H__

typedef struct {
    int tx, ty;
} canvas_t;

canvas_t canvas_new();
void canvas_translate(canvas_t *canvas, int tx, int ty);
void canvas_transform(const canvas_t *canvas, int *x, int *y);

#endif
