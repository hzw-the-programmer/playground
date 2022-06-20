#include "scroll_bar.h"

void scroll_bar_layout(scroll_bar_t *sb, const rect_t *rect) {
    padding_t padding = {
        0,
        SCROLL_BAR_DEFAULT_PADDING,
        SCROLL_BAR_DEFAULT_PADDING,
        SCROLL_BAR_DEFAULT_PADDING,
    };
    rect_t r;

    padding.l = rect->w - padding.r - SCROLL_BAR_DEFAULT_WIDTH;
    r = rect_add_padding(rect, &padding);
    r.y += (int)(sb->offset_percentage * r.h);
    r.h = (int)(sb->display_percentage * r.h);
    sb->rect = r;
}
