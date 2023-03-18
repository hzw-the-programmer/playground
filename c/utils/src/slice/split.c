#include "split.h"

split_t split_new(slice_t s, char c) {
    split_t split;

    split.s = s;
    split.c = c;

    return split;
}

slice_t split_next(split_t *split) {
#if 1
    slice_t s;
    int i;

    s = split->s;

    if (s.data == 0) return s;

    i = slice_search(split->s, split->c);
    if (i != -1) {
        s.len = i;
        split->s.len -= i + 1;
        split->s.data += i + (split->s.len > 0);
        return s;
    }

    split->s.data = 0;
    split->s.len = 0;

    return s;
#else
#if 1
    h_slice s;

    s = split->s;

    if (s.data == 0) return s;

    while (split->s.len > 0) {
        if (*split->s.data == split->c) {
            s.len = split->s.data - s.data;
            split->s.len--;
            split->s.data += split->s.len > 0;
            return s;
        }
        split->s.len--;
        split->s.data++;
    }

    split->s.data = 0;
    split->s.len = 0;

    return s;
#else
    int i;
    h_slice s;

    s = split->s;
    
    if (s.data == 0) return s;

    for (i = 0; i < s.len; i++) {
        if (s.data[i] == split->c) {
            int b, e;
            
            s.len = i;
            b = i + 1;
            e = split->s.len;
            if (b == e) {
                b = e = i;
            }
            split->s = h_slice_sub(split->s, b, e);
            return s;
        }
    }

    split->s.data = 0;
    split->s.len = 0;

    return s;
#endif
#endif
}

split_t split_new_ext(
    uint8_t *data, int data_len,
    const uint8_t *sep, int sep_len) {
    split_t split;

    split.s = slice_new(data, data_len);
    split.sep = slice_new((uint8_t*)sep, sep_len);

    return split;
}

slice_t split_next_ext(split_t *split) {
    slice_t r = {0}, sub;

    r = split->s;
    sub = slice_slice(split->s, split->sep);
    if (sub.data) {
        r.data = split->s.data;
        r.len = sub.data - split->s.data;
        split->s.data = sub.data + split->sep.len;
        split->s.len = sub.len - split->sep.len;
        return r;
    }

    split->s.data = NULL;
    split->s.len = 0;
    
    return r;
}