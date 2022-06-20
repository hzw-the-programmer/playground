#include "rect.h"

rect_t rect_add_padding(const rect_t *rect, const padding_t *padding) {
    rect_t r;

    r = *rect;
    r.x -= padding->l;
    r.y -= padding->t;
    r.w += padding->l + padding->r;
    r.h += padding->t + padding->b; 

    return r;
}

rect_t rect_sub_padding(const rect_t *rect, const padding_t *padding) {
    rect_t r;

    r = *rect;
    r.x += padding->l;
    r.y += padding->t;
    r.w -= padding->l + padding->r;
    r.h -= padding->t + padding->b; 

    return r;
}
