extern void* HzwMalloc(size_t size, char *file, int line);
#define HZW_MALLOC(size) HzwMalloc(size, __FILE__, __LINE__)

extern void HzwFree(void *ptr);

extern void MemoryCheck();
