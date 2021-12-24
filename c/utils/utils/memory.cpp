#include <stdlib.h> // malloc
#include <stdio.h> // printf
//#include <string.h> // strdup

#include "utlist.h"

#include "memory.h"

typedef struct hzwMemItem {
	void *ptr;
	size_t size;
	char *file;
	int line;
	hzwMemItem *next;
} hzwMemItem;

static hzwMemItem *memItemHead = NULL;

void* HzwMalloc(size_t size, char *file, int line) {
	hzwMemItem *memItem = NULL;
	
	void *ptr = malloc(size);
	if (!ptr) {
		return ptr;
	}

	memItem = (hzwMemItem*)malloc(sizeof(hzwMemItem));
	assert(memItem);
	memItem->ptr = ptr;
	memItem->size = size;
	//memItem->file = strdup(file);
	memItem->file = file;
	memItem->line = line;

	LL_APPEND(memItemHead, memItem);

	return ptr;
}

int hzwCmp(hzwMemItem *memItem, void *ptr) {
	return memItem->ptr == ptr ? 0 : 1;
}

void HzwFree(void *ptr) {
	hzwMemItem *memItem = NULL;
	LL_SEARCH(memItemHead, memItem, ptr, hzwCmp);
	LL_DELETE(memItemHead, memItem);
}

void MemoryCheck() {
	hzwMemItem *memItem = NULL;

	LL_FOREACH(memItemHead, memItem) {
		printf("%s: %d\n", memItem->file, memItem->line);
	}
}
