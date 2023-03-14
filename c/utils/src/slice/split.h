#if !defined(__SPLIT_H__)
#define __SPLIT_H__

#include "slice.h"

typedef struct {
    slice_t s;
    slice_t sep;
    char c;
} split_t;

split_t split_new(slice_t in, char c);
slice_t split_next(split_t *split);

split_t split_new_ext(
    uint8_t *data, int data_len,
    const uint8_t *sep, int sep_len);
slice_t split_next_ext(split_t *split);

#endif // __SPLIT_H__
