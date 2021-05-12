#include <assert.h> // for assert
#include <stdarg.h> // for va_list, va_start, va_end
#include <string.h> // for memcpy, memset, memmove, strcmp

#define SIGN 0x01
#define LEFT 0x02
#define PLUS 0x04
#define SPACE 0x08
#define ZEROPAD 0x10
#define SMALL 0x20
#define SPECIAL 0x40

enum format_type {
	FORMAT_TYPE_NONE,
	FORMAT_TYPE_CHAR,
	FORMAT_TYPE_STR,
	FORMAT_TYPE_PERCENT_CHAR,
	FORMAT_TYPE_UBYTE,
	FORMAT_TYPE_BYTE,
	FORMAT_TYPE_USHORT,
	FORMAT_TYPE_SHORT,
	FORMAT_TYPE_UINT,
	FORMAT_TYPE_INT,
	FORMAT_TYPE_ULONG,
	FORMAT_TYPE_LONG,
	FORMAT_TYPE_LONG_LONG,
};

struct printf_spec {
	unsigned int type;
	signed int field_width;
	unsigned int flags;
	unsigned int base;
	signed int precision;
};

char hex_asc_upper[] = "0123456789ABCDEF";

int isdigit(int c) {
	return c >= '0' && c <= '9';
}

int skip_atoi(const char **s) {
	int i  = 0;

	do {
		i = i * 10 + *((*s)++) - '0';
	} while(isdigit(**s));

	return i;
}

int format_decode(const char *fmt, struct printf_spec *spec) {
	const char *start = fmt;
	char qualifier;

	spec->type = FORMAT_TYPE_NONE;

	for (; *fmt; ++fmt) {
		if (*fmt == '%') {
			break;
		}
	}

	if (fmt != start || !*fmt) {
		return fmt - start;
	}

	spec->flags = 0;

	while (1) {
		bool found = true;

		++fmt;

		switch (*fmt) {
			case '-':
				spec->flags |= LEFT;
				break;
			case '+':
				spec->flags |= PLUS;
				break;
			case ' ':
				spec->flags |= SPACE;
				break;
			case '#':
				spec->flags |= SPECIAL;
				break;
			case '0':
				spec->flags |= ZEROPAD;
				break;
			default:
				found = false;
				break;
		}

		if (!found) {
			break;
		}
	}

	spec->field_width = -1;

	if (isdigit(*fmt)) {
		spec->field_width = skip_atoi(&fmt);
	}

	spec->precision = -1;
	if (*fmt == '.') {
		++fmt;
		if (isdigit(*fmt)) {
			spec->precision = skip_atoi(&fmt);
		}
	}

	qualifier = 0;
	if (*fmt == 'l' || *fmt == 'h') {
		qualifier = *fmt++;
		if (qualifier == *fmt) {
			if (qualifier == 'l') {
				qualifier = 'L';
				++fmt;
			} else if (qualifier == 'h') {
				qualifier = 'H';
				++fmt;
			}
		}
	}

	spec->base = 10;
	switch (*fmt) {
		case 'c':
			spec->type = FORMAT_TYPE_CHAR;
			return ++fmt - start;

		case 's':
			spec->type = FORMAT_TYPE_STR;
			return ++fmt - start;

		case '%':
			spec->type = FORMAT_TYPE_PERCENT_CHAR;
			return ++fmt - start;

		case 'o':
			spec->base = 8;
			break;

		case 'x':
			spec->flags |= SMALL;
		case 'X':
			spec->base = 16;
			break;

		case 'd':
		case 'i':
			spec->flags |= SIGN;
		case 'u':
			break;
	}

	if (qualifier == 'L') {
		spec->type = FORMAT_TYPE_LONG_LONG;
	} else if (qualifier == 'l') {
		spec->type = FORMAT_TYPE_ULONG + (spec->flags & SIGN);
	} else if (qualifier == 'H') {
		spec->type = FORMAT_TYPE_UBYTE + (spec->flags & SIGN);
	} else if (qualifier == 'h') {
		spec->type = FORMAT_TYPE_USHORT + (spec->flags & SIGN);
	} else {
		spec->type = FORMAT_TYPE_UINT + (spec->flags & SIGN);
	}

	return ++fmt - start;
}

void move_right(char *buf, char *end, unsigned len, unsigned spaces) {
	size_t size;
	if (buf >= end) {
		return;
	}
	size = end - buf;
	if (size <= spaces) {
		memset(buf, ' ', size);
		return;
	}
	if (len) {
		if (len > size - spaces) {
			len = size - spaces;
		}
		memmove(buf + spaces, buf, len);
	}
	memset(buf, ' ', spaces);
}

char* widen_string(char *buf, int n, char *end, struct printf_spec spec) {
	unsigned spaces;

	if (n >= spec.field_width) {
		return buf;
	}
	spaces = spec.field_width - n;
	if (!(spec.flags & LEFT)) {
		move_right(buf - n, end, n, spaces);
		return buf + spaces;
	}
	while (spaces--) {
		if (buf < end) {
			*buf = ' ';
		}
		++buf;
	}
	return buf;
}

char* string_nocheck(char *buf, char *end, const char *s, struct printf_spec spec) {
	int len = 0;
	int lim = spec.precision;

	while (lim--) {
		char c = *s++;
		if (!c) {
			break;
		}
		if (buf < end) {
			*buf = c;
		}
		++buf;
		++len;
	}
	return widen_string(buf, len, end, spec);
}

char* string(char *buf, char *end, const char *s, struct printf_spec spec) {
	return string_nocheck(buf, end, s, spec);
}

char* put_dec(char *buf, unsigned long long n) {
	do {
		*buf++ = n % 10;
		n /= 10;
	} while(n);
	return buf;
}

char* number(char *buf, char *end, unsigned long long num, struct printf_spec spec) {
	char tmp[3 * sizeof(num)];
	char sign;
	char locase;
	int need_pfx = (spec.flags & SPECIAL) && spec.base != 10;
	int i;
	bool is_zero = num == 0LL;
	int field_width = spec.field_width;
	int precision = spec.precision;

	locase = spec.flags & SMALL;
	
	if (spec.flags & LEFT) {
		spec.flags &= ~ZEROPAD;
	}

	sign = 0;
	if (spec.flags & SIGN) {
		if ((signed long long)num < 0) {
			sign = '-';
			num = -(signed long long)num;
			field_width--;
		} else if (spec.flags & PLUS) {
			sign = '+';
			field_width--;
		} else if (spec.flags & SPACE) {
			sign = ' ';
			field_width--;
		}
	}

	if (need_pfx) {
		if (spec.base == 16) {
			field_width -= 2;
		} else if (!is_zero) {
			field_width--;
		}
	}

	i = 0;
	if (num < spec.base) {
		tmp[i++] = hex_asc_upper[num] | locase;
	} else if (spec.base != 10) {
		int mask = spec.base - 1;
		int shift = 3;
		if (spec.base == 16) {
			shift = 4;
		}
		do {
			tmp[i++] = hex_asc_upper[((unsigned char)num) & mask] | locase;
			num >>= shift;
		} while(num);
	} else {
		i = put_dec(tmp, num) - tmp;
	}
	
	if (i > precision) {
		precision = i;
	}

	field_width -= precision;

	if (!(spec.flags & (LEFT | ZEROPAD))) {
		while (--field_width >= 0) {
			if (buf < end) {
				*buf = ' ';
			}
			++buf;
		}
	}

	if (sign) {
		if (buf < end) {
			*buf = sign;
		}
		++buf;
	}

	if (need_pfx) {
		if (spec.base == 16 || !is_zero) {
			if (buf < end) {
				*buf = '0';
			}
			++buf;
		}
		if (spec.base == 16) {
			if (buf < end) {
				*buf = 'X' | locase;
			}
			++buf;
		}
	}

	if (!(spec.flags & LEFT)) {
		char c = ' ' + (spec.flags & ZEROPAD);
		while (--field_width >= 0) {
			if (buf < end) {
				*buf = c;
			}
			++buf;
		}
	}

	while (i <= --precision) {
		if (buf < end) {
			*buf = '0';
		}
		++buf;
	}

	while (--i >= 0) {
		if (buf < end) {
			*buf = tmp[i];
		}
		++buf;
	}

	while (--field_width >= 0) {
		if (buf < end) {
			*buf = ' ';
		}
		++buf;
	}

	return buf;
}

int hzw_vsnprintf(char *buf, size_t size, const char *fmt, va_list args) {
	unsigned long long num;
	char *str, *end;
	struct printf_spec spec = {0};

	str = buf;
	end = buf + size;

	while (*fmt) {
		const char *old_fmt = fmt;
		int read = format_decode(fmt, &spec);
		
		fmt += read;

		switch(spec.type) {
			case FORMAT_TYPE_NONE: {
				int copy = read;
				if (str < end) {
					if (copy > end - str) {
						copy = end - str;
					}
					memcpy(str, old_fmt, copy);
				}
				str += read;
				break;
			}

			case FORMAT_TYPE_CHAR: {
				char c;

				if (!(spec.flags & LEFT)) {
					while (--spec.field_width > 0) {
						if (str < end) {
							*str = ' ';
						}
						++str;
					}
				}

				c = (unsigned char)va_arg(args, int);
				if (str < end) {
					*str = c;
				}
				++str;

				while (--spec.field_width > 0) {
					if (str < end) {
						*str = ' ';
					}
					++str;
				}
				break;
			}

			case FORMAT_TYPE_STR:
				str = string(str, end, va_arg(args, char*), spec);
				break;

			case FORMAT_TYPE_PERCENT_CHAR:
				if (str < end) {
					*str = '%';
				}
				++str;
				break;

			default:
				switch (spec.type) {
					case FORMAT_TYPE_LONG_LONG:
						num = va_arg(args, long long);
						break;
					case FORMAT_TYPE_ULONG:
						num = va_arg(args, unsigned long);
						break;
					case FORMAT_TYPE_LONG:
						num = va_arg(args, long);
						break;
					case FORMAT_TYPE_UBYTE:
						num = (unsigned char)va_arg(args, int);
						break;
					case FORMAT_TYPE_BYTE:
						num = (char)va_arg(args, int);
						break;
					case FORMAT_TYPE_USHORT:
						num = (unsigned short)va_arg(args, int);
						break;
					case FORMAT_TYPE_SHORT:
						num = (short)va_arg(args, int);
						break;
					case FORMAT_TYPE_INT:
						num = va_arg(args, int);
						break;
					default:
						num = va_arg(args, unsigned int);
						break;
				}

				str = number(str, end, num, spec);
				break;
		}
	}

	if (str < end) {
		*str = '\0';
	} else {
		end[-1] = '\0';
	}

	return str - buf;
}

int hzw_snprintf(char *buf, size_t size, const char *fmt, ...) {
	va_list args;
	int i;

	va_start(args, fmt);
	i = hzw_vsnprintf(buf, size, fmt, args);
	va_end(args);

	return i;
}

void test_hzw_snprintf() {
	char formatted[512];
	for (int i = 0; i < 512; i++) {
		formatted[i] = 0xff;
	}
	int len;

	char *fmt;
	char *expected;

	expected = "abcdefg";
	len = hzw_snprintf(formatted, 512, "abcdefg");
	assert(strcmp(formatted, expected) == 0);
	assert(len == 7);

	expected = "ab";
	len = hzw_snprintf(formatted, 3, "abcdefg");
	assert(strcmp(formatted, expected) == 0);
	assert(len == 7);

	expected = "a";
	len = hzw_snprintf(formatted, 512, "%c", 'a');
	assert(strcmp(formatted, expected) == 0);
	assert(len == 1);

	expected = "         a";
	len = hzw_snprintf(formatted, 512, "%10c", 'a');
	assert(strcmp(formatted, expected) == 0);
	assert(len == 10);

	expected = "a         ";
	len = hzw_snprintf(formatted, 512, "%-10c", 'a');
	assert(strcmp(formatted, expected) == 0);
	assert(len == 10);

	expected = "";
	len = hzw_snprintf(formatted, 1, "%c", 'a');
	assert(strcmp(formatted, expected) == 0);
	assert(len == 1);

	expected = "    ";
	len = hzw_snprintf(formatted, 5, "%10c", 'a');
	assert(strcmp(formatted, expected) == 0);
	assert(len == 10);

	expected = "a      ";
	len = hzw_snprintf(formatted, 8, "%-10c", 'a');
	assert(strcmp(formatted, expected) == 0);
	assert(len == 10);

	// %s
	{
		// enough space
		expected = "abcd";
		len = hzw_snprintf(formatted, 512, "a%sd", "bc");
		assert(strcmp(formatted, expected) == 0);
		assert(len == 4);

		expected = "a        bcd";
		len = hzw_snprintf(formatted, 512, "a%10sd", "bc");
		assert(strcmp(formatted, expected) == 0);
		assert(len == 12);

		expected = "abc        d";
		len = hzw_snprintf(formatted, 512, "a%-10sd", "bc");
		assert(strcmp(formatted, expected) == 0);
		assert(len == 12);

		expected = "a       bcdh";
		len = hzw_snprintf(formatted, 512, "a%10.3sh", "bcdefg");
		assert(strcmp(formatted, expected) == 0);
		assert(len == 12);

		expected = "abcd       h";
		len = hzw_snprintf(formatted, 512, "a%-10.3sh", "bcdefg");
		assert(strcmp(formatted, expected) == 0);
		assert(len == 12);

		// not enough space
		expected = "a";
		len = hzw_snprintf(formatted, 2, "a%sd", "bc");
		assert(strcmp(formatted, expected) == 0);
		assert(len == 4);

		expected = "a   ";
		len = hzw_snprintf(formatted, 5, "a%10sd", "bc");
		assert(strcmp(formatted, expected) == 0);
		assert(len == 12);
	}

	// %%
	{
		expected = "a%bc";
		len = hzw_snprintf(formatted, 512, "a%%%s", "bc");
		assert(strcmp(formatted, expected) == 0);
		assert(len == 4);
	}
}

void test_format_decode() {
	char *fmt = "abcd%02defg%dh%1234dijklmn%014d%02ldhehe%14180lld%02hd%14180hhd%-10d%-010d%0-10d"
		"abcd%02iefg%ih%1234iijklmn%014i%02lihehe%14180lli%02hi%14180hhi"
		"abcd%02uefg%uh%1234uijklmn%014u%02luhehe%14180llu%02hu%14180hhu"
		"abcd%02oefg%oh%1234oijklmn%014o%02lohehe%14180llo%02ho%14180hho"
		"abcd%02Xefg%Xh%1234Xijklmn%014X%02lXhehe%14180llX%02hX%14180hhX"
		"abcd%02xefg%xh%1234xijklmn%014x%02lxhehe%14180llx%02hx%14180hhx"
		"%c%14180c%-14180c";
	struct printf_spec spec = {0};
	int read = 0;
	
	{
		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 4);
		fmt += 4;

		// %02d
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(spec.flags & SIGN);
		assert(spec.type == FORMAT_TYPE_INT);
		assert(spec.field_width == 2);
		assert(!(spec.flags & LEFT));
		assert(spec.flags & ZEROPAD);
		assert(read == 4);
		fmt += 4;

		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 3);
		fmt += 3;

		// %d
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(spec.flags & SIGN);
		assert(spec.type == FORMAT_TYPE_INT);
		assert(spec.field_width == -1);
		assert(!(spec.flags & LEFT));
		assert(!(spec.flags & ZEROPAD));
		assert(read == 2);
		fmt += 2;

		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 1);
		fmt += 1;

		// %1234d
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(spec.flags & SIGN);
		assert(spec.type == FORMAT_TYPE_INT);
		assert(spec.field_width == 1234);
		assert(!(spec.flags & LEFT));
		assert(!(spec.flags & ZEROPAD));
		assert(read == 6);
		fmt += 6;

		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 6);
		fmt += 6;

		// %014d
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(spec.flags & SIGN);
		assert(spec.type == FORMAT_TYPE_INT);
		assert(spec.field_width == 14);
		assert(!(spec.flags & LEFT));
		assert(spec.flags & ZEROPAD);
		assert(read == 5);
		fmt += 5;

		// %02ld
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(spec.flags & SIGN);
		assert(spec.type == FORMAT_TYPE_LONG);
		assert(spec.field_width == 2);
		assert(!(spec.flags & LEFT));
		assert(spec.flags & ZEROPAD);
		assert(read == 5);
		fmt += 5;

		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 4);
		fmt += 4;

		// %14180lld
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(spec.flags & SIGN);
		assert(spec.type == FORMAT_TYPE_LONG_LONG);
		assert(spec.field_width == 14180);
		assert(!(spec.flags & LEFT));
		assert(!(spec.flags & ZEROPAD));
		assert(read == 9);
		fmt += 9;

		// %02hd
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(spec.flags & SIGN);
		assert(spec.type == FORMAT_TYPE_SHORT);
		assert(spec.field_width == 2);
		assert(!(spec.flags & LEFT));
		assert(spec.flags & ZEROPAD);
		assert(read == 5);
		fmt += 5;

		// %14180hhd
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(spec.flags & SIGN);
		assert(spec.type == FORMAT_TYPE_BYTE);
		assert(spec.field_width == 14180);
		assert(!(spec.flags & LEFT));
		assert(!(spec.flags & ZEROPAD));
		assert(read == 9);
		fmt += 9;

		// %-10d
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(spec.flags & SIGN);
		assert(!(spec.flags & SMALL));
		assert(spec.type == FORMAT_TYPE_INT);
		assert(spec.field_width == 10);
		assert(spec.flags & LEFT);
		assert(!(spec.flags & ZEROPAD));
		assert(read == 5);
		fmt += 5;

		// %-010d
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(spec.flags & SIGN);
		assert(!(spec.flags & SMALL));
		assert(spec.type == FORMAT_TYPE_INT);
		assert(spec.field_width == 10);
		assert(spec.flags & LEFT);
		assert(spec.flags & ZEROPAD);
		assert(read == 6);
		fmt += 6;

		// %0-10d
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(spec.flags & SIGN);
		assert(!(spec.flags & SMALL));
		assert(spec.type == FORMAT_TYPE_INT);
		assert(spec.field_width == 10);
		assert(spec.flags & LEFT);
		assert(spec.flags & ZEROPAD);
		assert(read == 6);
		fmt += 6;
	}

	{
		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 4);
		fmt += 4;

		// %02i
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(spec.flags & SIGN);
		assert(spec.type == FORMAT_TYPE_INT);
		assert(spec.field_width == 2);
		assert(!(spec.flags & LEFT));
		assert(spec.flags & ZEROPAD);
		assert(read == 4);
		fmt += 4;

		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 3);
		fmt += 3;

		// %i
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(spec.flags & SIGN);
		assert(spec.type == FORMAT_TYPE_INT);
		assert(spec.field_width == -1);
		assert(!(spec.flags & LEFT));
		assert(!(spec.flags & ZEROPAD));
		assert(read == 2);
		fmt += 2;

		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 1);
		fmt += 1;

		// %1234i
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(spec.flags & SIGN);
		assert(spec.type == FORMAT_TYPE_INT);
		assert(spec.field_width == 1234);
		assert(!(spec.flags & LEFT));
		assert(!(spec.flags & ZEROPAD));
		assert(read == 6);
		fmt += 6;

		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 6);
		fmt += 6;

		// %014i
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(spec.flags & SIGN);
		assert(spec.type == FORMAT_TYPE_INT);
		assert(spec.field_width == 14);
		assert(!(spec.flags & LEFT));
		assert(spec.flags & ZEROPAD);
		assert(read == 5);
		fmt += 5;

		// %02li
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(spec.flags & SIGN);
		assert(spec.type == FORMAT_TYPE_LONG);
		assert(spec.field_width == 2);
		assert(!(spec.flags & LEFT));
		assert(spec.flags & ZEROPAD);
		assert(read == 5);
		fmt += 5;

		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 4);
		fmt += 4;

		// %14180lli
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(spec.flags & SIGN);
		assert(spec.type == FORMAT_TYPE_LONG_LONG);
		assert(spec.field_width == 14180);
		assert(!(spec.flags & LEFT));
		assert(!(spec.flags & ZEROPAD));
		assert(read == 9);
		fmt += 9;

		// %02hi
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(spec.flags & SIGN);
		assert(spec.type == FORMAT_TYPE_SHORT);
		assert(spec.field_width == 2);
		assert(!(spec.flags & LEFT));
		assert(spec.flags & ZEROPAD);
		assert(read == 5);
		fmt += 5;

		// %14180hhi
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(spec.flags & SIGN);
		assert(spec.type == FORMAT_TYPE_BYTE);
		assert(spec.field_width == 14180);
		assert(!(spec.flags & LEFT));
		assert(!(spec.flags & ZEROPAD));
		assert(read == 9);
		fmt += 9;
	}

	{
		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 4);
		fmt += 4;

		// %02u
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(!(spec.flags & SIGN));
		assert(spec.type == FORMAT_TYPE_UINT);
		assert(spec.field_width == 2);
		assert(!(spec.flags & LEFT));
		assert(spec.flags & ZEROPAD);
		assert(read == 4);
		fmt += 4;

		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 3);
		fmt += 3;

		// %u
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(!(spec.flags & SIGN));
		assert(spec.type == FORMAT_TYPE_UINT);
		assert(spec.field_width == -1);
		assert(!(spec.flags & LEFT));
		assert(!(spec.flags & ZEROPAD));
		assert(read == 2);
		fmt += 2;

		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 1);
		fmt += 1;

		// %1234u
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(!(spec.flags & SIGN));
		assert(spec.type == FORMAT_TYPE_UINT);
		assert(spec.field_width == 1234);
		assert(!(spec.flags & LEFT));
		assert(!(spec.flags & ZEROPAD));
		assert(read == 6);
		fmt += 6;

		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 6);
		fmt += 6;

		// %014u
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(!(spec.flags & SIGN));
		assert(spec.type == FORMAT_TYPE_UINT);
		assert(spec.field_width == 14);
		assert(!(spec.flags & LEFT));
		assert(spec.flags & ZEROPAD);
		assert(read == 5);
		fmt += 5;

		// %02lu
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(!(spec.flags & SIGN));
		assert(spec.type == FORMAT_TYPE_ULONG);
		assert(spec.field_width == 2);
		assert(!(spec.flags & LEFT));
		assert(spec.flags & ZEROPAD);
		assert(read == 5);
		fmt += 5;

		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 4);
		fmt += 4;

		// %14180llu
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(!(spec.flags & SIGN));
		assert(spec.type == FORMAT_TYPE_LONG_LONG);
		assert(spec.field_width == 14180);
		assert(!(spec.flags & LEFT));
		assert(!(spec.flags & ZEROPAD));
		assert(read == 9);
		fmt += 9;

		// %02hu
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(!(spec.flags & SIGN));
		assert(spec.type == FORMAT_TYPE_USHORT);
		assert(spec.field_width == 2);
		assert(!(spec.flags & LEFT));
		assert(spec.flags & ZEROPAD);
		assert(read == 5);
		fmt += 5;

		// %14180hhu
		read = format_decode(fmt, &spec);
		assert(spec.base == 10);
		assert(!(spec.flags & SIGN));
		assert(spec.type == FORMAT_TYPE_UBYTE);
		assert(spec.field_width == 14180);
		assert(!(spec.flags & LEFT));
		assert(!(spec.flags & ZEROPAD));
		assert(read == 9);
		fmt += 9;
	}

	{
		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 4);
		fmt += 4;

		// %02o
		read = format_decode(fmt, &spec);
		assert(spec.base == 8);
		assert(!(spec.flags & SIGN));
		assert(!(spec.flags & SMALL));
		assert(spec.type == FORMAT_TYPE_UINT);
		assert(spec.field_width == 2);
		assert(!(spec.flags & LEFT));
		assert(spec.flags & ZEROPAD);
		assert(read == 4);
		fmt += 4;

		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 3);
		fmt += 3;

		// %o
		read = format_decode(fmt, &spec);
		assert(spec.base == 8);
		assert(!(spec.flags & SIGN));
		assert(!(spec.flags & SMALL));
		assert(spec.type == FORMAT_TYPE_UINT);
		assert(spec.field_width == -1);
		assert(!(spec.flags & LEFT));
		assert(!(spec.flags & ZEROPAD));
		assert(read == 2);
		fmt += 2;

		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 1);
		fmt += 1;

		// %1234o
		read = format_decode(fmt, &spec);
		assert(spec.base == 8);
		assert(!(spec.flags & SIGN));
		assert(!(spec.flags & SMALL));
		assert(spec.type == FORMAT_TYPE_UINT);
		assert(spec.field_width == 1234);
		assert(!(spec.flags & LEFT));
		assert(!(spec.flags & ZEROPAD));
		assert(read == 6);
		fmt += 6;

		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 6);
		fmt += 6;

		// %014o
		read = format_decode(fmt, &spec);
		assert(spec.base == 8);
		assert(!(spec.flags & SIGN));
		assert(!(spec.flags & SMALL));
		assert(spec.type == FORMAT_TYPE_UINT);
		assert(spec.field_width == 14);
		assert(!(spec.flags & LEFT));
		assert(spec.flags & ZEROPAD);
		assert(read == 5);
		fmt += 5;

		// %02lo
		read = format_decode(fmt, &spec);
		assert(spec.base == 8);
		assert(!(spec.flags & SIGN));
		assert(!(spec.flags & SMALL));
		assert(spec.type == FORMAT_TYPE_ULONG);
		assert(spec.field_width == 2);
		assert(!(spec.flags & LEFT));
		assert(spec.flags & ZEROPAD);
		assert(read == 5);
		fmt += 5;

		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 4);
		fmt += 4;

		// %14180llo
		read = format_decode(fmt, &spec);
		assert(spec.base == 8);
		assert(!(spec.flags & SIGN));
		assert(!(spec.flags & SMALL));
		assert(spec.type == FORMAT_TYPE_LONG_LONG);
		assert(spec.field_width == 14180);
		assert(!(spec.flags & LEFT));
		assert(!(spec.flags & ZEROPAD));
		assert(read == 9);
		fmt += 9;

		// %02ho
		read = format_decode(fmt, &spec);
		assert(spec.base == 8);
		assert(!(spec.flags & SIGN));
		assert(!(spec.flags & SMALL));
		assert(spec.type == FORMAT_TYPE_USHORT);
		assert(spec.field_width == 2);
		assert(!(spec.flags & LEFT));
		assert(spec.flags & ZEROPAD);
		assert(read == 5);
		fmt += 5;

		// %14180hho
		read = format_decode(fmt, &spec);
		assert(spec.base == 8);
		assert(!(spec.flags & SIGN));
		assert(!(spec.flags & SMALL));
		assert(spec.type == FORMAT_TYPE_UBYTE);
		assert(spec.field_width == 14180);
		assert(!(spec.flags & LEFT));
		assert(!(spec.flags & ZEROPAD));
		assert(read == 9);
		fmt += 9;
	}

	{
		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 4);
		fmt += 4;

		// %02X
		read = format_decode(fmt, &spec);
		assert(spec.base == 16);
		assert(!(spec.flags & SIGN));
		assert(!(spec.flags & SMALL));
		assert(spec.type == FORMAT_TYPE_UINT);
		assert(spec.field_width == 2);
		assert(!(spec.flags & LEFT));
		assert(spec.flags & ZEROPAD);
		assert(read == 4);
		fmt += 4;

		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 3);
		fmt += 3;

		// %X
		read = format_decode(fmt, &spec);
		assert(spec.base == 16);
		assert(!(spec.flags & SIGN));
		assert(!(spec.flags & SMALL));
		assert(spec.type == FORMAT_TYPE_UINT);
		assert(spec.field_width == -1);
		assert(!(spec.flags & LEFT));
		assert(!(spec.flags & ZEROPAD));
		assert(read == 2);
		fmt += 2;

		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 1);
		fmt += 1;

		// %1234X
		read = format_decode(fmt, &spec);
		assert(spec.base == 16);
		assert(!(spec.flags & SIGN));
		assert(!(spec.flags & SMALL));
		assert(spec.type == FORMAT_TYPE_UINT);
		assert(spec.field_width == 1234);
		assert(!(spec.flags & LEFT));
		assert(!(spec.flags & ZEROPAD));
		assert(read == 6);
		fmt += 6;

		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 6);
		fmt += 6;

		// %014X
		read = format_decode(fmt, &spec);
		assert(spec.base == 16);
		assert(!(spec.flags & SIGN));
		assert(!(spec.flags & SMALL));
		assert(spec.type == FORMAT_TYPE_UINT);
		assert(spec.field_width == 14);
		assert(!(spec.flags & LEFT));
		assert(spec.flags & ZEROPAD);
		assert(read == 5);
		fmt += 5;

		// %02lX
		read = format_decode(fmt, &spec);
		assert(spec.base == 16);
		assert(!(spec.flags & SIGN));
		assert(!(spec.flags & SMALL));
		assert(spec.type == FORMAT_TYPE_ULONG);
		assert(spec.field_width == 2);
		assert(!(spec.flags & LEFT));
		assert(spec.flags & ZEROPAD);
		assert(read == 5);
		fmt += 5;

		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 4);
		fmt += 4;

		// %14180llX
		read = format_decode(fmt, &spec);
		assert(spec.base == 16);
		assert(!(spec.flags & SIGN));
		assert(!(spec.flags & SMALL));
		assert(spec.type == FORMAT_TYPE_LONG_LONG);
		assert(spec.field_width == 14180);
		assert(!(spec.flags & LEFT));
		assert(!(spec.flags & ZEROPAD));
		assert(read == 9);
		fmt += 9;

		// %02hX
		read = format_decode(fmt, &spec);
		assert(spec.base == 16);
		assert(!(spec.flags & SIGN));
		assert(!(spec.flags & SMALL));
		assert(spec.type == FORMAT_TYPE_USHORT);
		assert(spec.field_width == 2);
		assert(!(spec.flags & LEFT));
		assert(spec.flags & ZEROPAD);
		assert(read == 5);
		fmt += 5;

		// %14180hhX
		read = format_decode(fmt, &spec);
		assert(spec.base == 16);
		assert(!(spec.flags & SIGN));
		assert(!(spec.flags & SMALL));
		assert(spec.type == FORMAT_TYPE_UBYTE);
		assert(spec.field_width == 14180);
		assert(!(spec.flags & LEFT));
		assert(!(spec.flags & ZEROPAD));
		assert(read == 9);
		fmt += 9;
	}

	{
		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 4);
		fmt += 4;

		// %02x
		read = format_decode(fmt, &spec);
		assert(spec.base == 16);
		assert(!(spec.flags & SIGN));
		assert(spec.flags & SMALL);
		assert(spec.type == FORMAT_TYPE_UINT);
		assert(spec.field_width == 2);
		assert(!(spec.flags & LEFT));
		assert(spec.flags & ZEROPAD);
		assert(read == 4);
		fmt += 4;

		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 3);
		fmt += 3;

		// %x
		read = format_decode(fmt, &spec);
		assert(spec.base == 16);
		assert(!(spec.flags & SIGN));
		assert(spec.flags & SMALL);
		assert(spec.type == FORMAT_TYPE_UINT);
		assert(spec.field_width == -1);
		assert(!(spec.flags & LEFT));
		assert(!(spec.flags & ZEROPAD));
		assert(read == 2);
		fmt += 2;

		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 1);
		fmt += 1;

		// %1234x
		read = format_decode(fmt, &spec);
		assert(spec.base == 16);
		assert(!(spec.flags & SIGN));
		assert(spec.flags & SMALL);
		assert(spec.type == FORMAT_TYPE_UINT);
		assert(spec.field_width == 1234);
		assert(!(spec.flags & LEFT));
		assert(!(spec.flags & ZEROPAD));
		assert(read == 6);
		fmt += 6;

		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 6);
		fmt += 6;

		// %014x
		read = format_decode(fmt, &spec);
		assert(spec.base == 16);
		assert(!(spec.flags & SIGN));
		assert(spec.flags & SMALL);
		assert(spec.type == FORMAT_TYPE_UINT);
		assert(spec.field_width == 14);
		assert(!(spec.flags & LEFT));
		assert(spec.flags & ZEROPAD);
		assert(read == 5);
		fmt += 5;

		// %02lx
		read = format_decode(fmt, &spec);
		assert(spec.base == 16);
		assert(!(spec.flags & SIGN));
		assert(spec.flags & SMALL);
		assert(spec.type == FORMAT_TYPE_ULONG);
		assert(spec.field_width == 2);
		assert(!(spec.flags & LEFT));
		assert(spec.flags & ZEROPAD);
		assert(read == 5);
		fmt += 5;

		read = format_decode(fmt, &spec);
		assert(spec.type == FORMAT_TYPE_NONE);
		assert(read == 4);
		fmt += 4;

		// %14180llx
		read = format_decode(fmt, &spec);
		assert(spec.base == 16);
		assert(!(spec.flags & SIGN));
		assert(spec.flags & SMALL);
		assert(spec.type == FORMAT_TYPE_LONG_LONG);
		assert(spec.field_width == 14180);
		assert(!(spec.flags & LEFT));
		assert(!(spec.flags & ZEROPAD));
		assert(read == 9);
		fmt += 9;

		// %02hx
		read = format_decode(fmt, &spec);
		assert(spec.base == 16);
		assert(!(spec.flags & SIGN));
		assert(spec.flags & SMALL);
		assert(spec.type == FORMAT_TYPE_USHORT);
		assert(spec.field_width == 2);
		assert(!(spec.flags & LEFT));
		assert(spec.flags & ZEROPAD);
		assert(read == 5);
		fmt += 5;

		// %14180hhx
		read = format_decode(fmt, &spec);
		assert(spec.base == 16);
		assert(!(spec.flags & SIGN));
		assert(spec.flags & SMALL);
		assert(spec.type == FORMAT_TYPE_UBYTE);
		assert(spec.field_width == 14180);
		assert(!(spec.flags & LEFT));
		assert(!(spec.flags & ZEROPAD));
		assert(read == 9);
		fmt += 9;
	}

	// %c
	read = format_decode(fmt, &spec);
	assert(spec.type == FORMAT_TYPE_CHAR);
	assert(spec.field_width == -1);
	assert(!(spec.flags & LEFT));
	assert(read == 2);
	fmt += 2;

	// %14180c
	read = format_decode(fmt, &spec);
	assert(spec.type == FORMAT_TYPE_CHAR);
	assert(spec.field_width == 14180);
	assert(!(spec.flags & LEFT));
	assert(read == 7);
	fmt += 7;

	// %-14180c
	read = format_decode(fmt, &spec);
	assert(spec.type == FORMAT_TYPE_CHAR);
	assert(spec.field_width == 14180);
	assert(spec.flags & LEFT);
	assert(read == 8);
	fmt += 8;

	read = format_decode(fmt, &spec);
	assert(read == 0);
}

void test_snprintf() {
	test_format_decode();
	test_hzw_snprintf();
}
