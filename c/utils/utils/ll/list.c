#include "list.h"
#include "string.h"

list_t list_new(const rect_t *rect, rect_t *rects, int len, const list_adapter_t *adapter) {
    list_t list = {0};

    list.rect = *rect;
    list.rects = rects;
    list.len = len;
    list.adapter = *adapter;

    return list;
}

void list_measure(list_t *list) {
    int i, min;

    min = list->rect.h / list->len;
    for (i = 0; i < list->len; i++) {
        list->adapter.measure(
            list->adapter.data, list->start + i,
            list->start + i == list->current, &list->rect,
            &list->rects[i].w, &list->rects[i].h);
        if (list->rects[i].h < min) {
            list->rects[i].h = min;
        }
    }
}

void list_layout(list_t *list) {
    int i, x, y;

    x = list->rect.x;
    y = list->rect.y + list->offset;
    for (i = 0; i < list->len; i++) {
        list->rects[i].x = x;
        list->rects[i].y = y;
        y += list->rects[i].h;
    }
}

void list_rects_offset_y(list_t *list, int offset) {
    int i;

    for (i = 0; i < list->len; i++) {
        list->rects[i].y += offset;
    }
}

void list_rects_in_end(list_t *list, const rect_t *rect) {
    memmove(list->rects, list->rects + 1, (list->len - 1) * sizeof(rect_t));
    list->rects[list->len - 1] = *rect;
}

void list_rects_in_front(list_t *list, const rect_t *rect) {
    memmove(list->rects + 1, list->rects, (list->len - 1) * sizeof(rect_t));
    list->rects[0] = *rect;
}

void list_jump_to_bottom(list_t *list) {
    int len, index;
    int current_y, max_y;

    len = list->adapter.len(list->adapter.data);
    list->start = len - list->len;
    list->current = len - 1;
    index = list->current - list->start;

    list_measure(list);
    list_layout(list);

    current_y = list->rects[index].y + list->rects[index].h;
    max_y = list->rect.y + list->rect.h;

    if (current_y > max_y) {
        list_rects_offset_y(list, max_y - current_y);
    } 
}

void list_jump_to_top(list_t *list) {
    list->start = 0;
    list->current = 0;
    
    list_measure(list);
    list_layout(list);
}

void list_down(list_t *list) {
    int start, current, len, index;
    int current_y, max_y;
    
    start = list->start;
    current = list->current;
    len = list->adapter.len(list->adapter.data);

    current++;
    
    if (current > len - 1) {
        list_jump_to_top(list);
        return;
    }

    list->current = current;

    if (current > start + list->len - 1) {
        rect_t rect;
        
        list->adapter.measure(
            list->adapter.data, current,
            true, &list->rect,
            &rect.w, &rect.h);

        rect.x = list->rects[list->len - 1].x;
        rect.y = list->rect.y + list->rect.h;

        list_rects_in_end(list, &rect);

        start++;
        list->start = start;
    }

    index = current - start;
    current_y = list->rects[index].y + list->rects[index].h;
    max_y = list->rect.y + list->rect.h;

    if (current_y > max_y) {
        list_rects_offset_y(list, max_y - current_y);
    } 
}

void list_up(list_t *list) {
    int start, current, len, index;

    start = list->start;
    current = list->current;
    len = list->adapter.len(list->adapter.data);

    current--;

    if (current < 0) {
        list_jump_to_bottom(list);
        return;
    }

    list->current = current;

    if (current < start) {
        rect_t rect;

        list->adapter.measure(
            list->adapter.data, current,
            true, &list->rect,
            &rect.w, &rect.h);

        rect.x = list->rects[0].x;
        rect.y = -rect.h;

        list_rects_in_front(list, &rect);

        start--;
        list->start = start;
    }
    
    index = current - start;
    if (list->rects[index].y < 0) {
        list_rects_offset_y(list, -list->rects[index].y);
        return;
    }
}
