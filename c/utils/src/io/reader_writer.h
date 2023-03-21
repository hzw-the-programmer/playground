#if !defined(__READER_WRITER_H__)
#define __READER_WRITER_H__

typedef int (*write_t)(void*, slice_t);
typedef int (*read_t)(void*, slice_t*);

#endif
