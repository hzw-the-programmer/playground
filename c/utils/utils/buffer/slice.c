#include "slice.h"
#include "../utils.h"

h_slice h_slice_new(char *data, int len) {
    h_slice s;

    s.data = data;
    s.len = len;
    
    return s;
}

h_slice h_slice_sub(h_slice in, int begin, int end) {
    h_slice out;

    out.data = in.data + begin;
    out.len = end - begin;

    return out;
}

h_slice_split h_slice_split_new(h_slice s, char c) {
    h_slice_split split;

    split.s = s;
    split.c = c;

    return split;
}

h_slice h_slice_split_next(h_slice_split *split) {
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
}

h_slice h_slice_ltrim(h_slice in) {
    int i;
    h_slice out = {0};

    for (i = 0; i < in.len; i++) {
        if (!is_space(in.data[i])) {
            out.data = in.data + i;
            out.len = in.len - i;
            break;
        }
    }

    return out;
}

h_slice h_slice_rtrim(h_slice in) {
    int i;
    h_slice out = {0};

    for (i = 0; i < in.len; i++) {
        int j = in.len - 1 - i;
        if (!is_space(in.data[j])) {
            out.data = in.data;
            out.len = j + 1;
            break;
        }
    }

    return out;
}

h_slice h_slice_trim(h_slice s) {
    s = h_slice_ltrim(s);
    s = h_slice_rtrim(s);
    return s;
}
