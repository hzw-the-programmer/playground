void replace(char *t, int tl, KeyValue *kvs, int kvsl, char *o, int ol) {
	int ti = 0, oi = 0, kvsi = 0, len = 0, i = 0;

	while (ti < tl && oi < ol) {
		if (t[ti] == '$' && ti < tl - 1) {
			if (t[ti+1] == '$') {
				o[oi] = '$';
				oi+=1;
				ti+=2;
			} else {
				kvsi = 0;
				while (kvsi < kvsl) {
					char *key1 = kvs[kvsi].key;
					char *key2 = kvs[kvsi].key;

					while (*key1 != 0 && *key1 == t[ti+(key1-key2)]) {
						key1++;
					}

					if (*key1 == 0) {
						char *value = kvs[kvsi].value;

						while (*value != 0) {
							o[oi++] = *value++;
						}

						ti+=key1 - key2;
						break;
					}
					kvsi++;
				}
			}
		} else {
			o[oi] = t[ti];
			oi+=1;
			ti+=1;
		}
	}
}
