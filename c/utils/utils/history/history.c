#include <assert.h>
#include "history.h"

#define HISTORY_LEN 10

static HistoryItem gHistoryItems[HISTORY_LEN];
static int gHistoryIndex = 0;

bool history_full() {
    return gHistoryIndex == ARRAY_SIZE(gHistoryItems);
}

bool history_empty() {
    return gHistoryIndex == 0;
}

void history_push(HistoryItem item) {
    assert(!history_full());
    gHistoryItems[gHistoryIndex++] = item;
}

HistoryItem history_pop() {
    assert(!history_empty());
    return gHistoryItems[--gHistoryIndex];
}

HistoryItem history_peek() {
    int index = 0;
    assert(!history_empty());
    index = gHistoryIndex - 1;
    return gHistoryItems[index];
}

int history_item_cmp(HistoryItem a, HistoryItem b) {
    if (a.self == b.self) {
        return 0;
    }
    return 1;
}

int history_item_index(HistoryItem item) {
    int i = 0;

    for (i = 0; i < gHistoryIndex; i++) {
        if (history_item_cmp(gHistoryItems[i], item) == 0) {
            return i;
        }
    }

    return -1;
}

void history_start(HistoryItem item) {
    HistoryItem top = {0};

    if (history_empty()) {
        item.on_create(item.self);
        history_push(item);
        item.on_resume(item.self);
        return;
    }

    top = history_peek();
    if (history_item_cmp(top, item) == 0) {
        top.on_data(top.self);
        return;
    }

    if (history_item_index(item) < 0) {
        item.on_create(item.self);
        top.on_pause(item.self);
        if (top.flags & HISTORY_FLAG_NO_HISTORY) {
            history_pop();
            top.on_destroy(top.self);
        }
        history_push(item);
        item.on_resume(item.self);
        return;
    }

    top.on_pause(top.self);
    history_pop();
    top.on_destroy(top.self);

    while (!history_empty()) {
        top = history_peek();

        if (history_item_cmp(top, item) == 0) {
            top.on_resume(top.self);
            return;
        }

        history_pop();
        top.on_destroy(top.self);
    }
}

void history_replace_before(HistoryItem item, HistoryItem nitem) {
    HistoryItem top = {0};

    if (history_empty() || history_item_index(item) < 0) {
        return;
    }

    top = history_peek();
    
    if (history_item_cmp(top, item) == 0) {
        return;
    }

    nitem.on_create(nitem.self);

    top.on_pause(top.self);
    history_pop();
    top.on_destroy(top.self);
    
    while (!history_empty()) {
            top = history_peek();
            
            if (history_item_cmp(top, item) == 0) {
                history_push(nitem);
                nitem.on_resume(nitem.self);
                return;
            }

            history_pop();
            top.on_destroy(top.self);
    }
}

void history_clear() {
    HistoryItem top = {0};
    
    if (history_empty()) {
        return;
    }

    top = history_peek();
    top.on_pause(top.self);
    history_pop();
    top.on_destroy(top.self);

    while (!history_empty()) {
        top = history_pop();
        top.on_destroy(top.self);
    }
}

void history_back() {
    HistoryItem top = {0};
    
    if (history_empty()) {
        return;
    }

    top = history_peek();
    top.on_pause(top.self);
    history_pop();
    top.on_destroy(top.self);

    if (history_empty()) {
        return;
    }

    top = history_peek();
    top.on_resume(top.self);
}