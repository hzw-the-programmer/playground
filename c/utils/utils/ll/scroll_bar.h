#include "rect.h"

#define SCROLL_BAR_DEFAULT_PADDING (2)
#define SCROLL_BAR_DEFAULT_WIDTH (2)

typedef struct {
    rect_t rect;
    float offset_percentage;
    float display_percentage;
} scroll_bar_t;
