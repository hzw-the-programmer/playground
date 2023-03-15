#include <string.h>

#include "slice.h"
#include "utils.h"

slice_t slice_new(char *data, int len) {
    slice_t s;

    s.data = data;
    s.len = len;
    
    return s;
}

slice_t slice_sub(slice_t in, int begin, int end) {
    slice_t out;

    out.data = in.data + begin;
    out.len = end - begin;

    return out;
}

int slice_search(slice_t s, char b) {
    int i;

    for (i = 0; i < s.len; i++) {
        if (s.data[i] == b) {
            return i;
        }
    }

    return -1;
}

slice_t slice_slice(slice_t ss, slice_t s) {
    slice_t r = {0};
    unsigned char *p;

    if (s.len <= 0 || s.len > ss.len) {
        return r;
    }
    
    while (ss.len >= s.len) {
        p = memchr(ss.data, s.data[0], ss.len);
        if (!p) {
            return r;
        }
        
        ss.len -= p - ss.data;
        if (ss.len < s.len) {
            return r;
        }
        ss.data = p;
        
        if (!memcmp(p, s.data, s.len)) {
            r.data = p;
            r.len = ss.len;
            return r;
        }

        ss.data++;
        ss.len--;
    }

    return r;
}

slice_t slice_ltrim(slice_t s, slice_t cutset) {
    while (s.len > 0) {
        if (slice_search(cutset, *s.data) == -1) {
            return s;
        }
        s.len--;
        s.data += s.len > 0;
    }

    return s;
}

slice_t slice_rtrim(slice_t s, slice_t cutset) {
    while (s.len > 0) {
        if (slice_search(cutset, s.data[s.len - 1]) == -1) {
            return s;
        }
        s.len--;
    }

    return s;
}

slice_t slice_trim(slice_t s, slice_t cutset) {
    return slice_rtrim(slice_ltrim(s, cutset), cutset);
}

slice_t slice_ltrim_space(slice_t s) {
    return slice_ltrim(s, slice_new(SPACES, SPACES_LEN));
}

slice_t slice_rtrim_space(slice_t s) {
    return slice_rtrim(s, slice_new(SPACES, SPACES_LEN));
}

slice_t slice_trim_space(slice_t s) {
    return slice_trim(s, slice_new(SPACES, SPACES_LEN));
}

uint64_t slice_to_uint64(slice_t s) {
    uint64_t n = 0;

    while (s.len > 0) {
        if (*s.data < '0' || *s.data > '9') {
            break;
        }
        n *= 10;
        n += *s.data - '0';
        s.len--;
        s.data++;
    }

    return n;
}
