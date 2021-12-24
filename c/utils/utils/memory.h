#ifdef __cplusplus
extern "C" {
#endif

void* HzwMalloc(size_t size, char *file, int line);
void HzwFree(void *ptr);
void MemoryCheck();

#define HZW_MALLOC(size) HzwMalloc(size, __FILE__, __LINE__)

#ifdef __cplusplus
}
#endif
