#include <assert.h> // for assert
#include <stdio.h> // for printf
#include <string.h> // for memcmp
#include <stdlib.h> // for malloc

typedef struct {
	char a;
	int b;
} TestStruct1;

void printmem(unsigned char *c, int len) {
	for (int i = 0; i < len; i++) {
		printf("%02x", c[i]);
	}
	printf("\n");
}

void test_struct() {
	TestStruct1 a = {1, 2};
	TestStruct1 b = {1, 2};
	unsigned char *pa = (unsigned char*)&a;
	unsigned char *pb = (unsigned char*)&b;
	
	printmem(pa, sizeof(a));
	printmem(pb, sizeof(b));
	*(pb+1) = 0x11;
	printmem(pa, sizeof(a));
	printmem(pb, sizeof(b));
	assert(a.a == b.a && a.b == b.b);

	a = b;
	printmem(pa, sizeof(a));
	printmem(pb, sizeof(b));

	assert(a.a == b.a && a.b == b.b);
	assert(sizeof(a) == 8);
	assert(memcmp(&a, &b, sizeof(a)) == 0);

	TestStruct1 *c = (TestStruct1*)malloc(sizeof(TestStruct1));
	TestStruct1 *d = (TestStruct1*)malloc(sizeof(TestStruct1));
	printmem((unsigned char*)c, sizeof(*c));
	printmem((unsigned char*)d, sizeof(*d));
	c->a = d->a = 1;
	c->b = d->b = 2;
	printmem((unsigned char*)c, sizeof(*c));
	printmem((unsigned char*)d, sizeof(*d));

	assert(a.a == c->a && a.b == c->b);
	//assert(memcmp(&a, c, sizeof(a)) == 0);
}
