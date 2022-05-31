#if !defined(__H_SLICE__)
#define __H_SLICE__

typedef struct {
    char *data;
    int len;
} h_slice;

h_slice h_slice_new(char *data, int len);

h_slice h_slice_sub(h_slice in, int begin, int end);
int h_slice_search(h_slice s, char b);

h_slice h_slice_ltrim(h_slice s, h_slice cutset);
h_slice h_slice_rtrim(h_slice s, h_slice cutset);
h_slice h_slice_trim(h_slice s, h_slice cutset);
h_slice h_slice_ltrim_space(h_slice s);
h_slice h_slice_rtrim_space(h_slice s);
h_slice h_slice_trim_space(h_slice s);

#endif // __H_SLICE__
