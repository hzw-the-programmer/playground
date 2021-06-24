#include <assert.h>
#include <stdio.h>
#include <string.h>

unsigned int pick_one_utf8_char(const char **start, size_t *remainder_p) {
	unsigned char *s = (unsigned char *)*start;
	unsigned int ch;
	size_t remainder, incr;

	remainder = remainder_p ? *remainder_p : 0x7fffffff;

	if (remainder < 1) {
		goto invalid;
	} if (s[0] < 0x80) { // 0xxx xxxx
		ch = s[0];
		incr = 1;
	} else if ((s[0] & 0xe0) == 0xc0) { // 110x xxxx 10xx xxxx
		if (remainder < 2) {
			goto invalid;
		}
		ch = ((s[0] & 0x1f) << 6) | (s[1] & 0x3f);
		incr = 2;
	} else if ((s[0] & 0xf0) == 0xe0) { // 1110 xxxx 10xx xxxx 10xx xxxx
		if (remainder < 3) {
			goto invalid;
		}
		ch = ((s[0] & 0x0f) << 12) | ((s[1] & 0x3f) << 6) | (s[2] & 0x3f);
		incr = 3;
	} else if ((s[0] & 0xf8) == 0xf0) { // 1111 0xxx 10xx xxxx 10xx xxxx 10xx xxxx
		if (remainder < 4) {
			goto invalid;
		}
		ch = ((s[0] & 0x07) << 18) | ((s[1] & 0x3f) << 12) | ((s[2] & 0x3f) << 6) | (s[3] & 0x3f);
		incr = 4;
	} else {
invalid:
		*start = NULL;
		return 0;
	}

	*start += incr;
	if (remainder_p) {
		*remainder_p = remainder - incr;
	}
	
	return ch;
}

int utf8len(const char *utf8) {
	int len = 0;
	size_t remainder = 0x7fffffff;
	while (*utf8) {
		pick_one_utf8_char(&utf8, &remainder);
		len++;
	}
	return len;
}

int put_one_utf8_char(unsigned int ch, char **start) {
	int size = 0;
	
	if (ch < 0x80) { // 0xxx xxxx = 0x7f
		*(*start)++ = ch;
		size = 1;
	} else if (ch < 0x0800) { // 110x xxxx 10xx xxxx = 0x07ff
		*(*start)++ = 0xc0 | (ch >> 6);
		*(*start)++ = 0x80 | (ch & 0x3f);
		size = 2;
	} else if (ch < 0x010000) { // 1110 xxxx 10xx xxxx 10xx xxxx =  0xffff
		*(*start)++ = 0xe0 | (ch >> 12);
		*(*start)++ = 0x80 | ((ch >> 6) & 0x3f);
		*(*start)++ = 0x80 | (ch & 0x3f);
		size = 3;
	} else if (ch < 0x200000) { // 1111 0xxx 10xx xxxx 10xx xxxx 10xx xxxx = 0x1fffff
		*(*start)++ = 0xf0 | (ch >> 18);
		*(*start)++ = 0x80 | ((ch >> 12) & 0x3f);
		*(*start)++ = 0x80 | ((ch >> 6) & 0x3f);
		*(*start)++ = 0x80 | (ch & 0x3f);
		size = 4;
	}

	return size;
}

void test_utf8len() {
	// Œ“∞Æ∫Á«≈£°I love HQ!
	assert(15 == utf8len("\xE6\x88\x91\xE7\x88\xB1\xE8\x99\xB9\xE6\xA1\xA5\xEF\xBC\x81\x49\x20\x6C\x6F\x76\x65\x20\x48\x51\x21"));
}

void test_put_one_utf8_char() {
	char out[] = {0xFF, 0xFF, 0xFF, 0xFF, 0xFF};
	char *p;
	char *expected;
	int size;

	// Œ“
	expected = "\xE6\x88\x91";
	p = out;
	size = put_one_utf8_char(0x6211, &p);
	assert(size == 3);
	*p = 0;
	assert(strcmp(expected, out) == 0);

	// ∞Æ
	expected = "\xE7\x88\xB1";
	p = out;
	size = put_one_utf8_char(0x7231, &p);
	assert(size == 3);
	*p = 0;
	assert(strcmp(expected, out) == 0);

	// ∫Á
	expected = "\xE8\x99\xB9";
	p = out;
	size = put_one_utf8_char(0x8679, &p);
	assert(size == 3);
	*p = 0;
	assert(strcmp(expected, out) == 0);

	// «≈
	expected = "\xE6\xA1\xA5";
	p = out;
	size = put_one_utf8_char(0x6865, &p);
	assert(size == 3);
	*p = 0;
	assert(strcmp(expected, out) == 0);

	// £°
	expected = "\xEF\xBC\x81";
	p = out;
	size = put_one_utf8_char(0xFF01, &p);
	assert(size == 3);
	*p = 0;
	assert(strcmp(expected, out) == 0);

	// I
	expected = "\x49";
	p = out;
	size = put_one_utf8_char(0x49, &p);
	assert(size == 1);
	*p = 0;
	assert(strcmp(expected, out) == 0);

	//  
	expected = "\x20";
	p = out;
	size = put_one_utf8_char(0x20, &p);
	assert(size == 1);
	*p = 0;
	assert(strcmp(expected, out) == 0);

	// l
	expected = "\x6C";
	p = out;
	size = put_one_utf8_char(0x6C, &p);
	assert(size == 1);
	*p = 0;
	assert(strcmp(expected, out) == 0);

	// o
	expected = "\x6F";
	p = out;
	size = put_one_utf8_char(0x6F, &p);
	assert(size == 1);
	*p = 0;
	assert(strcmp(expected, out) == 0);

	// v
	expected = "\x76";
	p = out;
	size = put_one_utf8_char(0x76, &p);
	assert(size == 1);
	*p = 0;
	assert(strcmp(expected, out) == 0);

	// e
	expected = "\x65";
	p = out;
	size = put_one_utf8_char(0x65, &p);
	assert(size == 1);
	*p = 0;
	assert(strcmp(expected, out) == 0);

	//  
	expected = "\x20";
	p = out;
	size = put_one_utf8_char(0x20, &p);
	assert(size == 1);
	*p = 0;
	assert(strcmp(expected, out) == 0);

	// H
	expected = "\x48";
	p = out;
	size = put_one_utf8_char(0x48, &p);
	assert(size == 1);
	*p = 0;
	assert(strcmp(expected, out) == 0);

	// Q
	expected = "\x51";
	p = out;
	size = put_one_utf8_char(0x51, &p);
	assert(size == 1);
	*p = 0;
	assert(strcmp(expected, out) == 0);

	// !
	expected = "\x21";
	p = out;
	size = put_one_utf8_char(0x21, &p);
	assert(size == 1);
	*p = 0;
	assert(strcmp(expected, out) == 0);
}

void test_emoji() {
	char buf[] = {0xf0, 0x9f, 0x98, 0x80};
	size_t len = 4;
	unsigned int r = 0x01f600;
	int i = 0;

	while (true) {
		unsigned int ch;
		char *start;
		size_t reminder;

		start = buf;
		reminder = len;
		ch = pick_one_utf8_char((const char **)&start, &reminder);
		assert(ch == r);
		assert(start - buf == 4);
		assert(reminder == 0);

		buf[len-1]++;
		r++;
		i++;
		if (i == 64) {
			buf[len-2] = 0x99;
			buf[len-1] = 0x80;
		} else if (i == 128) {
			buf[len-2] = 0x9a;
			buf[len-1] = 0x80;
		} else if (i == 192) {
			buf[len-2] = 0x9b;
			buf[len-1] = 0x80;
		} else if (i == 256) {
			break;
		}
	}
}

void test_str() {
	char *str = "\x68\x65\x6C\x6C\x6F\x20\x77\x6F\x72\x6C\x64\x21\xE4\xBD\xA0\xE5\xA5\xBD\xEF\xBC\x8C\xE4\xB8\x96\xE7\x95\x8C\xEF\xBC\x81";
	unsigned int cps[] = {
		0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21,
		0x4f60, 0x597d, 0xff0c, 0x4e16, 0x754c, 0xff01,
	};
	size_t cpsl = sizeof(cps) / sizeof(*cps);

	for (int i = 0; i < cpsl; i++) {
		assert(cps[i] == pick_one_utf8_char((const char **)&str, NULL));
	}
	assert(*str == 0);
}

int is_emoji(unsigned int ch) {
	return ch >= 0x01f600 && ch <= 0x01f6ff;
}

void replace_emoji(char *out, const char *in, const char *r, int max) {
	unsigned int ch;
	int count = 0, founded = 0, rl = strlen(r);

	while (*in) {
		ch = pick_one_utf8_char(&in, NULL);

		if (is_emoji(ch)) {
			if (!founded && count < max) {
				founded = true;
				count++;
				strcpy(out, r);
				out += rl;
			}
		} else {
			founded = false;
			*out++ = ch;
		}
	}
	*out++ = 0;
}

int replace_emoji_len(const char *in, int rl, int max) {
	unsigned int ch;
	int count = 0, founded = 0, len = 0;

	while (*in) {
		ch = pick_one_utf8_char(&in, NULL);

		if (is_emoji(ch)) {
			if (!founded && count < max) {
				founded = true;
				count++;
				len += rl;
			}
		} else {
			founded = false;
			len++;
		}
	}

	return len;
}

void test_replace_emoji() {
	char out[512];
	for (int i = 0; i < 512; i++) {
		out[i] = 0xff;
	}

	char in[] = "h"
		"\xf0\x9f\x98\x80"
		"el"
		"\xf0\x9f\x98\x81" "\xf0\x9f\x98\x82"
		"lo "
		"\xf0\x9f\x98\x83" "\xf0\x9f\x98\x84" "\xf0\x9f\x98\x85"
		"world!";
	char r[] = "[emoji]";
	int rl = strlen(r);
	char expected[] = "h[emoji]el[emoji]lo world!";

	assert(replace_emoji_len(in, rl, 2) == 26);

	replace_emoji(out, in, "[emoji]", 2);

	assert(strcmp(expected, out) == 0);
}

void test_utf8() {
	test_utf8len();
	test_put_one_utf8_char();
	test_emoji();
	test_str();
	test_replace_emoji();
}
