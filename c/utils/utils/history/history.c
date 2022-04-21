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
        item.on_resume(item.self);
        history_push(item);
        return;
    }

    top = history_peek();
    if (history_item_cmp(top, item) == 0) {
        top.on_data(top.self);
        return;
    }

    if (history_item_index(item) < 0) {
        item.on_create(item.self);
        top = history_peek();
        top.on_pause(item.self);
        if (top.flags & HISTORY_FLAG_NO_HISTORY) {
            top.on_destroy(top.self);
            history_pop();
        }
        item.on_resume(item.self);
        history_push(item);
        return;
    }

    while (!history_empty()) {
        top = history_peek();

        if (history_item_cmp(top, item) == 0) {
            top.on_resume(top.self);
            return;
        }

        top.on_pause(top.self);
        top.on_destroy(top.self);
        history_pop();
    }
}

void history_clear() {
    while (!history_empty()) {
        HistoryItem item = history_pop();
        item.on_pause(item.self);
        item.on_destroy(item.self);
    }
}

void history_back() {
    HistoryItem top = {0};
    
    if (history_empty()) {
        return;
    }

    top = history_pop();
    top.on_pause(top.self);
    top.on_destroy(top.self);

    if (history_empty()) {
        return;
    }

    top = history_peek();
    top.on_resume(top.self);
}