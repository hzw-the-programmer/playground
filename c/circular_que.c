#include <sys/types.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <stdio.h>

#include "circular_que.h"

//#define TEST

circular_que_t* create_circular_que(size_t nelem, size_t elsize) {
    circular_que_t *que;

    que = malloc(sizeof(circular_que_t));
    if (que == NULL) {
        return NULL;
    }

    que->head = 0;
    que->tail = 0;
    que->nelem = nelem;
    que->elsize = elsize;
    que->data = calloc(nelem, elsize);
    if (que->data == NULL) {
        free(que);
        return NULL;
    }
    
    return que;
}

void destroy_circular_que(circular_que_t *que) {
    free(que->data);
    free(que);
}

int circular_que_is_empty(circular_que_t *que) {
    return que->head == que->tail;
}

int circular_que_is_full(circular_que_t *que) {
    return (que->tail + 1) % que->nelem == que->head;
}

int circular_que_push(circular_que_t *que, void *elem) {
    if (circular_que_is_full(que)) {
        return -1;
    }

    memcpy(que->data + que->tail * que->elsize, elem, que->elsize);
    que->tail = (que->tail + 1) % que->nelem;

    return que->tail;
}

int circular_que_pop(circular_que_t *que, void *elem) {
    if (circular_que_is_empty(que)) {
        return -1;
    }

    memcpy(elem, que->data + que->head * que->elsize, que->elsize);
    que->head = (que->head + 1) % que->nelem;

    return que->head;
}

#ifdef TEST

struct elem {
    char a;
    short b;
    int c;
    float d;
};

int main(int argc, char *argv[]) {
    circular_que_t *que;
    struct elem e1, e2, e3;
    int res;

    e1.a = 'A';
    e1.b = 1;
    e1.c = 2;
    e1.d = 1.23;

    e2.a = 'B';
    e2.b = 11;
    e2.c = 22;
    e2.d = 32.1;

    e3.a = 'C';
    e3.b = 111;
    e3.c = 222;
    e3.d = 0.5;

    que = create_circular_que(3, sizeof(struct elem));
    
    res = circular_que_push(que, &e1);
    assert(1 == res);
    
    res = circular_que_push(que, &e2);
    assert(2 == res);
    
    res = circular_que_push(que, &e3);
    assert(-1 == res);
    
    res = circular_que_pop(que, &e3);
    assert(1 == res);
    assert('A' == e3.a);
    assert(1 == e3.b);
    assert(2 == e3.c);

    res = circular_que_pop(que, &e3);
    assert(2 == res);
    assert('B' == e3.a);
    assert(11 == e3.b);
    assert(22 == e3.c);

    res = circular_que_pop(que, &e3);
    assert(-1 == res);
    assert('B' == e3.a);
    assert(11 == e3.b);
    assert(22 == e3.c);

    destroy_circular_que(que);
}

#endif
