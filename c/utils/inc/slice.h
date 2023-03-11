#if !defined(__SLICE_H__)
#define __SLICE_H__

#include "types.h"

typedef struct {
    char *data;
    int len;
} slice_t;

slice_t slice_new(char *data, int len);

slice_t slice_sub(slice_t in, int begin, int end);
int slice_search(slice_t s, char b);
slice_t slice_slice(slice_t ss, slice_t s);

slice_t slice_ltrim(slice_t s, slice_t cutset);
slice_t slice_rtrim(slice_t s, slice_t cutset);
slice_t slice_trim(slice_t s, slice_t cutset);
slice_t slice_ltrim_space(slice_t s);
slice_t slice_rtrim_space(slice_t s);
slice_t slice_trim_space(slice_t s);
uint64 slice_to_uint64(slice_t s);

#endif // __SLICE_H__
