#include "utarray.h"
#include <assert.h>

typedef struct {
    int a;
    int b;
} intpair_t;

void utarray_test() {
    UT_array *pairs, *pairs_cpy;
    UT_icd pairicd = { sizeof(intpair_t),NULL,NULL,NULL};
    intpair_t it, *ip;
    int i;

    utarray_new(pairs, &pairicd);
    assert(0 == utarray_len(pairs));

    it.a = 1;
    it.b=2;
    utarray_push_back(pairs, &it);
    assert(1 == utarray_len(pairs));

    ip = (intpair_t*)utarray_back(pairs);
    assert(ip->a == 1 && ip->b == 2);

    utarray_pop_back(pairs);
    assert(0 == utarray_len(pairs));

    it.a = 3;
    it.b=4;
    utarray_push_back(pairs, &it);

    it.a = 5;
    it.b=6;
    utarray_push_back(pairs, &it);

    assert(2 == utarray_len(pairs));

    i = 0;
    ip=NULL;
    while( (ip=(intpair_t*)utarray_next(pairs,ip)) != NULL ) {
        if (i == 0) {
            assert(ip->a == 3 && ip->b == 4);
        } else if (i == 1) {
            assert(ip->a == 5 && ip->b == 6);
        } else {
            assert(0);
        }
        i++;
    }

    utarray_erase(pairs,0,1);
    assert(1 == utarray_len(pairs));

    while( (ip=(intpair_t*)utarray_next(pairs,ip)) != NULL ) {
        assert(ip->a == 5 && ip->b == 6);
    }

    it.a = 1;
    it.b=2;
    utarray_push_back(pairs, &it);

    i = 0;
    while( (ip=(intpair_t*)utarray_next(pairs,ip)) != NULL ) {
        if (i == 0) {
            assert(ip->a == 5 && ip->b == 6);
        } else if (i == 1) {
            assert(ip->a == 1 && ip->b == 2);
        } else {
            assert(0);
        }
        i++;
    }

    utarray_clear(pairs);
    assert(0 == utarray_len(pairs));

    utarray_extend_back(pairs);
    assert(1 == utarray_len(pairs));
   
    ip = (intpair_t*)utarray_back(pairs);
    assert((ip == (intpair_t*)utarray_front(pairs)));

    it.a = 1;
    it.b=2;
    utarray_push_back(pairs, &it);

    i = 0;
    ip=NULL;
    while( (ip=(intpair_t*)utarray_next(pairs,ip)) != NULL ) {
         if (i == 0) {
            assert(ip->a == 0 && ip->b == 0);
        } else if (i == 1) {
            assert(ip->a == 1 && ip->b == 2);
        } else {
            assert(0);
        }
        i++;
    }

    utarray_erase(pairs,1,1);
    assert(1 == utarray_len(pairs));

    while( (ip=(intpair_t*)utarray_next(pairs,ip)) != NULL ) {
        assert(ip->a == 0 && ip->b == 0);
    }

    it.a = 3;
    it.b=4;
    utarray_push_back(pairs, &it);

    i = 0;
    for(ip=(intpair_t*)utarray_front(pairs); ip!=NULL; ip=(intpair_t*)utarray_next(pairs,ip)) {
        if (i == 0) {
            assert(ip->a == 0 && ip->b == 0);
        } else if (i == 1) {
            assert(ip->a == 3 && ip->b == 4);
        } else {
            assert(0);
        }
        i++;
    }

    ip = (intpair_t*)utarray_back(pairs);
    assert(ip->a == 3 && ip->b == 4);

    utarray_new(pairs_cpy, &pairicd);
    utarray_concat(pairs_cpy, pairs);
    assert(2 == utarray_len(pairs_cpy));

    i = 0;
    ip=NULL;
    while( (ip=(intpair_t*)utarray_next(pairs_cpy,ip)) != NULL ) {
        if (i == 0) {
            assert(ip->a == 0 && ip->b == 0);
        } else if (i == 1) {
            assert(ip->a == 3 && ip->b == 4);
        } else {
            assert(0);
        }
        i++;
    }

    it.a=5;
    it.b=6;
    utarray_insert(pairs_cpy, &it, 0);
    assert(3 == utarray_len(pairs_cpy));

    i = 0;
    while( (ip=(intpair_t*)utarray_next(pairs_cpy,ip)) != NULL ) {
        if (i == 0) {
            assert(ip->a == 5 && ip->b == 6);
        } else if (i == 1) {
            assert(ip->a == 0 && ip->b == 0);
        } else if (i == 2) {
            assert(ip->a == 3 && ip->b == 4);
        } else {
            assert(0);
        }
        i++;
    }

    utarray_erase(pairs_cpy,0,2);
    assert(1 == utarray_len(pairs_cpy));

    while( (ip=(intpair_t*)utarray_next(pairs_cpy,ip)) != NULL ) {
        assert(ip->a == 3 && ip->b == 4);
    }

    utarray_inserta(pairs_cpy, pairs, 1);
    assert(3 == utarray_len(pairs_cpy));

    i = 0;
    while( (ip=(intpair_t*)utarray_next(pairs_cpy,ip)) != NULL ) {
        if (i == 0) {
            assert(ip->a == 3 && ip->b == 4);
        } else if (i == 1) {
            assert(ip->a == 0 && ip->b == 0);
        } else if (i == 2) {
            assert(ip->a == 3 && ip->b == 4);
        } else {
            assert(0);
        }
        i++;
    }

    utarray_free(pairs_cpy);
    assert(2 == utarray_len(pairs));

    utarray_resize(pairs, 30);
    assert(30 == utarray_len(pairs));

    i = 0;
    while( (ip=(intpair_t*)utarray_next(pairs,ip)) != NULL ) {
        if (i == 0) {
            assert(ip->a == 0 && ip->b == 0);
        } else if (i == 1) {
            assert(ip->a == 3 && ip->b == 4);
        } else {
            assert(ip->a == 0 && ip->b == 0);
        }
        i++;
    }
    assert(i == 30);

    utarray_resize(pairs, 1);
    assert(1 == utarray_len(pairs));

    utarray_resize(pairs, 0);
    assert(0 == utarray_len(pairs));

    utarray_free(pairs);
}
