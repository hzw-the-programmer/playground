#include <assert.h>
#include <string.h>

#include "memory.h"

typedef struct action_s action_t;
typedef struct state_s state_t;
typedef struct reducer_node_s reducer_node_t;
typedef struct subscriber_node_s subscriber_node_t;

typedef int reducer_t(state_t*, action_t);
typedef void subscriber_t(state_t*);

typedef enum {
    ACTION_TYPE_UNKNOWN,
    ACTION_TYPE_1,
    ACTION_TYPE_2,
    ACTION_TYPE_3,
    ACTION_TYPE_4,
    ACTION_TYPE_5,
    ACTION_TYPE_MAX,
} action_type;

struct action_s {
    action_type type;
    void *payload;
};

struct state_s {
    int count;
};

struct reducer_node_s {
    reducer_t *reducer;
    reducer_node_t *next;
};

struct subscriber_node_s {
    subscriber_t *subscriber;
    void *data;
    reducer_t *reducer;
    subscriber_node_t *next;
};

reducer_node_t *reducer_header = 0;
subscriber_node_t *subscriber_header = 0;
state_t state = {0};

int register_reducer(reducer_t *reducer) {
    reducer_node_t *node = 0;
    reducer_node_t **nodep = &reducer_header;

    node = (reducer_node_t*)HZW_MALLOC(sizeof(reducer_node_t));
    
    if (!node) {
        return 0;
    }

    node->reducer = reducer;
    node->next = 0;

    while(*nodep) {
        nodep = &(*nodep)->next;
    }
    *nodep = node;

    return 1;
}

int unregister_reducer(reducer_t *reducer) {
    reducer_node_t *node = 0;
    reducer_node_t **nodep = &reducer_header;

    while (*nodep) {
        if ((*nodep)->reducer == reducer) {
            node =  *nodep;
            *nodep = node->next;
            HzwFree(node);
            return 1;
        }
        nodep = &(*nodep)->next;
    }

    return 0;
}

int register_subscriber(subscriber_t *subscriber, void *data, reducer_t *reducer) {
    subscriber_node_t *node = 0;
    subscriber_node_t **nodep = &subscriber_header;

    node = (subscriber_node_t*)HZW_MALLOC(sizeof(subscriber_node_t));

    if (!node) {
        return 0;
    }

    node->subscriber = subscriber;
    node->data = data;
    node->reducer = reducer;
    node->next = 0;

    while (*nodep) {
        nodep = &(*nodep)->next;
    }
    *nodep = node;
}

int unregister_subscriber(subscriber_t *subscriber, void *data, reducer_t *reducer) {
    subscriber_node_t *node = 0;
    subscriber_node_t **nodep = &subscriber_header;

    while (*nodep) {
        if ((*nodep)->subscriber == subscriber
            && (*nodep)->data == data
            && (*nodep)->reducer == reducer) {
            node = *nodep;
            *nodep = node->next;
            HzwFree(node);
            return 1;
        }
        nodep = &(*nodep)->next;
    }

    return 0;
}

void dispatch(action_t action) {
    reducer_node_t *reducer_node;
    reducer_t *reducer;
    subscriber_node_t *subscriber_node;
    int changed = 0;

    reducer_node = reducer_header;
    while (reducer_node) {
        reducer = reducer_node->reducer;
        if (reducer(&state, action)) {
            changed = 1;
            subscriber_node = subscriber_header;
            while (subscriber_node) {
                if (subscriber_node->reducer == reducer) {
                    subscriber_node->subscriber(&state);
                }
                subscriber_node = subscriber_node->next;
            }
        }
        reducer_node = reducer_node->next;
    }

    if (changed) {
            subscriber_node = subscriber_header;
            while (subscriber_node) {
                if (!subscriber_node->reducer) {
                    subscriber_node->subscriber(&state);
                }
                subscriber_node = subscriber_node->next;
            }
    }
}

#define TEST_LOG_LEN 1024

char test_log[TEST_LOG_LEN] = {0};

void test_log_print(char *msg) {
    if (strlen(test_log)) {
        strcat(test_log, ",");
    }
    strcat(test_log, msg);
}

void test_log_clear() {
    memset(test_log, 0, TEST_LOG_LEN);
}

void test_log_assert(char *msg) {
    assert(strcmp(test_log, msg) == 0);
}

int test_reducer_1(state_t *state, action_t action) {
    switch (action.type) {
        case ACTION_TYPE_1:
            test_log_print("reducer_1 1");
            return 1;
        default:
            test_log_print("reducer_1 0");
            return 0;
    }
}

int test_reducer_2(state_t *state, action_t action) {
    switch (action.type) {
        case ACTION_TYPE_2:
            test_log_print("reducer_2 1");
            return 1;
        default:
            test_log_print("reducer_2 0");
            return 0;
    }
}

int test_reducer_3(state_t *state, action_t action) {
    switch (action.type) {
        case ACTION_TYPE_3:
            test_log_print("reducer_3 1");
            return 1;
        default:
            test_log_print("reducer_3 0");
            return 0;
    }
}

int test_reducer_4(state_t *state, action_t action) {
    switch (action.type) {
        case ACTION_TYPE_4:
            test_log_print("reducer_4 1");
            return 1;
        default:
            test_log_print("reducer_4 0");
            return 0;
    }
}

void test_register_reducer() {
    reducer_t* reducers[] = {
        test_reducer_1, test_reducer_2, test_reducer_3, test_reducer_4,
    };
    int i, len;
    reducer_node_t *node;

    len = sizeof(reducers) / sizeof(reducers[0]);
    for (i = 0; i < len; i++) {
        register_reducer(reducers[i]);
    }

    for (i = 0, node = reducer_header; i < len; i++, node = node->next) {
        assert(reducers[i] == node->reducer);
    }
    assert(!node);

    for (i = 0; i < len; i++) {
        unregister_reducer(reducers[i]);
    }
    assert(!reducer_header);

    for (i = 0; i < len; i++) {
        register_reducer(reducers[i]);
    }

    for (i = 0, node = reducer_header; i < len; i++, node = node->next) {
        assert(reducers[i] == node->reducer);
    }
    assert(!node);

    for (i = len - 1; i >=0 ; i--) {
        unregister_reducer(reducers[i]);
    }
    assert(!reducer_header);
}

void test_subscriber_1(state_t *state) {
    test_log_print("subscriber_1");
}

void test_subscriber_2(state_t *state) {
    test_log_print("subscriber_2");
}

void test_subscriber_3(state_t *state) {
    test_log_print("subscriber_3");
}

void test_subscriber_4(state_t *state) {
    test_log_print("subscriber_4");
}

void test_subscriber_5(state_t *state) {
    test_log_print("subscriber_5");
}

void test_subscriber_6(state_t *state) {
    test_log_print("subscriber_6");
}

typedef struct {
    subscriber_t *subscriber;
    void *data;
    reducer_t *reducer;
} test_subscriber_t;

void test_register_subscriber() {
    test_subscriber_t subscribers[] = {
        {test_subscriber_1, 0, test_reducer_1},
        {test_subscriber_2, 0, test_reducer_2},
        {test_subscriber_3, 0, test_reducer_3},
        {test_subscriber_4, 0, test_reducer_4},
        {test_subscriber_5, 0, 0},
    };
    int i, len;
    subscriber_node_t *node;

    len = sizeof(subscribers) / sizeof(subscribers[0]);
    for (i = 0; i < len; i++) {
        register_subscriber(subscribers[i].subscriber, subscribers[i].data, subscribers[i].reducer);
    }

    for (i = 0, node = subscriber_header; i < len; i++, node = node->next) {
        assert(subscribers[i].subscriber == node->subscriber);
        assert(subscribers[i].data == node->data);
        assert(subscribers[i].reducer == node->reducer);
    }
    assert(!node);

    for (i = 0; i < len; i++) {
        unregister_subscriber(subscribers[i].subscriber, subscribers[i].data, subscribers[i].reducer);
    }
    assert(!subscriber_header);

    for (i = 0; i < len; i++) {
        register_subscriber(subscribers[i].subscriber, subscribers[i].data, subscribers[i].reducer);
    }

    for (i = 0, node = subscriber_header; i < len; i++, node = node->next) {
        assert(subscribers[i].subscriber == node->subscriber);
        assert(subscribers[i].data == node->data);
        assert(subscribers[i].reducer == node->reducer);
    }
    assert(!node);

    for (i = len - 1; i >=0 ; i--) {
        unregister_subscriber(subscribers[i].subscriber, subscribers[i].data, subscribers[i].reducer);
    }
    assert(!subscriber_header);
}

void test_dispatch() {
    reducer_t* reducers[] = {
        test_reducer_1, test_reducer_2, test_reducer_3, test_reducer_4,
    };
    test_subscriber_t subscribers[] = {
        {test_subscriber_1, 0, test_reducer_1},
        {test_subscriber_2, 0, test_reducer_2},
        {test_subscriber_3, 0, test_reducer_3},
        {test_subscriber_4, 0, test_reducer_4},
        {test_subscriber_5, 0, 0},
    };
     int i, len;
    action_t action;

    len = sizeof(reducers) / sizeof(reducers[0]);
    for (i = 0; i < len; i++) {
        register_reducer(reducers[i]);
    }

    len = sizeof(subscribers) / sizeof(subscribers[0]);
    for (i = 0; i < len; i++) {
        register_subscriber(subscribers[i].subscriber, subscribers[i].data, subscribers[i].reducer);
    }

    test_log_clear();
    action.type = ACTION_TYPE_1;
    dispatch(action);
    test_log_assert(
        "reducer_1 1"
        ",subscriber_1"
        ",reducer_2 0"
        ",reducer_3 0"
        ",reducer_4 0"
        ",subscriber_5");

    test_log_clear();
    action.type = ACTION_TYPE_2;
    dispatch(action);
    test_log_assert(
        "reducer_1 0"
        ",reducer_2 1"
        ",subscriber_2"
        ",reducer_3 0"
        ",reducer_4 0"
        ",subscriber_5");

    test_log_clear();
    action.type = ACTION_TYPE_3;
    dispatch(action);
    test_log_assert(
        "reducer_1 0"
        ",reducer_2 0"
        ",reducer_3 1"
        ",subscriber_3"
        ",reducer_4 0"
        ",subscriber_5");

    test_log_clear();
    action.type = ACTION_TYPE_4;
    dispatch(action);
    test_log_assert(
        "reducer_1 0"
        ",reducer_2 0"
        ",reducer_3 0"
        ",reducer_4 1"
        ",subscriber_4"
        ",subscriber_5");

    test_log_clear();
    action.type = ACTION_TYPE_5;
    dispatch(action);
    test_log_assert(
        "reducer_1 0"
        ",reducer_2 0"
        ",reducer_3 0"
        ",reducer_4 0");

    len = sizeof(reducers) / sizeof(reducers[0]);
    for (i = 0; i < len; i++) {
        unregister_reducer(reducers[i]);
    }
    assert(!reducer_header);

    len = sizeof(subscribers) / sizeof(subscribers[0]);
    for (i = len - 1; i >=0 ; i--) {
        unregister_subscriber(subscribers[i].subscriber, subscribers[i].data, subscribers[i].reducer);
    }
    assert(!subscriber_header);
}

void test_redux1() {
    test_register_reducer();
    test_register_subscriber();
    test_dispatch();
}
