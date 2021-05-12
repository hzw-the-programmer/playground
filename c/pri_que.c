#include <stdlib.h>
#include <assert.h>
#include <string.h>

#include "pri_que.h"

#define TEST

pri_que_t* create_pri_que(size_t nelem, size_t elsize, int (*less)(void*, void*)) {
    pri_que_t *que = malloc(sizeof(pri_que_t));

    if (que == NULL) {
        return NULL;
    }

    que->head = 0;
    que->nelem = nelem;
    que->elsize = elsize;
    
    que->elems = calloc(nelem, elsize);
    if (que->elems == NULL) {
        free(que);
        return NULL;
    }

    que->less = less;

    return que;
}

void destroy_pri_que(pri_que_t *que) {
    free(que->elems);
    free(que);
}

int pri_que_is_empty(pri_que_t *que) {
    return que->head == 0;
}

int pri_que_is_full(pri_que_t *que) {
    return que->head == que->nelem;
}

int pri_que_insert(pri_que_t *que, void *elem) {
    if (pri_que_is_full(que)) {
        return 0;
    }

    memcpy(que->elems + que->elsize * que->head, elem, que->elsize);
    que->head++;

    return 1;
}

int pri_que_delete(pri_que_t *que, void *elem) {
    if (pri_que_is_empty(que)) {
        return 0;
    }

    void *max = que->elems;
    for (size_t i = 1; i < que->head; i++) {
        if (que->less(max, que->elems + que->elsize * i)) {
            max = que->elems + que->elsize * i;
        }
    }
    
    memcpy(elem, max, que->elsize);

    que->head--;
    memcpy(max, que->elems + que->elsize * que->head, que->elsize);

    return 1;
}

#ifdef TEST

typedef struct place {
    int id;
    char *name;
} place_t;

int less(void *_p1, void *_p2) {
    place_t *p1 = _p1;
    place_t *p2 = _p2;

    if (p1->id < p2->id) {
        return 1;
    }

    return 0;
}

int main(int argc, char *argv[]) {
    pri_que_t *que = create_pri_que(10, sizeof(place_t), less);
    place_t p;

    assert(pri_que_is_empty(que));

    p.id = 0;
    p.name = "P5";
    pri_que_insert(que, &p);

    p.id = 2;
    p.name = "P5";
    pri_que_insert(que, &p);

    p.id = 1;
    p.name = "P5";
    pri_que_insert(que, &p);

    p.id = 4;
    p.name = "P5";
    pri_que_insert(que, &p);

    p.id = 9;
    p.name = "P1";
    pri_que_insert(que, &p);

    p.id = 5;
    p.name = "P2";
    pri_que_insert(que, &p);

    p.id = 8;
    p.name = "P3";
    pri_que_insert(que, &p);

    p.id = 6;
    p.name = "P4";
    pri_que_insert(que, &p);

    p.id = 3;
    p.name = "P5";
    pri_que_insert(que, &p);

    p.id = 7;
    p.name = "P5";
    pri_que_insert(que, &p);

    assert(pri_que_is_full(que));

    pri_que_delete(que, &p);
    assert(9 == p.id);

    pri_que_delete(que, &p);
    assert(8 == p.id);

    pri_que_delete(que, &p);
    assert(7 == p.id);

    pri_que_delete(que, &p);
    assert(6 == p.id);
    
    pri_que_delete(que, &p);
    assert(5 == p.id);

    pri_que_delete(que, &p);
    assert(4 == p.id);

    pri_que_delete(que, &p);
    assert(3 == p.id);

    pri_que_delete(que, &p);
    assert(2 == p.id);

    pri_que_delete(que, &p);
    assert(1 == p.id);

    pri_que_delete(que, &p);
    assert(0 == p.id);

    assert(pri_que_is_empty(que));

    destroy_pri_que(que);
}

#endif
