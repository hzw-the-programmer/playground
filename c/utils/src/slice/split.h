#if !defined(__SPLIT_H__)
#define __SPLIT_H__

#include "slice.h"

typedef struct {
    slice_t slice;
    slice_t sep;
} split_t;

split_t split_new(slice_t slice, slice_t sep);
slice_t split_next(split_t *split);

#endif // __SPLIT_H__
