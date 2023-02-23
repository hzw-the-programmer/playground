#include <stdlib.h>
#include <assert.h>

typedef int heap_elem_t;

struct heap_s {
    size_t len;
    size_t alloc;
    heap_elem_t array[];
};

static struct heap_s* heap_new(size_t alloc) {
    struct heap_s *h;
    size_t hs = sizeof(*h);
    size_t es = sizeof(h->array[0]);

    h = malloc(hs + es * alloc);
    if (h) {
        h->len = 0;
        h->alloc = alloc;
    }

    return h;
}

static void heap_insert(struct heap_s *h, heap_elem_t e) {
    size_t c, p;
    heap_elem_t t;

    c = h->len++;    
    h->array[c] = e;

    while (c > 0) {
        p = (c - 1) / 2;
        if (h->array[c] < h->array[p]) {
            t = h->array[c];
            h->array[c] = h->array[p];
            h->array[p] = t;
            c = p;
        }
    }
}

static heap_elem_t heap_extract(struct heap_s *h) {
    size_t p, lc, rc, m;
    heap_elem_t r, t;

    r = h->array[0];
    h->array[0] = h->array[--h->len];

    p = 0;

    while (1) {
        lc = 2 * p + 1;
        rc = 2 *p + 2;

        m = p;
        if (lc < h->len && h->array[m] > h->array[lc]) {
            m = lc;
        }
        if (rc < h->len && h->array[m] > h->array[rc]) {
            m = rc;
        }

        if (m == p) {
            return r;
        }

        t = h->array[p];
        h->array[p] = h->array[m];
        h->array[m] = t;
        p = m;
    }
}

#define ARRAY_SIZE(a) (sizeof(a)/sizeof(a[0]))

static void test_heap_1() {
    struct heap_s *h;
    //size_t i;
    int i;
    heap_elem_t es[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
    
    h = heap_new(ARRAY_SIZE(es));

    for (i = ARRAY_SIZE(es) - 1; i >= 0; i--) {
        heap_insert(h, es[i]);
    }

    for (i = 0; i < ARRAY_SIZE(es); i++) {
        assert(es[i] == heap_extract(h));
    }

    free(h);
}

void test_heap() {
    test_heap_1();
}