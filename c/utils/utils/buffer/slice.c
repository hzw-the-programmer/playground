#include "slice.h"
#include "../utils.h"

Slice slice_new(char *buf, int len) {
    Slice s;

    s.buf = buf;
    s.len = len;
    
    return s;
}

Slice slice_sub(Slice in, int begin, int end) {
    Slice out;

    out.buf = in.buf + begin;
    out.len = end - begin;

    return out;
}

SliceSplit slice_split(Slice s, char c) {
    SliceSplit split;

    split.s = s;
    split.c = c;

    return split;
}

Slice slice_split_next(SliceSplit *split) {
    int i;
    Slice s;

    s = split->s;
    
    if (s.buf == 0) return s;

    for (i = 0; i < s.len; i++) {
        if (s.buf[i] == split->c) {
            int b, e;
            
            s.len = i;
            b = i + 1;
            e = split->s.len;
            if (b == e) {
                b = e = i;
            }
            split->s = slice_sub(split->s, b, e);
            return s;
        }
    }

    split->s.buf = 0;
    split->s.len = 0;

    return s;
}

Slice slice_ltrim(Slice in) {
    int i;
    Slice out = {0};

    for (i = 0; i < in.len; i++) {
        if (!is_space(in.buf[i])) {
            out.buf = in.buf + i;
            out.len = in.len - i;
            break;
        }
    }

    return out;
}

Slice slice_rtrim(Slice in) {
    int i;
    Slice out = {0};

    for (i = 0; i < in.len; i++) {
        int j = in.len - 1 - i;
        if (!is_space(in.buf[j])) {
            out.buf = in.buf;
            out.len = j + 1;
            break;
        }
    }

    return out;
}

Slice slice_trim(Slice s) {
    s = slice_ltrim(s);
    s = slice_rtrim(s);
    return s;
}
