#include "slice.h"

typedef struct {
    h_slice s;
    char c;
} h_split;

h_split h_split_new(h_slice in, char c);
h_slice h_split_next(h_split *split);
