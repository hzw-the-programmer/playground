typedef struct {
    char *data;
    int len;
} h_slice;

typedef struct {
    h_slice s;
    char c;
} h_slice_split;

h_slice h_slice_new(char *data, int len);
h_slice h_slice_sub(h_slice in, int begin, int end);
h_slice_split h_slice_split_new(h_slice in, char c);
h_slice h_slice_split_next(h_slice_split *split);
h_slice h_slice_ltrim(h_slice s, h_slice cutset);
h_slice h_slice_rtrim(h_slice s, h_slice cutset);
h_slice h_slice_trim(h_slice s, h_slice cutset);
h_slice h_slice_ltrim_space(h_slice s);
h_slice h_slice_rtrim_space(h_slice s);
h_slice h_slice_trim_space(h_slice s);
