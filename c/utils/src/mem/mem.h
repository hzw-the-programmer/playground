#if !defined(__MEM_H__)
#define __MEM_H__

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

void* hmalloc(size_t size, char *file, int line);
void hfree(void *ptr);
void hmcheck();

#define malloc(size) hmalloc(size, __FILE__, __LINE__)
#define free(ptr) hfree(ptr);

#ifdef __cplusplus
}
#endif
#endif
