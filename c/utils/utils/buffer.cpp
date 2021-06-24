#include <string.h>
#include <assert.h>

unsigned int pick_one_utf8_char(const char **start, size_t *reminder_p);

typedef struct {
	size_t len;
	size_t cap;
	size_t index;
	char *buf;
} Buffer;

size_t bufferAvailable(Buffer *buffer) {
	return buffer->len - buffer->index;
}

size_t bufferReadOneUtf8Char(Buffer *buffer, unsigned int *codepoint) {
	char *s = buffer->buf + buffer->index;
	size_t len = 0;

	if (bufferAvailable(buffer) < 1) {
		goto invalid;
	} else if ((s[0] & 0x80) == 0) {
		// 0xxx xxxx
		*codepoint = s[0];
		len = 1;
	} else if ((s[0] & 0xe0) == 0xc0) {
		// 110x xxxx 10xx xxxx
		if (bufferAvailable(buffer) < 2) {
			goto invalid;
		}
		*codepoint = ((s[0] & 0x1f) << 6) | (s[1] & 0x3f);
		len = 2;
	} else if ((s[0] & 0xf0) == 0xe0) {
		// 1110 xxxx 10xx xxxx 10xx xxxx
		if (bufferAvailable(buffer) < 3) {
			goto invalid;
		}
		*codepoint = ((s[0] & 0x0f) << 12) | ((s[1] & 0x3f) << 6) | (s[2] & 0x3f);
		len = 3;
	} else if ((s[0] & 0xf8) == 0xf0) {
		// 1111 0xxx 10xx xxxx 10xx xxxx 10xx xxxx
		if (bufferAvailable(buffer) < 4) {
			goto invalid;
		}
		*codepoint = ((s[0] & 0x07) << 18) | ((s[1] & 0x3f) << 12) | ((s[2] & 0x3f) << 6) | (s[3] & 0x3f);
		len = 4;
	} else {
invalid:
		*codepoint = 0;
		return 0;
	}

	buffer->index += len;

	return len;
}

void test_bufferReadOneUtf8Char() {
	char *buf = "\x68"
		"\xe2\x98\xba" "\xef\xb8\x8f"
		"\xf0\x9f\x98\x80"
		"\xe6\x88\x91";
	size_t l = strlen(buf) + 1;
	
	unsigned int codepoint;
	size_t len;

	Buffer buffer = {l, l, 0, buf};

	len = bufferReadOneUtf8Char(&buffer, &codepoint);
	assert(len == 1);
	assert(codepoint == 0x68);

	len = bufferReadOneUtf8Char(&buffer, &codepoint);
	assert(len == 3);
	assert(codepoint == 0x263a);

	len = bufferReadOneUtf8Char(&buffer, &codepoint);
	assert(len == 3);
	assert(codepoint == 0xfe0f);

	len = bufferReadOneUtf8Char(&buffer, &codepoint);
	assert(len == 4);
	assert(codepoint == 0x1f600);

	len = bufferReadOneUtf8Char(&buffer, &codepoint);
	assert(len == 3);
	assert(codepoint == 0x6211);

	len = bufferReadOneUtf8Char(&buffer, &codepoint);
	assert(len == 1);
	assert(codepoint == 0);

	len = bufferReadOneUtf8Char(&buffer, &codepoint);
	assert(len == 0);
	assert(codepoint == 0);
}

void isEmoji(unsigned int *codepoints, size_t len) {

}

void replace(char *out,
					  char *in,
				      int (*needReplace)(unsigned int *codepoints, size_t len),
					  char *replacement) {
#if 0
	size_t codepointLen,
	unsigned int codepoint;
	
	size_t inLen = strlen(in);
	Buffer buffer = {l, l, 0, in};
	while (true) {
		codepointLen = bufferReadOneUtf8Char(&buffer, &codepoint);
		if (!codepointLen) {
			return;
		}

	}
#endif
}

void test_replace() {
	//replace();
}

void test_buffer() {
	test_bufferReadOneUtf8Char();
	test_replace();
}
