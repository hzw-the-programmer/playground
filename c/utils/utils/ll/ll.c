#include "ll.h"

void ll_measure(ll_item *items, int len, int *pw, int *ph, int *pmax_w, int *pmax_h) {
#if 0
    int w = 0, h = 0, max_w = 0, max_h = 0;
    ll_item *item = items;

    while (item < items + len) {
        if (item->w == 0 && item->h == 0 && item->on_measure != NULL) {
            item->on_measure(item, &item->w, &item->h);
        }
        
        if (item->w > max_w) max_w = item->w;
        if (item->h > max_h) max_h = item->h;
        
        w += item->w;
        h += item->h;
        
        item++;
    }

    if (pw) *pw = w;
    if (ph) *ph = h;
    if (pmax_w) *pmax_w = max_w;
    if (pmax_h) *pmax_h = max_h;
#endif
}

void ll_measure_group_horizontal(ll_item *item, int *w, int *h) {

}