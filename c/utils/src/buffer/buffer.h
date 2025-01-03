#if !defined(__BUFFER_H__)
#define __BUFFER_H__

#include "types.h"
#include "slice/slice.h"

typedef struct {
    int cap;
    int w, r;
    uint8_t *ptr;
} buf_t;

int buf_available(const buf_t *buf);
uint8_t* buf_write_ptr(const buf_t *buf);
void buf_write_inc(buf_t *buf, int len);
int buf_buffered(const buf_t *buf);
uint8_t* buf_read_ptr(const buf_t *buf);
void buf_read_inc(buf_t *buf, int len);
void buf_tidy(buf_t *buf);

buf_t* buf_new(int cap);
int buf_write(buf_t *buf, slice_t slice);
int buf_read(buf_t *buf, slice_t *slice);
int buf_read_out(buf_t *buf, slice_t *out);

void buf_split(buf_t *buf, slice_t sep, int (*cb)(void*, slice_t*), void *arg);
buf_t* buf_static(uint8_t *ptr, int len);

slice_t buf_buffered_slice(const buf_t *buf);

#endif
