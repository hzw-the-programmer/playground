```
void hzwDebug(const uint8 *buf, int len) {
	char str[2048] = {0};
	char *strp = str;
	uint8 *bufp = buf;
	int slen = 0;
	char *fmt = "0x%02x, ";

	SCI_TRACE_LOW("{");

	do {
		if (bufp == buf + len - 1) fmt = "0x%02x";

		slen = sprintf(strp, fmt, *bufp++);
		strp += slen;

		if (!((bufp - buf) % 10)) {
			*strp = 0;
			SCI_TRACE_LOW("%s", str);
			strp = str;
		}
	} while (bufp < buf + len);

	if (strp != str) {
		*strp = 0;
		SCI_TRACE_LOW("%s", str);
	}

	SCI_TRACE_LOW("}");
}
```
