#include "string.h" // memset, strlen, strcmp
#include "assert.h" // assert

typedef struct {
	char *key;
	char *value;
} KeyValue;

int replace(char *in, int in_len, KeyValue *kvs, int kvs_len, char *out, int out_len) {
	int in_index = 0;
	int out_index = 0;
	int start_index = -1;
	int kvs_index = 0;
	int k_index = 0;
	char *k = NULL;
	char *v = NULL;

	while (in_index < in_len) {
		if (out_index < out_len) {
			out[out_index] = in[in_index];
		}
		out_index++;
		in_index++;

		if (in[in_index - 1] == '{' && start_index == -1) {
			start_index = in_index - 1;
		} else if (in[in_index - 1] == '}' && start_index != -1) {
			for (kvs_index = 0; kvs_index < kvs_len; kvs_index++) {
				k = kvs[kvs_index].key;
				k_index = start_index + 1;

				while (*k && *k == in[k_index]) {
					k++;
					k_index++;
				}

				if (!*k && in[k_index] == '}') {
					v = kvs[kvs_index].value;
					out_index -= in_index - start_index;

					while (*v) {
						if (out_index < out_len) {
							out[out_index] = *v;
						}
						out_index++;
						v++;
					}
					
					break;
				}
			}

			start_index = -1;
		}
	}

	if (out_index < out_len) {
		out[out_index] = 0;
	} else if (out_len > 0) {
		out[out_len - 1] = 0;
	}

	return out_index;
}

void test_template() {
	char out[512] = {0};
	char *in = "{user1} knows {user2}, {user3} but {user2} does not know {user1}";
	KeyValue kvs[] = {
		{
			"user1",
			"hezhiwen",
		},
		{
			"user2",
			"zhangyiming",
		},
	};
	int len = 0;
	char *expected = "hezhiwen knows zhangyiming, {user3} but zhangyiming does not know hezhiwen";

	memset(out, 0xff, sizeof(out));
	len = replace(in, strlen(in), kvs, sizeof(kvs) / sizeof(kvs[0]), out, sizeof(out));
	assert(len == strlen(expected));
	assert(strcmp(out, expected) == 0);

	len = replace(in, strlen(in), kvs, sizeof(kvs) / sizeof(kvs[0]), NULL, 0);
	assert(len == strlen(expected));
}
