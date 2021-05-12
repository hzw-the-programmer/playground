void replace(char *t, int tl, KeyValue *kvs, int kvsl, char *o, int ol) {
	int ti = 0, oi = 0, kvsi = 0, s = 0, e = 0, i = 0;
	char *key, *value;

	while (ti < tl && oi < ol) {
		if (t[ti] == '{') {
			s = e = ti;
			while (e < tl && t[e] != '}') e++;
			if (e < tl && t[e] == '}') {
				s+=1;
				while (kvsi < kvsl) {
					key = kvs[kvsi].key;
					i = 0;
					while (*key != 0 && s+i < e && *key == t[s+i]) {
						key++;
						i++;
					}
					if (*key == 0 && s+i == e) {
						value = kvs[kvsi].value;
						while (*value != 0) {
							o[oi++] = *value++;
						}
						ti+=e-s+2;
						break;
					}
					kvsi++;
				}
				continue;
			}
		}

		o[oi++] = t[ti++];
	}
}
