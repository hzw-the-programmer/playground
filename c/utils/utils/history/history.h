#if !defined(__HISTORY_H__)
#define __HISTORY_H__
#include "../types.h"
#include "../utils.h"

#define HISTORY_FLAG_NO_HISTORY 0x01

typedef void OnCreate(void *self);
typedef void OnResume(void *self);
typedef void OnPause(void *self);
typedef void OnDestroy(void *self);
typedef void OnData(void *self);

typedef struct {
    void *self;
    unsigned int flags;
    OnCreate *on_create;
    OnResume *on_resume;
    OnPause *on_pause;
    OnDestroy *on_destroy;
    OnData *on_data;
} HistoryItem;

void history_start(HistoryItem item);
void history_clear_top(HistoryItem item);
void history_back();
void history_clear();
#endif
