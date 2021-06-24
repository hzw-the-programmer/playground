#include <assert.h>
#include "memory.h"

#define uint8 unsigned char
#define NULL 0

char hexChar(uint8 num) {
	assert(num < 16);
	
	if (num < 10) return '0' + num;

	return 'a' + num - 10;
}

int hex(uint8 *in, int inLen, char *out, int outLen ) {
	int i = 0, j = 0;

	for (i = 0; i < inLen; i++) {
		if (j < outLen) {
			out[j] = hexChar((in[i] & 0xf0) >> 4);
		}
		j++;
		if (j < outLen) {
			out[j] = hexChar(in[i] & 0x0f);
		}
		j++;
	}

	if (j < outLen) {
		out[j] = 0;
	} else if (outLen > 0) {
		out[outLen - 1] = 0;
	}

	return j;
}

char* hexExt(uint8 *in, int inLen) {
	int outLen = hex(in, inLen, NULL, 0);
	char *out = (char*)HZW_MALLOC(outLen + 1);
	hex(in, inLen, out, outLen + 1);
	return out;
}

void testHex() {
	uint8 in[256] = {0};
	int i = 0;
	char *out = NULL;
	for (i = 0; i < 256; i++) {
		in[i] = i;
	}
	out = hexExt(in, 256);
	assert(out[512] == 0);
	assert(out[511] == 'f');
	assert(out[510] == 'f');
	assert(out[509] == 'e');
	assert(out[508] == 'f');
	HzwFree(out);
}
