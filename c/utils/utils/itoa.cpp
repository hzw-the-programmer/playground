#include <assert.h>
#include <string.h>
#include "memory.h"

#define NULL 0

char *itoa(int value, char *result, int base) {
	char *chars = "zyxwvutsrqponmlkjihgfedcba9876543210123456789abcdefghijklmnopqrstuvwxyz";
	int temp_value;
	char *start = result, *end = result, temp_char;

	if (base < 2 || base > 36) {
		*result = 0;
		return result;
	}

	do {
		temp_value = value;
		value /= base;
		*end++ = chars[35 + temp_value - value * base];
	} while (value);

	if (temp_value < 0) {
		*end++ = '-';
	}

	*end-- = 0;

	while (start < end) {
		temp_char = *start;
		*start++ = *end;
		*end-- = temp_char;
	}

	return result;
}

int itoa1(int num, int base, char *out, int outLen) {
	char *cs = "zyxwvutsrqponmlkjihgfedcba9876543210123456789abcdefghijklmnopqrstuvwxyz";
	int outIndex = 0, tnum = 0, i = 0, j = 0;
	char tc = 0;

	if (base < 2 || base > 36) {
		goto end;
	}

	do {
		tnum = num;
		num /= base;
		if (outIndex < outLen) {
			out[outIndex] = cs[35 + tnum - num * base];
		}
		outIndex++;
	} while(num);

	if (tnum < 0) {
		if (outIndex < outLen) {
			out[outIndex] = '-';
		}
		outIndex++;
	}

	if (outIndex < outLen) {
		j = outIndex - 1;
	} else {
		j = outLen - 1;
	}

	for (i = 0; i < j; i++, j--) {
		tc = out[i];
		out[i] = out[j];
		out[j] = tc;
	}

end:
	if (outIndex < outLen) {
		out[outIndex] = 0;
	} else if (outLen > 0) {
		out[outLen - 1] = 0;
	}

	return outIndex;
}

char* itoa1Ext(int num, int base) {
	int outLen = itoa1(num, base, NULL, 0) + 1;
	char *out = (char*)HZW_MALLOC(outLen);
	if (out) {
		itoa1(num, base, out, outLen);
	}
	return out;
}

void test_itoa() {
	char buf[20] = {0};
	char *expected = NULL;
	char *out = NULL;
	
	itoa(0, buf, 10);
	itoa(1, buf, 10);
	itoa(12, buf, 10);
	itoa(123, buf, 10);
	itoa(1234, buf, 10);
	itoa(12345, buf, 10);
	itoa(-1, buf, 10);
	itoa(-12, buf, 10);
	itoa(-123, buf, 10);
	itoa(-1234, buf, 10);
	itoa(-12345, buf, 10);

	itoa(0, buf, 16);
	itoa(1, buf, 16);
	itoa(12, buf, 16);
	itoa(123, buf, 16);
	itoa(1234, buf, 16);
	itoa(12345, buf, 16);
	itoa(-1, buf, 16);
	itoa(-12, buf, 16);
	itoa(-123, buf, 16);
	itoa(-1234, buf, 16);
	itoa(-12345, buf, 16);

	expected = "-12345";
	assert(6 == itoa1(-12345, 10, NULL, 0));
	assert(6 == itoa1(-12345, 10, buf, 20));
	assert(!strcmp(expected, buf));

	expected = "12345";
	assert(5 == itoa1(12345, 10, NULL, 0));
	assert(5 == itoa1(12345, 10, buf, 20));
	assert(!strcmp(expected, buf));

	expected = "0";
	assert(1 == itoa1(0, 10, NULL, 0));
	assert(1 == itoa1(0, 10, buf, 20));
	assert(!strcmp(expected, buf));

	expected = "-12345";
	out = itoa1Ext(-12345, 10);
	assert(!strcmp(expected, out));
	HzwFree(out);

	expected = "12345";
	out = itoa1Ext(12345, 10);
	assert(!strcmp(expected, out));
	HzwFree(out);

	expected = "0";
	out = itoa1Ext(0, 10);
	assert(!strcmp(expected, out));
	HzwFree(out);
}
