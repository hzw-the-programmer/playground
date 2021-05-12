#include <sys/types.h>

typedef struct pri_que {
    size_t head;
    size_t nelem;
    size_t elsize;
    void *elems;
    int (*less)(void*, void*);
} pri_que_t;
