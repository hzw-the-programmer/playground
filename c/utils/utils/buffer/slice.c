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

int h_slice_search(h_slice s, char b) {
    int i;

    for (i = 0; i < s.len; i++) {
        if (s.data[i] == b) {
            return i;
        }
    }

    return -1;
}

h_slice h_slice_ltrim(h_slice s, h_slice cutset) {
    while (s.len > 0) {
        if (h_slice_search(cutset, *s.data) == -1) {
            return s;
        }
        s.len--;
        s.data += s.len > 0;
    }

    return s;
}

h_slice h_slice_rtrim(h_slice s, h_slice cutset) {
    while (s.len > 0) {
        if (h_slice_search(cutset, s.data[s.len - 1]) == -1) {
            return s;
        }
        s.len--;
    }

    return s;
}

h_slice h_slice_trim(h_slice s, h_slice cutset) {
    return h_slice_rtrim(h_slice_ltrim(s, cutset), cutset);
}

h_slice h_slice_ltrim_space(h_slice s) {
    return h_slice_ltrim(s, h_slice_new(SPACES, SPACES_LEN));
}

h_slice h_slice_rtrim_space(h_slice s) {
    return h_slice_rtrim(s, h_slice_new(SPACES, SPACES_LEN));
}

h_slice h_slice_trim_space(h_slice s) {
    return h_slice_trim(s, h_slice_new(SPACES, SPACES_LEN));
}