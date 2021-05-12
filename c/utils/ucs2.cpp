#include <assert.h> // for assert
#include <wchar.h> // for wcscmp
#include <string.h> // for strcmp

int asc_to_ucs2(char *out, const char *in) {
	int len = 0;
	while(*in) {
		*out++ = *in++;
		*out++ = 0;
		len++;
	}
	*out++ = 0;
	*out++ = 0;
	return len;
}

int asc_n_to_ucs2(char *out, const char *in, int n) {
	int len = 0;
	while (n--) {
		*out++ = *in++;
		*out++ = 0;
		len++;
	}
	*out++ = 0;
	*out++ = 0;
	return len;
}

int ucs2_to_asc(char *out, const char *in) {
	int len = 0;
	while (*in) {
		*out++ = *in;
		in += 2;
		len++;
	}
	*out++ = 0;
	return len;
}

int ucs2_n_to_asc(char *out, const char *in, int n) {
	int len = 0;
	while (n--) {
		*out++ = *in;
		in += 2;
		len++;
	}
	*out++ = 0;
	return len;
}

void test_ucs2() {
	wchar_t out[512];
	for (int i = 0; i < 512; i++) {
		out[i] = 0xff;
	}
	
	char out2[512];
	for (int i = 0; i < 512; i++) {
		out[i] = 0xff;
	}
	
	int len;

	len = asc_to_ucs2((char*)out, "abc");
	assert(wcscmp(L"abc", out) == 0);
	assert(3 == len);

	len = asc_n_to_ucs2((char*)out, "abc", 2);
	assert(wcscmp(L"ab", out) == 0);
	assert(2 == len);

	len = ucs2_to_asc(out2, (char*)L"abc");
	assert(strcmp("abc", out2) == 0);
	assert(3 == len);

	len = ucs2_n_to_asc(out2, (char*)L"abc", 2);
	assert(strcmp("ab", out2) == 0);
	assert(2 == len);
}
