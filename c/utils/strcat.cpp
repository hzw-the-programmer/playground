#include <assert.h> // for assert
#include <string.h> // for strcmp

char* hStrcat(char *dest, const char *src) {
	char *d = dest;
	char c;

	do {
		c = *d++;
	} while (c != 0);

	d -= 2;

	do {
		c = *src++;
		*++d = c;
	} while (c != 0);

	return dest;
}

void test_hStrcat() {
	char dest[100] = "hello";
	char src[] = " world";
	hStrcat(dest, src);
	assert(strcmp(dest, "hello world") == 0);
}
