#include <assert.h>
#include <string.h>

#define AXIS_NO_GRAVITY 0x00
#define AXIS_SPECIFIED 0x01
#define AXIS_PULL_BEFORE 0x02
#define AXIS_PULL_AFTER 0x04
#define AXIS_CLIP 0x08

#define AXIS_X_SHIFT 0
#define AXIS_Y_SHIFT 4

#define LEFT ((AXIS_SPECIFIED | AXIS_PULL_BEFORE) << AXIS_X_SHIFT)
#define RIGHT ((AXIS_SPECIFIED | AXIS_PULL_AFTER) << AXIS_X_SHIFT)
#define TOP ((AXIS_SPECIFIED | AXIS_PULL_BEFORE) << AXIS_Y_SHIFT)
#define BOTTOM ((AXIS_SPECIFIED | AXIS_PULL_AFTER) << AXIS_Y_SHIFT)

#define CENTER_HORIZONTAL (AXIS_SPECIFIED << AXIS_X_SHIFT)
#define FILL_HORIZONTAL (LEFT | RIGHT)
#define CENTER_VERTICAL (AXIS_SPECIFIED << AXIS_Y_SHIFT)
#define FILL_VERTICAL (TOP | BOTTOM)

#define HORIZONTAL_GRAVITY_MASK ((AXIS_SPECIFIED | AXIS_PULL_BEFORE | AXIS_PULL_AFTER) << AXIS_X_SHIFT)
#define VERTICAL_GRAVITY_MASK ((AXIS_SPECIFIED | AXIS_PULL_BEFORE | AXIS_PULL_AFTER) << AXIS_Y_SHIFT)

#define HORIZONTAL_CLIP_MASK (AXIS_CLIP << AXIS_X_SHIFT)
#define VERTICAL_CLIP_MASK (AXIS_CLIP << AXIS_Y_SHIFT)
#define CLIP_MASK (HORIZONTAL_CLIP_MASK | VERTICAL_CLIP_MASK)

#define RELATIVE_LAYOUT_DIRECTION 0x00800000

#define START (RELATIVE_LAYOUT_DIRECTION | LEFT)
#define END (RELATIVE_LAYOUT_DIRECTION | RIGHT)

#define LAYOUT_DIRECTION_LTR 0
#define LAYOUT_DIRECTION_RTL 1

typedef struct {
    int left, right, top, bottom;
} Rect;

int resolve_gravity(int gravity, int layoutDirection) {
    int result = gravity;

    if (result & RELATIVE_LAYOUT_DIRECTION) {
        result &= ~RELATIVE_LAYOUT_DIRECTION;
        if (layoutDirection == LAYOUT_DIRECTION_RTL) {
            if ((result & LEFT) == LEFT) {
                result &= ~LEFT;
                result |= RIGHT;
            } else if ((result & RIGHT) == RIGHT) {
                result &= ~RIGHT;
                result |= LEFT;
            }
        }
    }

    return result;
}

void apply_gravity(int gravity, int w, int h, Rect container, int xAdj, int yAdj, Rect *out, int layoutDirection) {
    gravity = resolve_gravity(gravity, layoutDirection);
    
    switch (gravity & HORIZONTAL_GRAVITY_MASK) {
        case CENTER_HORIZONTAL:
            out->left = container.left + (container.right - container.left - w) / 2 + xAdj;
            out->right = out->left + w;
            break;
        case LEFT:
            out->left = container.left + xAdj;
            out->right = out->left + w;
            break;
        case RIGHT:
            out->right = container.right - xAdj;
            out->left = out->right - w;
            break;
        default:
            out->left = container.left + xAdj;
            out->right = container.right - xAdj;
            break;
    }

    if (gravity & HORIZONTAL_CLIP_MASK) {
        if (out->left < container.left) {
            out->left = container.left;
        }
        if (out->right > container.right) {
            out->right = container.right;
        }
    }

    switch(gravity & VERTICAL_GRAVITY_MASK) {
        case CENTER_VERTICAL:
            out->top = container.top + (container.bottom - container.top - h) / 2 + yAdj;
            out->bottom = out->top + h;
            break;
        case TOP:
            out->top = container.top + yAdj;
            out->bottom = out->top + h;
            break;
        case BOTTOM:
            out->bottom = container.bottom - yAdj;
            out->top = out->bottom - h;
            break;
        default:
            out->top = container.top + yAdj;
            out->bottom = container.bottom - yAdj;
            break;
    }

    if (gravity & VERTICAL_CLIP_MASK) {
        if (out->top < container.top) {
            out->top = container.top;
        }
        if (out->bottom > container.bottom) {
            out->bottom = container.bottom;
        }
    }
}

void test_apply_gravity() {
    struct {
        int gravity;
        int w, h;
        Rect container, out, result;
        int xAdj, yAdj;
        int layoutDirection;
    } tests[] = {
        {
            LEFT,
            50, 50,
            {0, 100, 0, 100}, {0}, {0, 50, 0, 100},
            0, 0,
            LAYOUT_DIRECTION_LTR,
        },
         {
            RIGHT,
            50, 50,
            {0, 100, 0, 100}, {0}, {50, 100, 0, 100},
            0, 0,
            LAYOUT_DIRECTION_LTR,
        },
         {
            LEFT | RIGHT,
            50, 50,
            {0, 100, 0, 100}, {0}, {0, 100, 0, 100},
            0, 0,
            LAYOUT_DIRECTION_LTR,
        },
        {
            CENTER_HORIZONTAL,
            50, 50,
            {0, 100, 0, 100}, {0}, {25, 75, 0, 100},
            0, 0,
            LAYOUT_DIRECTION_LTR,
        },
         {
            CENTER_VERTICAL,
            50, 50,
            {0, 100, 0, 100}, {0}, {0, 100, 25, 75},
            0, 0,
            LAYOUT_DIRECTION_LTR,
        },
        {
            CENTER_HORIZONTAL | CENTER_VERTICAL,
            50, 50,
            {0, 100, 0, 100}, {0}, {25, 75, 25, 75},
            0, 0,
            LAYOUT_DIRECTION_LTR,
        },

        {
            LEFT,
            200, 200,
            {0, 100, 0, 100}, {0}, {0, 200, 0, 100},
            0, 0,
            LAYOUT_DIRECTION_LTR,
        },
         {
            LEFT | CLIP_MASK,
            200, 200,
            {0, 100, 0, 100}, {0}, {0, 100, 0, 100},
            0, 0,
            LAYOUT_DIRECTION_LTR,
        },

        {
            START,
            50, 50,
            {0, 100, 0, 100}, {0}, {50, 100, 0, 100},
            0, 0,
            LAYOUT_DIRECTION_RTL,
        },
         {
            END,
            50, 50,
            {0, 100, 0, 100}, {0}, {0, 50, 0, 100},
            0, 0,
            LAYOUT_DIRECTION_RTL,
        },
        {
            START | TOP,
            50, 50,
            {0, 100, 0, 100}, {0}, {50, 100, 0, 50},
            0, 0,
            LAYOUT_DIRECTION_RTL,
        },
         {
            END | BOTTOM,
            50, 50,
            {0, 100, 0, 100}, {0}, {0, 50, 50, 100},
            0, 0,
            LAYOUT_DIRECTION_RTL,
        },
         {
            END | CENTER_VERTICAL,
            50, 50,
            {0, 100, 0, 100}, {0}, {0, 50, 25, 75},
            0, 0,
            LAYOUT_DIRECTION_RTL,
        },
        {
            START,
            200, 200,
            {0, 100, 0, 100}, {0}, {-100, 100, 0, 100},
            0, 0,
            LAYOUT_DIRECTION_RTL,
        },
         {
            START | CLIP_MASK,
            200, 200,
            {0, 100, 0, 100}, {0}, {0, 100, 0, 100},
            0, 0,
            LAYOUT_DIRECTION_RTL,
        },
        {
            END,
            200, 200,
            {0, 100, 0, 100}, {0}, {0, 200, 0, 100},
            0, 0,
            LAYOUT_DIRECTION_RTL,
        },
         {
            END | CLIP_MASK,
            200, 200,
            {0, 100, 0, 100}, {0}, {0, 100, 0, 100},
            0, 0,
            LAYOUT_DIRECTION_RTL,
        },
    };
    int i = 0;

    for (i = 0; i < sizeof(tests) / sizeof(tests[0]); i++) {
        apply_gravity(
            tests[i].gravity,
            tests[i].w, tests[i].h,
            tests[i].container,
            tests[i].xAdj, tests[i].yAdj,
            &tests[i].out,
            tests[i].layoutDirection);
        assert(memcmp(&tests[i].out, &tests[i].result, sizeof(Rect)) == 0);
    }
}
