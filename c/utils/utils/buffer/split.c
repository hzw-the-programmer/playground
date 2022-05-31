#include "split.h"

h_split h_split_new(h_slice s, char c) {
    h_split split;

    split.s = s;
    split.c = c;

    return split;
}

h_slice h_split_next(h_split *split) {
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
}
