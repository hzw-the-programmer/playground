#if !defined(__SPLIT_H__)
#define __SPLIT_H__

#include "slice.h"

typedef struct {
    slice_t s;
    char c;
} split_t;

split_t split_new(slice_t in, char c);
slice_t split_next(split_t *split);

#endif // __SPLIT_H__
