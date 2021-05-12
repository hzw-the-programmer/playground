#if 1
typedef struct {
	char *key;
	char *value;
} KeyValue;

int isHolderStart(char *c) {
	return *c == 0x7b && *(c+1) == 0x00;
}

int isHolderEnd(char *c) {
	return *c == 0x7d && *(c+1) == 0x00;
}

int isStrEnd(char *c) {
	return *c == 0x00 && *(c+1) == 0x00;
}

int isCharEqual(char *a, char *b) {
	return *a == *b && *(a+1) == *(b+1);
}

void replace(char *t, int tl, KeyValue *kvs, int kvsl, char *o, int ol) {
	int ti = 0, oi = 0, kvsi = 0, s = 0, e = 0, i = 0;
	char *key, *value;

	while (ti < tl && oi < ol) {
		if (isHolderStart(t+ti)) {
			s = e = ti;

			while (e < tl && !isHolderEnd(t+e)) e+=2;

			if (e < tl && isHolderEnd(t+e)) {
				s+=2;
				while (kvsi < kvsl) {
					key = kvs[kvsi].key;
					i = 0;
					while (!isStrEnd(key) && s+i < e && isCharEqual(key, t+s+i)) {
						key+=2;
						i+=2;
					}
					if (isStrEnd(key) && s+i == e) {
						value = kvs[kvsi].value;
						while (!isStrEnd(value)) {
							*(o+oi) = *value;
							*(o+oi+1) = *(value+1);
							oi+=2;
							value+=2;
						}
						ti+=e-s+2*2;
						break;
					}
					kvsi++;
				}
				continue;
			}
		}

		o[oi] = t[ti];
		o[oi+1] = t[ti+1];
		oi+=2;
		ti+=2;
	}
}

void test() {
	char *t = L"{1} is invited by {11}, revard {11} $500.";
	int tl = strlen(t);
	KeyValue kvs[2] = {
		{
			L"1",
			L"hzw",
		},
		{
			L"11",
			L"yk",
		},
	};
	int kvsl = sizeof(kvs) / sizeof(KeyValue);
	char o[512] = {0};
	int ol = sizeof(o);
	replace(t, 82, kvs, kvsl, o, ol);

	{
		int a = 1 + 1;
	}
}
#endif
