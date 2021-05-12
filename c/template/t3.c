#if 1

#define HOLDER_START 0x007b
#define HOLDER_END 0x007d

typedef struct {
	unsigned short *key;
	unsigned short *value;
} KeyValue;

void replace_template_parameters(
	unsigned short *template, int template_len,
	KeyValue *parameters, int parameters_len,
	unsigned short *output_buf, int output_buf_len)
{
	int template_index = 0;
	int parameters_index = 0;
	int output_buf_index = 0;
	int holder_start = 0;
	int holder_end = 0;
	int index = 0;
	unsigned short *key = NULL;
	unsigned short *value = NULL;

	while (template_index < template_len
		&& output_buf_index < output_buf_len)
	{
		if (template[template_index] == HOLDER_START)
		{
			holder_start = holder_end = template_index;

			while (holder_end < template_len
				&& template[holder_end] != HOLDER_END)
			{
				holder_end++;
			}

			if (holder_end < template_len
				&& template[holder_end] == HOLDER_END)
			{
				parameters_index = 0;

				while (parameters_index < parameters_len)
				{
					key = parameters[parameters_index].key;
					index = 1;

					while (*key != 0
						&& (holder_start + index) < holder_end
						&& *key == template[holder_start + index])
					{
						key++;
						index++;
					}

					if (*key == 0
						&& holder_start + index == holder_end)
					{
						value = parameters[parameters_index].value;

						while (*value != 0)
						{
							output_buf[output_buf_index++] = *value++;
						}

						template_index += holder_end - holder_start + 1;

						break;
					}
						
					parameters_index++;
				}

				continue;
			}
		}

		output_buf[output_buf_index++] = template[template_index++];
	}
}

int wstrlen(unsigned short *w) {
	int i = 0;
	while (*w++ != 0) i++;
	return i;
}

int wstrequal(unsigned short *w1, unsigned short *w2) {
	while (*w1 != 0 && *w2 != 0 && *w1 == *w2) {
		w1++;
		w2++;
	}
	return *w1 == 0 && *w2 == 0;
}

typedef struct {
	unsigned short *template;
	KeyValue *parameters;
	int parameters_len;
	unsigned short *result;
} test_struct;

void test() {
	unsigned short *template1 = L"{1} is invited by {11}, revard {11} $500.";
	KeyValue parameters1[] = {
		{
			L"1",
			L"hzw",
		},
		{
			L"11",
			L"yk",
		},
	};
	int parameters1_len = sizeof(parameters1) / sizeof(parameters1[0]);
	unsigned short *result1 = L"hzw is invited by yk, revard yk $500.";

	unsigned short *template2 = L"hello {user}! welcome to {company}. {company} loves {user}.";
		KeyValue parameters2[] = {
			{
				L"user",
				L"hzw",
			},
			{
				L"company",
				L"bd",
			},
		};
	int parameters2_len = sizeof(parameters2) / sizeof(parameters2[0]);
	unsigned short *result2 = L"hello hzw! welcome to bd. bd loves hzw.";

	test_struct tests[] = {
		{
			template1,
			parameters1,
			parameters1_len,
			result1,
		},
		{
			template2,
			parameters2,
			parameters2_len,
			result2,
		},
	};

	int test_index = 0;
	for (; test_index < sizeof(tests) / sizeof(tests[0]); test_index++) {
		unsigned short *template = tests[test_index].template;
		int template_len = wstrlen(template);
		KeyValue *parameters = tests[test_index].parameters;
		int parameters_len = tests[test_index].parameters_len;
		unsigned short *result = tests[test_index].result;
		unsigned short output_buf[512] = {0};
		int output_buf_len = sizeof(output_buf) / sizeof(output_buf[0]);
		int pass = 0;
		replace_template_parameters(
				template,
				template_len,
				parameters,
				parameters_len,
				output_buf,
				output_buf_len);

		if (wstrequal(output_buf, result)) {
			pass = 1;
		}
	}
}
#endif
