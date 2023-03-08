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

void test_strncat() {
    char dest[100] = {'a', 'b', 'c', '\0', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n'};
    int cap = 6;
    int len = strlen(dest);

    //strncat(dest, "123", 6);
    //strncat(dest, "1", 6);
    //strncat(dest, "1234", 6);
    //strncat(dest, "1234567", 6);
    strncat(dest, "1234567", cap-len-1);
}

void test_hStrcat() {
	char dest[100] = "hello";
	char src[] = " world";
	hStrcat(dest, src);
	assert(strcmp(dest, "hello world") == 0);
    test_strncat();
}
