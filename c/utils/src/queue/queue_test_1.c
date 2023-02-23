#include "queue.h"
#define NULL 0

struct item_s {
    LIST_ENTRY(item_s) item;
    int i;
};

LIST_HEAD(head_s, item_s);

void test_queue_1() {
    struct head_s head;
    struct item_s item0 = {{1, NULL}, 0};
    struct item_s item1 = {{NULL, NULL}, 1};

#if 0
    LIST_INIT(&head);
    LIST_INSERT_HEAD(head, &item0, item);
    assert(item0.item.le_next == NULL);
    assert(head.lh_first ==);
#endif
}