#include <stddef.h> // NULL
#include <assert.h> // assert
#include <string.h> // memcpy

#include "memory.h"

typedef unsigned char uint8;

#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define MIN(a, b) ((a) < (b) ? (a) : (b))

typedef struct {
	uint8 *buf;
	int cap;
	int len;
} Slice;

void SliceFree(Slice *s) {
	if (!s) return;
	if (s->buf) HzwFree(s->buf);
	HzwFree(s);
}

Slice* SliceNew(int len, int cap) {
	Slice *s = NULL;
	bool suc = false;

	if (!cap) cap = len;
	if (!cap || cap < len) goto error;

	s = (Slice*)HZW_MALLOC(sizeof(Slice));
	if (!s) goto error;

	s->buf = (uint8*)HZW_MALLOC(cap);
	if (!s->buf) goto error;

	s->cap = cap;
	s->len = len;

	suc = true;

error:
	if (!suc) {
		SliceFree(s);
		s = NULL;
	}
	
	return s;
}

int SliceCopyBuf(Slice *s, uint8 *buf, int len) {
	int l = 0;

	l = MIN(s->len, len);
	memcpy(s->buf, buf, l);

	return l;
}

Slice SliceSlice(Slice *sp, int start, int end) {
	Slice s = {0};

#if 0
	s.buf = sp->buf + start;
	s.cap = 
#endif

	return s;
}

void test_slice_new() {
	Slice *s = SliceNew(0, 0);
	assert(!s);

	s = SliceNew(10, 0);
	assert(s);
	assert(s->buf);
	assert(s->cap == 10);
	assert(s->len == 10);
	SliceFree(s);

	s = SliceNew(0, 10);
	assert(s);
	assert(s->buf);
	assert(s->cap == 10);
	assert(s->len == 0);
	SliceFree(s);

	s = SliceNew(10, 10);
	assert(s);
	assert(s->buf);
	assert(s->cap == 10);
	assert(s->len == 10);
	SliceFree(s);

	s = SliceNew(10, 20);
	assert(s);
	assert(s->buf);
	assert(s->cap == 20);
	assert(s->len == 10);
	SliceFree(s);

	s = SliceNew(20, 10);
	assert(!s);
}

void test_slice_copy_buf() {
	char buf[] = "0123456789";
	Slice *s = NULL;
	int len = 0;

	s = SliceNew(5, 10);
	assert(s);

	len = SliceCopyBuf(s, (uint8*)buf, strlen(buf));
	assert(len == 5);
	assert(!strncmp((const char *)s->buf, buf, len));

	SliceFree(s);

	s = SliceNew(20, 0);
	assert(s);

	len = SliceCopyBuf(s, (uint8*)buf, strlen(buf));
	assert(len == 10);
	assert(!strncmp((const char *)s->buf, buf, len));

	SliceFree(s);
}

void test_slice_slice() {
	char buf[] = "0123456789";

	Slice *sp = NULL, s = {0};
	
	sp = SliceNew(5, 10);
	assert(sp);

	SliceCopyBuf(sp, (uint8*)buf, strlen(buf));

#if 0
	s = SliceSlice(sp, 1, 3);
	assert(!strcmp(s.buf, ""));
#endif

	SliceFree(sp);
}

void test_slice() {
	test_slice_new();
	test_slice_copy_buf();
	test_slice_slice();
}
