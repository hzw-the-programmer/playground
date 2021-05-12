```
#include "simple_memory_manager.h"
#include "app_zlib.h"

#define HZW_BUF_POOL_SIZE (100 * 1024)

STSMM smm = {0};

void* hzwMalloc(void *opaque, unsigned int items, unsigned int size) {
	return smmMallocAligned(&smm, items * size, 4);
}

void hzwFree(void *opaque, void *address) {
}

void test() {
    char source[] = {
        0x78, 0x9c, 0xca, 0x48, 0xcd, 0xc9, 0xc9, 0x57, 0x28, 0xcf,
        0x2f, 0xca, 0x49, 0xe1, 0x02, 0x04, 0x00, 0x00, 0xff, 0xff,
        0x1e, 0x72, 0x04, 0x67,
    };
    char dest[100] = {0};
    int destLen = 100;

    void *buf_pool = 0;
    int result = 0;

    buf_pool = applib_asm_alloc_anonymous(HZW_BUF_POOL_SIZE);
    smmInit(&smm, buf_pool, HZW_BUF_POOL_SIZE);

    result = uncompress_mtk(dest, &destLen, source, sizeof(source), hzwMalloc, hzwFree);

    applib_asm_free_anonymous(buf_pool);
}
```
