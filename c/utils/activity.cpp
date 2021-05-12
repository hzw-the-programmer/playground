#include <string.h>
#include <assert.h>
#include <stdio.h>

#include "memory.h"

#define NULL 0

void activity_test_log(char *msg);

#define LOG(msg) activity_test_log(msg)
#define LOG_LIFE_CYCLE(fmt, id) \
    do {                                                   \
        char msg[100] = {0};                   \
        sprintf(msg, fmt, id);                   \
        LOG(msg);                                  \
    } while (0)

#define ACTIVITY_FLAG_NO_HISTORY 0x01

#define ACTIVITY_HISTORY_FLAG_CLEAR_TOP 0x01
#define ACTIVITY_HISTORY_FLAG_CLEAR_BOTTOM 0x02
#define ACTIVITY_HISTORY_FLAG_REPLACE_TOP 0x04

typedef enum
{
    ACTIVITY_ID_BEGIN,
    ACTIVITY_ID_A,
    ACTIVITY_ID_B,
    ACTIVITY_ID_C,
    ACTIVITY_ID_D,
    ACTIVITY_ID_END,
} ACTIVITY_ID;

typedef struct ACTIVITY ACTIVITY;

#define ACTIVITY_FIELDS                                                \
    ACTIVITY_ID id;                                                               \
    unsigned int flag;                                                               \
    void (*on_create)(ACTIVITY *activity, void *data);          \
    void (*on_resume)(ACTIVITY *activity);                          \
    void (*on_data)(ACTIVITY *activity, void *data);             \
    void (*on_pause)(ACTIVITY *activity);                            \
    void (*on_destroy)(ACTIVITY *activity);                          \
    void (*on_draw)(ACTIVITY *activity);                              \
    void (*on_handle_key_event)(ACTIVITY *activity, int key_code, int key_event)

struct ACTIVITY {
    ACTIVITY_FIELDS;
};

void activity_on_create(ACTIVITY *activity, void *data) {
    LOG_LIFE_CYCLE("%d_on_create", activity->id);
}

void activity_on_resume(ACTIVITY *activity) {
    LOG_LIFE_CYCLE("%d_on_resume", activity->id);
}

void activity_on_data(ACTIVITY *activity, void *data) {
    LOG_LIFE_CYCLE("%d_on_data", activity->id);
}

void activity_on_pause(ACTIVITY *activity) {
    LOG_LIFE_CYCLE("%d_on_pause", activity->id);
}

void activity_on_destroy(ACTIVITY *activity) {
    LOG_LIFE_CYCLE("%d_on_destroy", activity->id);
}

void activity_on_draw(ACTIVITY *activity) {
    LOG_LIFE_CYCLE("%d_on_draw", activity->id);
}

void activity_on_handle_key_event(ACTIVITY *activity, int key_code, int key_event) {
   LOG_LIFE_CYCLE("%d_on_handle_key_event", activity->id);
}

ACTIVITY* activity_new(
    int size,
    void (*on_create)(ACTIVITY *activity, void *data),
    void (*on_resume)(ACTIVITY *activity),
    void (*on_data)(ACTIVITY *activity, void *data),
    void (*on_pause)(ACTIVITY *activity),
    void (*on_destroy)(ACTIVITY *activity),
    void (*on_draw)(ACTIVITY *activity),
    void (*on_handle_key_event)(ACTIVITY *activity, int key_code, int key_event))
{
    ACTIVITY *activity = NULL;

    activity = (ACTIVITY*)HZW_MALLOC(size);

    if (activity)
    {
        activity->on_create = on_create ? on_create : activity_on_create;
        activity->on_resume = on_resume ? on_resume : activity_on_resume;
        activity->on_data = on_data ? on_data : activity_on_data;
        activity->on_pause = on_pause ? on_pause : activity_on_pause;
        activity->on_destroy = on_destroy ? on_destroy : activity_on_destroy;
        activity->on_draw = on_draw ? on_draw : activity_on_draw;
        activity->on_handle_key_event = on_handle_key_event ? on_handle_key_event : activity_on_handle_key_event;
    }

    return activity;
}

typedef struct ACTIVITY_HISTORY_NODE {
    ACTIVITY *activity;
    struct ACTIVITY_HISTORY_NODE *next;
} ACTIVITY_HISTORY_NODE;

static ACTIVITY_HISTORY_NODE *gActivityHistory;

void activity_history_push(ACTIVITY *activity) {
    ACTIVITY_HISTORY_NODE *node = NULL;

    node = (ACTIVITY_HISTORY_NODE*)HZW_MALLOC(sizeof(ACTIVITY_HISTORY_NODE));

    if (!node)
    {
        return;
    }

    node->activity = activity;
    node->next = gActivityHistory;
    gActivityHistory = node;
}

ACTIVITY* activity_history_pop() {
    ACTIVITY_HISTORY_NODE *node = NULL;
    ACTIVITY *activity = NULL;

    node = gActivityHistory;
    
    if (node) {
        activity = node->activity;
        gActivityHistory = node->next;
        HzwFree(node);
    }

    return activity;
}

ACTIVITY* activity_history_top() {
    return gActivityHistory ? gActivityHistory->activity : NULL;
}

ACTIVITY* activity_history_find(ACTIVITY_ID id) {
    ACTIVITY_HISTORY_NODE *node = NULL;

    node = gActivityHistory;

    while (node)  {
        if (node->activity->id == id) {
            return node->activity;
        }
        node = node->next;
    }

    return NULL;
}

ACTIVITY* activity_history_remove(ACTIVITY_ID id) {
    ACTIVITY_HISTORY_NODE **node = NULL;
    ACTIVITY_HISTORY_NODE *tmp = NULL;
    ACTIVITY *activity = NULL;

    node = &gActivityHistory;

    while (*node) {
        if ((*node)->activity->id == id) {
            tmp = *node;
            activity = tmp->activity;
            *node = tmp->next;
            HzwFree(tmp);
            break;
        }

        node = &((*node)->next);
    }

    return activity;
}

void activity_destroy_to(ACTIVITY_ID id) {
    ACTIVITY *top = NULL;
    ACTIVITY *activity = NULL;

    top = activity_history_top();
    activity = activity_history_find(id);

    while (top != activity) {
        activity_history_pop();
        top->on_destroy(top);
        HzwFree(top);

        top = activity_history_top();
    }
}

void activity_start(
    ACTIVITY_ID id,
    unsigned int flag,
    void *data,
    unsigned int start_flag,
    ACTIVITY* (*new_activity)())
{
    ACTIVITY *activity = NULL;
    ACTIVITY *top = NULL;

    activity = activity_history_find(id);

    if (!activity) {
        activity = new_activity();

        if (!activity) {
            return;
        }

        activity->id = id;
        activity->flag = flag;

        top = activity_history_top();

        if (top) {
            top->on_pause(top);
            if (top->flag & ACTIVITY_FLAG_NO_HISTORY
                || start_flag & ACTIVITY_HISTORY_FLAG_REPLACE_TOP) {
                activity_history_pop();
                top->on_destroy(top);
                HzwFree(top);
            }
        }

        if (start_flag & ACTIVITY_HISTORY_FLAG_CLEAR_BOTTOM) {
            activity_destroy_to(ACTIVITY_ID_BEGIN);
        }

        activity->on_create(activity, data);
        activity_history_push(activity);
        activity->on_resume(activity);

        return;
    }

    top = activity_history_top();

    if (activity == top) {
        activity->on_data(activity, data);
        return;
    }

    if (start_flag & ACTIVITY_HISTORY_FLAG_CLEAR_TOP) {
        top->on_pause(top);
        activity_destroy_to(activity->id);
        activity->on_data(activity, data);
        activity->on_resume(activity);
    }
}

void activity_finish(ACTIVITY_ID id) {
    ACTIVITY *activity = NULL;
    ACTIVITY *top = NULL;

    activity = activity_history_find(id);

    if (!activity) {
        return;
    }

    top = activity_history_top();

    if (activity != top) {
        activity_history_remove(id);
        activity->on_destroy(activity);
        HzwFree(activity);
        return;
    }

    top->on_pause(top);
    activity_history_pop();
    top->on_destroy(top);
    HzwFree(top);

    top = activity_history_top();
    if (top) {
        top->on_resume(top);
    }
}

void activity_clear() {
    ACTIVITY *activity = NULL;
    int top = 1;

    while (activity = activity_history_top()) {
        if (top) {
            activity->on_pause(activity);
            top = 0;
        }
        activity_history_pop();
        activity->on_destroy(activity);
        HzwFree(activity);
    }
}

void activity_back() {
    ACTIVITY *top = NULL;

    top = activity_history_top();
    
    if (top) {
        activity_finish(top->id);
    }
}

typedef struct {
    ACTIVITY_FIELDS;
} ACTIVITY_A;

ACTIVITY* activity_a_new()
{
    ACTIVITY *activity = NULL;

    activity = activity_new(
        sizeof(ACTIVITY_A),
        NULL,
        NULL,
        NULL,
        NULL,
        NULL,
        NULL,
        NULL);

    return activity;
}

typedef struct {
    ACTIVITY_FIELDS;
} ACTIVITY_B;

ACTIVITY* activity_b_new()
{
    ACTIVITY *activity = NULL;

    activity = activity_new(
        sizeof(ACTIVITY_B),
        NULL,
        NULL,
        NULL,
        NULL,
        NULL,
        NULL,
        NULL);

    return activity;
}

typedef struct {
    ACTIVITY_FIELDS;
} ACTIVITY_C;

ACTIVITY* activity_c_new()
{
    ACTIVITY *activity = NULL;

    activity = activity_new(
        sizeof(ACTIVITY_C),
        NULL,
        NULL,
        NULL,
        NULL,
        NULL,
        NULL,
        NULL);

    return activity;
}

typedef struct {
    ACTIVITY_FIELDS;
} ACTIVITY_D;

ACTIVITY* activity_d_new()
{
    ACTIVITY *activity = NULL;

    activity = activity_new(
        sizeof(ACTIVITY_D),
        NULL,
        NULL,
        NULL,
        NULL,
        NULL,
        NULL,
        NULL);

    return activity;
}

typedef struct {
    ACTIVITY_ID id;
    unsigned int flag;
    void *data;
    unsigned int start_flag;
    ACTIVITY* (*new_activity)();
} ACTIVITY_TEST_START_PARAM;

#define ACTIVITY_TEST_LOG_LEN 1024

static char gActivityTestLog[ACTIVITY_TEST_LOG_LEN] = {0};

void activity_test_log(char *msg) {
    if (strlen(gActivityTestLog)) {
        strcat(gActivityTestLog, ",");
    }
    strcat(gActivityTestLog, msg);
}

void activity_test_start(
    ACTIVITY_TEST_START_PARAM *params,
    int len,
    char *result) {
    int i = 0;

    activity_clear();
    
    memset(gActivityTestLog, 0, ACTIVITY_TEST_LOG_LEN);

    for (i = 0; i < len; i++) {
        activity_start(
            params[i].id,
            params[i].flag,
            params[i].data,
            params[i].start_flag,
            params[i].new_activity);
    }

    assert(strcmp(gActivityTestLog, result) == 0);
}

void activity_test_back(
    int len,
    char *result) {
    int i = 0;

    memset(gActivityTestLog, 0, ACTIVITY_TEST_LOG_LEN);

    for (i = 0; i < len; i++)
    {
        activity_back();
    }

    assert(strcmp(gActivityTestLog, result) == 0);
}

void activity_test_1() {
   ACTIVITY_TEST_START_PARAM params[] =
    {
        {
            ACTIVITY_ID_A,
            0,
            NULL,
            0,
            activity_a_new,
        },
    };
    int len = sizeof(params) / sizeof(params[0]);

    activity_test_start(
        params,
        len,
        "1_on_create"
        ",1_on_resume");

    activity_test_back(
        len,
        "1_on_pause"
        ",1_on_destroy");

    assert(gActivityHistory == NULL);
}

void activity_test_2() {
   ACTIVITY_TEST_START_PARAM params[] =
    {
        {
            ACTIVITY_ID_A,
            0,
            NULL,
            0,
            activity_a_new,
        },
        {
            ACTIVITY_ID_B,
            0,
            NULL,
            0,
            activity_b_new,
        },
    };
    int len = sizeof(params) / sizeof(params[0]);

    activity_test_start(
        params,
        len,
        "1_on_create"
        ",1_on_resume"
        
        ",1_on_pause"
        ",2_on_create"
        ",2_on_resume");

    activity_test_back(
        len,
        "2_on_pause"
        ",2_on_destroy"
        ",1_on_resume"
        
        ",1_on_pause"
        ",1_on_destroy");

    assert(gActivityHistory == NULL);
}

void activity_test_3() {
   ACTIVITY_TEST_START_PARAM params[] =
    {
        {
            ACTIVITY_ID_A,
            0,
            NULL,
            0,
            activity_a_new,
        },
        {
            ACTIVITY_ID_B,
            0,
            NULL,
            0,
            activity_b_new,
        },
        {
            ACTIVITY_ID_A,
            0,
            NULL,
            ACTIVITY_HISTORY_FLAG_CLEAR_TOP,
            activity_a_new,
        },
    };
    int len = sizeof(params) / sizeof(params[0]);

    activity_test_start(
        params,
        len,
        "1_on_create"
        ",1_on_resume"
        
        ",1_on_pause"
        ",2_on_create"
        ",2_on_resume"
        
        ",2_on_pause"
        ",2_on_destroy"
        ",1_on_data"
        ",1_on_resume");

    activity_test_back(
        len,
        "1_on_pause"
        ",1_on_destroy");

    assert(gActivityHistory == NULL);
}

void activity_test_4() {
   ACTIVITY_TEST_START_PARAM params[] =
    {
        {
            ACTIVITY_ID_A,
            0,
            NULL,
            0,
            activity_a_new,
        },
        {
            ACTIVITY_ID_B,
            0,
            NULL,
            0,
            activity_b_new,
        },
        {
            ACTIVITY_ID_C,
            0,
            NULL,
            0,
            activity_c_new,
        },
        {
            ACTIVITY_ID_D,
            0,
            NULL,
            0,
            activity_d_new,
        },
         {
            ACTIVITY_ID_A,
            0,
            NULL,
            ACTIVITY_HISTORY_FLAG_CLEAR_TOP,
            activity_a_new,
        },
    };
    int len = sizeof(params) / sizeof(params[0]);

    activity_test_start(
        params,
        len,
        "1_on_create"
        ",1_on_resume"
        
        ",1_on_pause"
        ",2_on_create"
        ",2_on_resume"
        
        ",2_on_pause"
        ",3_on_create"
        ",3_on_resume"

        ",3_on_pause"
        ",4_on_create"
        ",4_on_resume"
        
        ",4_on_pause"
        ",4_on_destroy"
        ",3_on_destroy"
        ",2_on_destroy"
        ",1_on_data"
        ",1_on_resume");

    activity_test_back(
        len,
        "1_on_pause"
        ",1_on_destroy");

    assert(gActivityHistory == NULL);
}

void activity_test_5() {
   ACTIVITY_TEST_START_PARAM params[] =
    {
        {
            ACTIVITY_ID_A,
            0,
            NULL,
            0,
            activity_a_new,
        },
        {
            ACTIVITY_ID_B,
            0,
            NULL,
            0,
            activity_b_new,
        },
        {
            ACTIVITY_ID_C,
            0,
            NULL,
            0,
            activity_c_new,
        },
        {
            ACTIVITY_ID_D,
            0,
            NULL,
            0,
            activity_d_new,
        },
         {
            ACTIVITY_ID_B,
            0,
            NULL,
            ACTIVITY_HISTORY_FLAG_CLEAR_TOP,
            activity_b_new,
        },
    };
    int len = sizeof(params) / sizeof(params[0]);

    activity_test_start(
        params,
        len,
        "1_on_create"
        ",1_on_resume"
        
        ",1_on_pause"
        ",2_on_create"
        ",2_on_resume"
        
        ",2_on_pause"
        ",3_on_create"
        ",3_on_resume"

        ",3_on_pause"
        ",4_on_create"
        ",4_on_resume"
        
        ",4_on_pause"
        ",4_on_destroy"
        ",3_on_destroy"
        ",2_on_data"
        ",2_on_resume");

    activity_test_back(
        len,
        "2_on_pause"
        ",2_on_destroy"
        ",1_on_resume"
        
        ",1_on_pause"
        ",1_on_destroy");

    assert(gActivityHistory == NULL);
}

void activity_test_6() {
   ACTIVITY_TEST_START_PARAM params[] =
    {
        {
            ACTIVITY_ID_A,
            0,
            NULL,
            0,
            activity_a_new,
        },
        {
            ACTIVITY_ID_B,
            0,
            NULL,
            0,
            activity_b_new,
        },
        {
            ACTIVITY_ID_C,
            0,
            NULL,
            0,
            activity_c_new,
        },
        {
            ACTIVITY_ID_D,
            0,
            NULL,
            ACTIVITY_HISTORY_FLAG_CLEAR_BOTTOM,
            activity_d_new,
        },
    };
    int len = sizeof(params) / sizeof(params[0]);

    activity_test_start(
        params,
        len,
        "1_on_create"
        ",1_on_resume"
        
        ",1_on_pause"
        ",2_on_create"
        ",2_on_resume"
        
        ",2_on_pause"
        ",3_on_create"
        ",3_on_resume"

        ",3_on_pause"
        ",3_on_destroy"
        ",2_on_destroy"
        ",1_on_destroy"
        ",4_on_create"
        ",4_on_resume");

    activity_test_back(
        len,
        "4_on_pause"
        ",4_on_destroy");

    assert(gActivityHistory == NULL);
}

void activity_test_7() {
   ACTIVITY_TEST_START_PARAM params[] =
    {
        {
            ACTIVITY_ID_A,
            0,
            NULL,
            0,
            activity_a_new,
        },
        {
            ACTIVITY_ID_B,
            0,
            NULL,
            0,
            activity_b_new,
        },
        {
            ACTIVITY_ID_C,
            0,
            NULL,
            0,
            activity_c_new,
        },
        {
            ACTIVITY_ID_D,
            0,
            NULL,
            ACTIVITY_HISTORY_FLAG_REPLACE_TOP,
            activity_d_new,
        },
    };
    int len = sizeof(params) / sizeof(params[0]);

    activity_test_start(
        params,
        len,
        "1_on_create"
        ",1_on_resume"
        
        ",1_on_pause"
        ",2_on_create"
        ",2_on_resume"
        
        ",2_on_pause"
        ",3_on_create"
        ",3_on_resume"

        ",3_on_pause"
        ",3_on_destroy"
        ",4_on_create"
        ",4_on_resume");

    activity_test_back(
        len,
        "4_on_pause"
        ",4_on_destroy"
        ",2_on_resume"
        
        ",2_on_pause"
        ",2_on_destroy"
        ",1_on_resume"
        
        ",1_on_pause"
        ",1_on_destroy");

    assert(gActivityHistory == NULL);
}

void activity_test_8() {
   ACTIVITY_TEST_START_PARAM params[] =
    {
        {
            ACTIVITY_ID_A,
            0,
            NULL,
            0,
            activity_a_new,
        },
        {
            ACTIVITY_ID_B,
            0,
            NULL,
            0,
            activity_b_new,
        },
        {
            ACTIVITY_ID_C,
            ACTIVITY_FLAG_NO_HISTORY,
            NULL,
            0,
            activity_c_new,
        },
        {
            ACTIVITY_ID_D,
            0,
            NULL,
            0,
            activity_d_new,
        },
    };
    int len = sizeof(params) / sizeof(params[0]);

    activity_test_start(
        params,
        len,
        "1_on_create"
        ",1_on_resume"
        
        ",1_on_pause"
        ",2_on_create"
        ",2_on_resume"
        
        ",2_on_pause"
        ",3_on_create"
        ",3_on_resume"

        ",3_on_pause"
        ",3_on_destroy"
        ",4_on_create"
        ",4_on_resume");

    activity_test_back(
        len,
        "4_on_pause"
        ",4_on_destroy"
        ",2_on_resume"
        
        ",2_on_pause"
        ",2_on_destroy"
        ",1_on_resume"
        
        ",1_on_pause"
        ",1_on_destroy");

    assert(gActivityHistory == NULL);
}

void test_activity() {
    activity_test_1();
    activity_test_2();
    activity_test_3();
    activity_test_4();
    activity_test_5();
    activity_test_6();
    activity_test_7();
    activity_test_8();
}
