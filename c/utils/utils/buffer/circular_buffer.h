#if !defined(__CIRCULAR_BUFFER_H__)
#define __CIRCULAR_BUFFER_H__

typedef struct circular_buffer_s {
    char *data;
    int cap, len;
    int write, read;
} circular_buffer_t;

#endif
