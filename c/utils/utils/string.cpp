#include <assert.h> // for assert

int strcmp(const char *p1, const char *p2) {
	unsigned char c1, c2;
	
	do {
		c1 = (unsigned char)*p1++;
		c2 = (unsigned char)*p2++;
		if (c1 == 0) {
			break;
		}
	} while (c1 == c2);

	return c1 - c2;
}

void test_strcmp() {
	assert(strcmp("hzw", "hzw") == 0);
	assert(strcmp("hzw", "hzwe") < 0);
	assert(strcmp("hzwa", "hzw") > 0);
	assert(strcmp("\xf0", "\xf0") == 0);
    assert(strcmp("\xff", "\x01") > 0);
}

int strncmp(const char *p1, const char *p2, int n) {
    unsigned char c1, c2;

    while (n-- > 0) {
        c1 = (unsigned char)*p1++;
        c2 = (unsigned char)*p2++;
        if (c1 != c2) {
            return c1 - c2;
        }
        if (c1 == '\0') {
            return 0;
        }
    }

    return 0;
}

void test_strncmp() {
    assert(strncmp("123", "123", 3) == 0);
    assert(strncmp("123", "123", 10) == 0); 
    assert(strncmp("123", "1234", 3) == 0);
}

void test_string() {
    test_strcmp();
    test_strncmp();
}
