#include "../types.h"

typedef struct ll_item_s ll_item;
typedef void (*ll_on_measure)(ll_item *item, int *w, int *h);

struct ll_item_s {
    int x, y, w, h, weight;
    int index, len;
    ll_on_measure on_measure;
    void *data;
};

void ll_measure(ll_item *items, int len, int *pw, int *ph, int *pmax_w, int *pmax_h);

void ll_measure_group_horizontal(ll_item *item, int *w, int *h);