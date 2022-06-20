#if !defined(__RECT_H__)
#define __RECT_H__

#include "padding.h"

#define H_LEFT 0
#define H_CENTER 1
#define H_RIGHT 2
#define H_AUTO 3
#define H_AUTO_REVERSE 4

#define V_CENTER 0x20
#define V_BOTTOM 0x40

typedef struct {
    int x, y, w, h;
} rect_t;

rect_t rect_add_padding(const rect_t *rect, const padding_t *padding);
rect_t rect_sub_padding(const rect_t *rect, const padding_t *padding);

#endif // __RECT_H__
