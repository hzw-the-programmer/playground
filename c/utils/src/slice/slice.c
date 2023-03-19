#include <string.h>

#include "slice.h"
#include "utils.h"

slice_t slice_new(char *ptr, int len) {
    slice_t s;

    s.ptr = ptr;
    s.len = len;
    
    return s;
}

slice_t slice_sub(slice_t in, int begin, int end) {
    slice_t out;

    out.ptr = in.ptr + begin;
    out.len = end - begin;

    return out;
}

int slice_search(slice_t s, char b) {
    int i;

    for (i = 0; i < s.len; i++) {
        if (s.ptr[i] == b) {
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
        p = memchr(ss.ptr, s.ptr[0], ss.len);
        if (!p) {
            return r;
        }
        
        ss.len -= p - ss.ptr;
        if (ss.len < s.len) {
            return r;
        }
        ss.ptr = p;
        
        if (!memcmp(p, s.ptr, s.len)) {
            r.ptr = p;
            r.len = ss.len;
            return r;
        }

        ss.ptr++;
        ss.len--;
    }

    return r;
}

slice_t slice_ltrim(slice_t s, slice_t cutset) {
    while (s.len > 0) {
        if (slice_search(cutset, *s.ptr) == -1) {
            return s;
        }
        s.len--;
        s.ptr += s.len > 0;
    }

    return s;
}

slice_t slice_rtrim(slice_t s, slice_t cutset) {
    while (s.len > 0) {
        if (slice_search(cutset, s.ptr[s.len - 1]) == -1) {
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
        if (*s.ptr < '0' || *s.ptr > '9') {
            break;
        }
        n *= 10;
        n += *s.ptr - '0';
        s.len--;
        s.ptr++;
    }

    return n;
}
