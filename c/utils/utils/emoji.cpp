#include <assert.h>
#include <string.h>
#include "memory.h"

int pickRune(const char *utf8, int utf8Len, unsigned int *rune) {
	if (utf8Len < 1) return 0;

	if ((utf8[0] & 0x80) == 0x00) { // 0xxx xxxx
		*rune = utf8[0];
		return 1;
	} else if ((utf8[0] & 0xe0) == 0xc0) { // 110x xxxx 10xx xxxx
		if (utf8Len < 2) return 0;
		*rune = ((utf8[0] & 0x1f) << 6) | (utf8[1] & 0x3f);
		return 2;
	} else if ((utf8[0] & 0xf0) == 0xe0) { // 1110 xxxx 10xx xxxx 10xx xxxx
		if (utf8Len < 3) return 0;
		*rune = ((utf8[0] & 0x0f) << 12) | ((utf8[1] & 0x3f) << 6) | (utf8[2] & 0x3f);
		return 3;
	} else if ((utf8[0] & 0xf8) == 0xf0) { // 1111 0xxx 10xx xxxx 10xx xxxx 10xx xxxx
		if (utf8Len < 4) return 0;
		*rune = ((utf8[0] & 0x07) << 18) | ((utf8[1] & 0x3f) << 12) | ((utf8[2] & 0x3f) << 6) | (utf8[3] & 0x3f);
		return 4;
	}

	return 0;
}

int isEmoji(unsigned int rune) {
	return rune == 'a';
}

//int emojiReplace(const char *utf8, int utf8Len, const char *v, int vLen, int max, unsigned int *runes, int runesLen) {
int emojiReplace(const char *utf8, int utf8Len, const char *v, int vLen, int max, char *runes, int runesLen) {
	unsigned int rune = 0;
	int size = 0;
	int runesIndex = 0, vIndex = 0;
	int found = 0, count = 0;

	do {
		size = pickRune(utf8, utf8Len, &rune);
		utf8 += size;
		utf8Len -= size;

		if (!size) {
			break;
		}

		if (isEmoji(rune)) {
			if (!found && count < max) {
				found = 1;
				count++;
				for (vIndex = 0; vIndex < vLen; vIndex++) {
					if (runesIndex < runesLen) {
						runes[runesIndex] = v[vIndex];
					}
					runesIndex++;
				}
			}
		} else {
			found = 0;
			if (runesIndex < runesLen) {
				runes[runesIndex] = rune;
			}
			runesIndex++;
		}
	} while(utf8Len);

	if (runesIndex < runesLen) {
		runes[runesIndex] = 0;
	} else if (runesLen > 0) {
		runes[runesLen - 1] = 0;
	}

	return runesIndex;
}

int emojiReplace1(const char *utf8, int utf8Len, char *runes, int runesLen) {
	const char *v = "hZw";
	int max = 3;

	return emojiReplace(utf8, utf8Len, v, strlen(v), max, runes, runesLen);
}

char* emojiReplace2(const char *utf8, int utf8Len) {
	int runesLen = emojiReplace1(utf8, utf8Len, NULL, 0);
	char *runes = (char*)HZW_MALLOC(runesLen + 1);
	if (runes) {
		emojiReplace1(utf8, utf8Len, runes, runesLen + 1);
	}
	return runes;
}

void testEmoji() {
	unsigned int rune = 0;
	const char *utf8 = NULL;
	const char *v = NULL;
	char runes[100] = {0};
	char *expected = NULL;
	char *out = NULL;

	utf8 = "\x24";
	assert(1 == pickRune(utf8, strlen(utf8), &rune));
	assert(0x24 == rune);

	utf8 = "\xc2\xa2";
	assert(2 == pickRune(utf8, strlen(utf8), &rune));
	assert(0xa2 == rune);

	utf8 = "\xe0\xa4\xb9";
	assert(3 == pickRune(utf8, strlen(utf8), &rune));
	assert(0x0939 == rune);

	utf8 = "\xe2\x82\xac";
	assert(3 == pickRune(utf8, strlen(utf8), &rune));
	assert(0x20ac == rune);

	utf8 = "\xed\x95\x9c";
	assert(3 == pickRune(utf8, strlen(utf8), &rune));
	assert(0xd55c == rune);

	utf8 = "\xf0\x90\x8d\x88";
	assert(4 == pickRune(utf8, strlen(utf8), &rune));
	assert(0x10348 == rune);

	utf8 = "babaabaaab";
	v = "xXx";
	expected = "bxXxbxXxbb";
	assert(strlen(expected) == emojiReplace(utf8, strlen(utf8), v, strlen(v), 2, NULL, 0));
	assert(strlen(expected) == emojiReplace(utf8, strlen(utf8), v, strlen(v), 2, runes, strlen(expected) + 1));
	assert(strcmp(expected, runes) == 0);

	utf8 = "aaaabbbaaabbbbbbaaabbbbbbbbabababababababaaaaaa";
	v = "xXx";
	expected = "xXxbbbxXxbbbbbbxXxbbbbbbbbxXxbxXxbxXxbxXxbxXxbxXxbxXxbxXx";
	assert(strlen(expected) == emojiReplace(utf8, strlen(utf8), v, strlen(v), 50, NULL, 0));
	assert(strlen(expected) == emojiReplace(utf8, strlen(utf8), v, strlen(v), 50, runes, strlen(expected) + 1));
	assert(strcmp(expected, runes) == 0);

	utf8 = "aaaabbbaaabbbbbbaaabbbbbbbbabababababababaaaaaa";
	v = "xXx";
	expected = "xXxbbbxXxbbbbbbxXxbbbbbbbbxXxbxXxbbbbbb";
	assert(strlen(expected) == emojiReplace(utf8, strlen(utf8), v, strlen(v), 5, NULL, 0));
	assert(strlen(expected) == emojiReplace(utf8, strlen(utf8), v, strlen(v), 5, runes, strlen(expected) + 1));
	assert(strcmp(expected, runes) == 0);

	utf8 = "babaabaaab";
	expected = "bhZwbhZwbhZwb";
	assert(strlen(expected) == emojiReplace1(utf8, strlen(utf8), NULL, 0));
	assert(strlen(expected) == emojiReplace1(utf8, strlen(utf8), runes, strlen(expected) + 1));
	assert(strcmp(expected, runes) == 0);

	utf8 = "aaaabbbaaabbbbbbaaabbbbbbbbabababababababaaaaaa";
	expected = "hZwbbbhZwbbbbbbhZwbbbbbbbbbbbbbbb";
	assert(strlen(expected) == emojiReplace1(utf8, strlen(utf8), NULL, 0));
	assert(strlen(expected) == emojiReplace1(utf8, strlen(utf8), runes, strlen(expected) + 1));
	assert(strcmp(expected, runes) == 0);

	utf8 = "babaabaaab";
	expected = "bhZwbhZwbhZwb";
	out = emojiReplace2(utf8, strlen(utf8));
	assert(strcmp(expected, out) == 0);
	HzwFree(out);

	utf8 = "aaaabbbaaabbbbbbaaabbbbbbbbabababababababaaaaaa";
	expected = "hZwbbbhZwbbbbbbhZwbbbbbbbbbbbbbbb";
	out = emojiReplace2(utf8, strlen(utf8));
	assert(strcmp(expected, out) == 0);
	HzwFree(out);
}
