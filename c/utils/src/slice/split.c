#include "split.h"

split_t split_new(slice_t slice, slice_t sep) {
    split_t split;

    split.slice = slice;
    split.sep = sep;

    return split;
}

slice_t split_next(split_t *split) {
    slice_t sub, slice;

    sub = slice_slice(split->slice, split->sep);
    if (sub.data) {
        slice.data = split->slice.data;
        slice.len = sub.data - split->slice.data;
        split->slice.data = sub.data + split->sep.len;
        split->slice.len = sub.len - split->sep.len;
        return slice;
    }

    slice = split->slice;
    split->slice.data = NULL;
    split->slice.len = 0;
    return slice;
}