#include <stdarg.h> // for va_list, va_start, va_end
#include <assert.h> // for assert

typedef enum {
	STATE_NONE,
	STATE_PERCENT,
} STATE_E;

static int itoa(int value, char *result, int base) {
	char *chars = "zyxwvutsrqponmlkjihgfedcba9876543210123456789abcdefghijklmnopqrstuvwxyz";
	int temp_value, len;
	char *start = result, *end = result, temp_char;

	if (base < 2 || base > 36) {
		*result = '\0';
		return 0;
	}

	do {
		temp_value = value;
		value /= base;
		*end++ = chars[35 + temp_value - value * base];
	} while (value);

	if (temp_value < 0) {
		*end++ = '-';
	}

	len = end - start;

	*end-- = 0;

	while (start < end) {
		temp_char = *start;
		*start++ = *end;
		*end-- = temp_char;
	}

	return len;
}

void wsprintf(short *out, char *format, ...) {
	va_list args;
	STATE_E state = STATE_NONE;
	
	va_start(args, format);

	while (*format) {
		char c = *format++;
		switch (c) {
			case '%':
				if (state == STATE_PERCENT) {
					*out++ = '%';
					state = STATE_NONE;
				} else {
					state = STATE_PERCENT;
				}
				break;
			
			case 'w':
				if (state == STATE_PERCENT) {
					short *wstr = va_arg(args, short*);
					while (*wstr) {
						*out++ = *wstr++;
					}
					state = STATE_NONE;
				} else {
					*out++ = c;
				}
				break;

			case 'd':
				if (state == STATE_PERCENT) {
					 long num = va_arg(args, long);
					 char numstr[20] = {0};
					 char *pnum = numstr;
					 int len = itoa(num, numstr, 10);
					 while (*pnum) {
						*out++ = *pnum++;
					 }
					state = STATE_NONE;
				} else {
					*out++ = c;
				}
				break;

			case 's':
				if (state == STATE_PERCENT) {
					char *str = va_arg(args, char*);
					while (*str) {
						*out++ = *str++;
					}
					state = STATE_NONE;
				} else {
					*out++ = c;
				}
				break;

			case 'c':
				if (state == STATE_PERCENT) {
					char c = va_arg(args, char);
					*out++ = c;
					state = STATE_NONE;
				} else {
					*out++ = c;
				}
				break;

			default:
				*out++ = c;
				break;
		}
	}

	va_end(args);

	if (state == STATE_PERCENT) {
		*out++ = '%';
	}

	*out = 0;
}

void assert_equal(short *a, short *b) {
	while (*a) {
		assert(*a++ == *b++);
	}
	assert(*b == 0);
}

static void test1() {
	short out[512];
	for (int i = 0; i < sizeof(out) / sizeof(*out); i++) {
		out[i] = 0xff;
	}
	
	char *format = "%w lo%%w\\ve \n%w";
	short wstr1[] = {'h', 'z', 'w', '\0'};
	short wstr2[] = {'g', 'o', 'l', 'a', 'n', 'g', '\0'};
	
	short expect[] = {'h', 'z', 'w', ' ', 'l', 'o', '%', 'w', '\\', 'v', 'e', ' ', '\n', 'g', 'o', 'l', 'a', 'n', 'g', '\0'};

	wsprintf(out, format, wstr1, wstr2);
	assert_equal(out, expect);
}

static void test2() {
	short out[512];
	for (int i = 0; i < sizeof(out) / sizeof(*out); i++) {
		out[i] = 0xff;
	}
	
	char *format = "%w [%d]";
	short wstr1[] = {'h', 'z', 'w', '\0'};
	int num = 100;
	
	short expect[] = {'h', 'z', 'w', ' ', '[', '1', '0', '0', ']', '\0'};

	wsprintf(out, format, wstr1, num);
	assert_equal(out, expect);
}

static void test3() {
	short out[512];
	for (int i = 0; i < sizeof(out) / sizeof(*out); i++) {
		out[i] = 0xff;
	}
	
	char *format = "%d%w";
	int num = 100;
	short wstr1[] = {'h', 'z', 'w', '\0'};
	
	short expect[] = {'1', '0', '0', 'h', 'z', 'w', '\0'};

	wsprintf(out, format, num, wstr1);
	assert_equal(out, expect);
}

static void test4() {
	short out[512];
	for (int i = 0; i < sizeof(out) / sizeof(*out); i++) {
		out[i] = 0xff;
	}
	
	char *format = "%c:\\%s\\%s\\%s\\%s.%s";
	char c = 'C';
	char *s1 = "a";
	char *s2 = "bc";
	char *s3 = "def";
	char *s4 = "ghij";
	char *s5 = "klmno";
	
	short expect[] = {'C', ':', '\\', 'a', '\\', 'b', 'c', '\\', 'd', 'e', 'f', '\\', 'g', 'h', 'i', 'j', '.', 'k', 'l', 'm', 'n', 'o', '\0'};

	wsprintf(out, format, c, s1, s2, s3, s4, s5);
	assert_equal(out, expect);
}

static void test5() {
	short out[512];
	for (int i = 0; i < sizeof(out) / sizeof(*out); i++) {
		out[i] = 0xff;
	}
	
	char *format = "%d:%02d";
	int num1 = 108;
	int num2 = 8;
	
	short expect[] = {'1', '0', '8', ':', '0', '8', '\0'};

	wsprintf(out, format, num1, num2);
	assert_equal(out, expect);
}

void test_wsprintf() {
	test1();
	test2();
	test3();
	test4();
	//test5();
}
