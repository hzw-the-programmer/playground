#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <stddef.h> // offsetof

struct S1 {
	char *p;
	char c;
	int i;
};

struct S2 {
	char *p;
	char c[5];
	int i;
};

struct S3 {
	char *p;
	char c;
};

static void printhex(unsigned char *buf, int len) {
	char *fmt = "%02x ";
	for (int i = 0; i < len; i++) {
		if (i == len - 1) {
			fmt = "%02x\n";
		}
		printf(fmt, buf[i]);
	}
}

void test_struct2() {
	assert(4 == sizeof(char*));
	assert(4 == sizeof(int));
	assert(12 == sizeof(struct S1));

	struct S1 s1;
	memset(&s1, 0xff, sizeof(s1));
	printhex((unsigned char*)&s1, sizeof(s1));
	s1.p = (char*)0x01020304;
	s1.c = 0x05;
	s1.i = 0x06070809;
	printhex((unsigned char*)&s1, sizeof(s1));

	struct S2 s2;
	memset(&s2, 0xff, sizeof(s2));
	printhex((unsigned char*)&s2, sizeof(s2));
	s2.p = (char*)0x01020304;
	s2.c[0] = 0x05;
	s2.c[1] = 0x05;
	s2.c[2] = 0x05;
	s2.c[3] = 0x05;
	s2.c[4] = 0x05;
	s2.i = 0x06070809;
	printhex((unsigned char*)&s2, sizeof(s2));

	assert(8 == sizeof(struct S3));

	struct S3 s3;
	memset(&s3, 0xff, sizeof(s3));
	printhex((unsigned char*)&s3, sizeof(s3));
	s3.p = (char*)0x01020304;
	s3.c = 0x05;
	printhex((unsigned char*)&s3, sizeof(s3));
	printhex((unsigned char*)&s3.c, sizeof(s3.c));

	assert(sizeof(struct S3) - sizeof(char) == 7);
	assert(&s3.c - (char*)&s3 == 4);

	printhex((unsigned char*)&s3, sizeof(s3) - sizeof(s3.c));
	printhex((unsigned char*)&s3, &s3.c - (char*)&s3);

	assert(offsetof(struct S3, c) == 4);
	assert(sizeof(struct S3) == 8);
	assert(offsetof(struct S2, i) == 12);
}
