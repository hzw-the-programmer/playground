#include <stdlib.h> // malloc
#include <stdio.h> // printf
//#include <string.h> // strdup

#include "utlist.h"

typedef struct mitem_s {
	void *ptr;
	size_t size;
	const char *file;
	int line;
	struct mitem_s *next;
} mitem_t;

static mitem_t *mitem_head;

void* hmalloc(size_t size, const char *file, int line) {
	mitem_t *mitem;
	
	void *ptr = malloc(size);
	if (!ptr) {
		return NULL;
	}

	mitem = malloc(sizeof(*mitem));
	assert(mitem);
	mitem->ptr = ptr;
	mitem->size = size;
	//mitem->file = strdup(file);
	mitem->file = file;
	mitem->line = line;

	LL_APPEND(mitem_head, mitem);

	return ptr;
}

int hcmp(mitem_t *mitem, void *ptr) {
	return mitem->ptr == ptr ? 0 : 1;
}

void hfree(void *ptr) {
	mitem_t *mitem = NULL;

    LL_SEARCH(mitem_head, mitem, ptr, hcmp);
	LL_DELETE(mitem_head, mitem);
}

void hmcheck() {
	mitem_t *mitem;

	LL_FOREACH(mitem_head, mitem) {
		printf("%s: %d\n", mitem->file, mitem->line);
	}
}
