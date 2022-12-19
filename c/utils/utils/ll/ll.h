#include "../types.h"
#include "rect.h"

typedef struct ll_item_s ll_item;
typedef void (*ll_on_measure)(ll_item *item, int *w, int *h);

struct ll_item_s {
    rect_t rect;
    int weight;
    ll_on_measure on_measure;
};

typedef struct ll_group_s {
    rect_t rect;
    int weight;
    ll_on_measure on_measure;
} ll_group_t;

void ll_measure(ll_item *items, int len, int *pw, int *ph, int *pmax_w, int *pmax_h);

void ll_measure_group_horizontal(ll_item *item, int *w, int *h);