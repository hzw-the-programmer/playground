#include <stdlib.h> // malloc
#include <string.h> // memcpy
#include <stdio.h>

#define TRUE 1
#define FALSE 0
#define MAX_STR_COUNT 500

static const char english[] = {
	'h', 'e', ' ', 'z', 'h', 'i', ' ', 'w', ' e', 'n', 0,
	'g', 'o', ' ', 'm', 'a', 's', 't', 'e', 'r', 0,
	0,
	0,
	'g', 'i', 't', ' ', 'm', 'a', 's', 't', 'e', 'r', 0,
};

static wchar_t *gWstrs = NULL;
static int gWstrsLen = 0;

static int *gOffsets = NULL;
static int gOffsetsLen = 0;

static int loadStrsFromRom(wchar_t **wstrs, int *len) {
	int _len = 0;
	const char *start = NULL, *end = NULL;
	wchar_t *_wstrs = NULL;
	int i = 0;

	*wstrs = NULL;
	*len = 0;

	_len = sizeof(english) / sizeof(english[0]);
	start = english;
	end = start + _len;

	_wstrs = (wchar_t*)malloc(sizeof(wchar_t) * _len);
	if (!_wstrs) {
		return FALSE;
	}

	*wstrs = _wstrs;
	*len = _len;
	while (start < end) {
		*_wstrs++ = *start++;
	}

	return TRUE;
}

static int loadStrsFromFile(wchar_t **wstrs, int *len) {
	FILE *file = NULL;
	wchar_t *_wstrs = NULL;
	int _len = 0, ret = 0;

	*wstrs = NULL;
	*len = 0;

	file = fopen("D:\\vc\\str\\Debug\\english", "r");
	if (!file) {
		ret = FALSE;
		goto end;
	}

	ret = fseek(file, 0, SEEK_END);
	if (ret) {
		ret = FALSE;
		goto end;
	}

	_len = ftell(file);
	if (!_len) {
		ret = FALSE;
		goto end;
	}

	ret = fseek(file, 0, SEEK_SET);
	if (ret) {
		ret = FALSE;
		goto end;
	}

	_wstrs = (wchar_t*)malloc(_len);
	if (!_wstrs) {
		ret = FALSE;
		goto end;
	}

	ret = fread(_wstrs, 1, _len, file);
	if (ret != _len) {
		ret = FALSE;
		goto end;
	}

	*wstrs = _wstrs;
	*len = _len / 2;

	_wstrs = NULL;

end:
	if (file) {
		fclose(file);
	}

	if (_wstrs) {
		free(_wstrs);
	}

	return ret;
}

static int calcOffsets(wchar_t *wstrs, int wstrsLen, int **offsets, int *offsetsLen) {
	int _offsets[MAX_STR_COUNT] = {0};
	int _len = 0;
	int preZero = FALSE;
	int i = 0;
	wchar_t wc;

	*offsets = NULL;
	*offsetsLen = 0;

	preZero = TRUE;
	for (i = 0; i < wstrsLen; i++) {
		if (preZero) {
			if (_len < MAX_STR_COUNT) {
				_offsets[_len++] = i;
			}
		}
		preZero = wstrs[i] == 0;
	}

	*offsets = (int*)malloc(sizeof(int) * _len);
	if (!*offsets) {
		return FALSE;
	}
	memcpy(*offsets, _offsets, sizeof(int) * _len);
	*offsetsLen = _len;

	return TRUE;
}

wchar_t* GetWstr(int index) {
	if (index < 0 || index > gOffsetsLen - 1) {
		return NULL;
	}
	return gWstrs + gOffsets[index];
}

void InitStrs() {
	wchar_t *wstrs = NULL;
	int wstrsLen = 0;
	int *offsets = NULL;
	int offsetsLen = 0;
	int ret = FALSE;

	if (0) {
		ret = loadStrsFromRom(&wstrs, &wstrsLen);
	} else {
		ret = loadStrsFromFile(&wstrs, &wstrsLen);
	}

	if (!ret) {
		return;
	}

	ret = calcOffsets(wstrs, wstrsLen, &offsets, &offsetsLen);

	if (!ret) {
		return;
	}

	gWstrs = wstrs;
	gWstrsLen = wstrsLen;

	gOffsets = offsets;
	gOffsetsLen = offsetsLen;
}

int main(int argc, char **argv) {
	wchar_t *wstr = NULL;
	int i = 0;
	
	InitStrs();

	for (i = 0; i < 10; i++) {
		wstr = GetWstr(i);
	}
}
