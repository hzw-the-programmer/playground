typedef struct {
    char *buf;
    int len;
} Slice;

typedef struct {
    Slice s;
    char c;
} SliceSplit;

Slice slice_new(char *buf, int len);
Slice slice_sub(Slice in, int begin, int end);
SliceSplit slice_split(Slice in, char c);
Slice slice_split_next(SliceSplit *split);
Slice slice_ltrim(Slice in);
Slice slice_rtrim(Slice in);
Slice slice_trim(Slice s);
