#if !defined(__LIST_H__)
#define __LIST_H__

#include "rect.h"
#include "../types.h"

typedef struct {
    void *data;
    int (*len)(void *data);
    void (*measure)(void *data, int index, bool focused, const rect_t *rect, int *w, int *h);
} list_adapter_t;

typedef struct {
    rect_t rect;
    rect_t *rects;
    int len;
    list_adapter_t adapter;
    int offset;
    int start, current;
} list_t;

list_t list_new(const rect_t *rect, rect_t *rects, int len, const list_adapter_t *adapter);
void list_measure(list_t *list);
void list_layout(list_t *list);

#endif
