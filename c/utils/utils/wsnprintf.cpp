#include <assert.h> // for assert
#include <stdarg.h> // for va_list, va_start, va_end
#include <string.h> // for memcpy, memset, memmove, strcmp
#include <wchar.h>

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
	FORMAT_TYPE_WSTR,
	FORMAT_TYPE_PERCENT_CHAR,
	FORMAT_TYPE_INVALID,
	FORMAT_TYPE_LONG_LONG,
	FORMAT_TYPE_ULONG,
	FORMAT_TYPE_LONG,
	FORMAT_TYPE_UBYTE,
	FORMAT_TYPE_BYTE,
	FORMAT_TYPE_USHORT,
	FORMAT_TYPE_SHORT,
	FORMAT_TYPE_UINT,
	FORMAT_TYPE_INT,
};

struct printf_spec {
	unsigned int type;
	signed int field_width;
	unsigned int flags;
	unsigned int base;
	signed int precision;
};

static char hex_asc_upper[] = "0123456789ABCDEF";

static int isdigit(int c) {
	return c >= '0' && c <= '9';
}

static int skip_atoi(const char **s) {
	int i  = 0;

	do {
		i = i * 10 + *((*s)++) - '0';
	} while(isdigit(**s));

	return i;
}

static int format_decode(const char *fmt, struct printf_spec *spec) {
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

		case 'w':
			spec->type = FORMAT_TYPE_WSTR;
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

		default:
			spec->type = FORMAT_TYPE_INVALID;
			return fmt - start;
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

static void bufset(wchar_t *buf, wchar_t wc, int size) {
	int i;
	
	for (i = 0; i < size; i++) {
		buf[i] = wc;
	}
}

static void bufmove(wchar_t *buf1, wchar_t *buf2, int size) {
	int i;
	
	for (i = size - 1; i >= 0; i++) {
		buf1[i] = buf2[i];
	}
}

static void move_right(wchar_t *buf, wchar_t *end, unsigned len, unsigned spaces) {
	size_t size;

	if (buf >= end) {
		return;
	}

	size = end - buf;

	if (size <= spaces) {
		bufset(buf, ' ', size);
		return;
	}

	if (len) {
		if (len > size - spaces) {
			len = size - spaces;
		}
		bufmove(buf + spaces, buf, len);
	}
	
	bufset(buf, ' ', spaces);
}

static wchar_t* widen_string(wchar_t *buf, int n, wchar_t *end, struct printf_spec spec) {
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

static wchar_t* string_nocheck(wchar_t *buf, wchar_t *end, const char *s, struct printf_spec spec) {
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

static wchar_t* wstring_nocheck(wchar_t *buf, wchar_t *end, const wchar_t *s, struct printf_spec spec) {
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

static wchar_t* error_string(wchar_t *buf, wchar_t *end, const char *s, struct printf_spec spec) {
	return string_nocheck(buf, end, s, spec);
}

static const char* check_pointer_msg(const void *ptr) {
	if (!ptr) {
		return "(null)";
	}

	return NULL;
}

#define EFAULT 14 /*Bad address*/

static int check_pointer(wchar_t **buf, wchar_t *end, const void *ptr, struct printf_spec spec) {
	const char *err_msg;

	err_msg = check_pointer_msg(ptr);
	if (err_msg) {
		*buf = error_string(*buf, end, err_msg, spec);
		return -EFAULT;
	}

	return 0;
}

static wchar_t* string(wchar_t *buf, wchar_t *end, const char *s, struct printf_spec spec) {
	if (check_pointer(&buf, end, s, spec)) {
		return buf;
	}

	return string_nocheck(buf, end, s, spec);
}

static wchar_t* wstring(wchar_t *buf, wchar_t *end, const wchar_t *s, struct printf_spec spec) {
	if (check_pointer(&buf, end, s, spec)) {
		return buf;
	}

	return wstring_nocheck(buf, end, s, spec);
}

static char* put_dec(char *buf, unsigned long long n) {
	do {
		*buf++ = (n % 10) + '0';
		n /= 10;
	} while(n);
	
	return buf;
}

static wchar_t* number(wchar_t *buf, wchar_t *end, unsigned long long num, struct printf_spec spec) {
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

static void bufcopy(wchar_t *ws, const char *s, int len) {
	int i;
	for (i = 0; i < len; i++) {
		ws[i] = s[i];
	}
}

int vwsnprintf(wchar_t *buf, size_t size, const char *fmt, va_list args) {
	unsigned long long num;
	wchar_t *str, *end;
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
					bufcopy(str, old_fmt, copy);
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

			case FORMAT_TYPE_WSTR:
				str = wstring(str, end, va_arg(args, wchar_t*), spec);
				break;

			case FORMAT_TYPE_PERCENT_CHAR:
				if (str < end) {
					*str = '%';
				}
				++str;
				break;

			case FORMAT_TYPE_INVALID:
				goto out;

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

out:
	if (str < end) {
		*str = '\0';
	} else {
		end[-1] = '\0';
	}

	return str - buf;
}

int wsnprintf(wchar_t *buf, size_t size, const char *fmt, ...) {
	va_list args;
	int i;

	va_start(args, fmt);
	i = vwsnprintf(buf, size, fmt, args);
	va_end(args);

	return i;
}

int wsprintf(wchar_t *buf, const char *fmt, ...) {
	va_list args;
	int i;

	va_start(args, fmt);
	i = vwsnprintf(buf, 0x7fffffff >> 1, fmt, args);
	va_end(args);

	return i;
}

void test_wsnprintf() {
	wchar_t out[512];
	wchar_t *expected;
	int len;

	expected = L"C:\\hzw";
	len = wsnprintf(out, 512, "%c:\\%s", 'C', "hzw");
	assert(wcscmp(expected, out) == 0);
	assert(len == 6);

	expected = L"C:\\hzw\\gomaster";
	len = wsnprintf(out, 512, "%c:\\%s\\%s", 'C', "hzw", "gomaster");
	assert(wcscmp(expected, out) == 0);
	assert(len == 15);

	expected = L"C:\\hzw\\gomaster\\gitmaster";
	len = wsnprintf(out, 512, "%c:\\%s\\%s\\%s", 'C', "hzw", "gomaster", "gitmaster");
	assert(wcscmp(expected, out) == 0);
	assert(len == 25);

	expected = L"C:\\hzw\\gomaster\\gitmaster\\log.tmp";
	len = wsnprintf(out, 512, "%c:\\%s\\%s\\%s\\%s.%s", 'C', "hzw", "gomaster", "gitmaster", "log", "tmp");
	assert(wcscmp(expected, out) == 0);
	assert(len == 33);

	expected = L"hello world";
	len = wsnprintf(out, 512, "%w", L"hello world");
	assert(wcscmp(expected, out) == 0);
	assert(len == 11);

	expected = L"hello world 14180";
	len = wsnprintf(out, 512, "%w %d", L"hello world", 14180);
	assert(wcscmp(expected, out) == 0);
	assert(len == 17);

	expected = L"14180 hello world";
	len = wsnprintf(out, 512, "%d %w", 14180, L"hello world");
	assert(wcscmp(expected, out) == 0);
	assert(len == 17);

	expected = L"hello world [14180]";
	len = wsnprintf(out, 512, "%w [%d]", L"hello world", 14180);
	assert(wcscmp(expected, out) == 0);
	assert(len == 19);

	expected = L"hello world 01:02";
	len = wsnprintf(out, 512, "%w %02d:%02d", L"hello world", 1, 2);
	assert(wcscmp(expected, out) == 0);
	assert(len == 17);

	expected = L"hi (null)!";
	len = wsnprintf(out, 512, "hi %s!", NULL);
	assert(wcscmp(expected, out) == 0);
	assert(len == 10);

	expected = L"hi (null)!";
	len = wsnprintf(out, 512, "hi %w!", NULL);
	assert(wcscmp(expected, out) == 0);
	assert(len == 10);
}

static int wskip_atoi(const wchar_t **s) {
	int i  = 0;

	do {
		i = i * 10 + *((*s)++) - '0';
	} while(isdigit(**s));

	return i;
}

static int wformat_decode(const wchar_t *fmt, struct printf_spec *spec) {
	const wchar_t *start = fmt;
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
		spec->field_width = wskip_atoi(&fmt);
	}

	spec->precision = -1;
	if (*fmt == '.') {
		++fmt;
		if (isdigit(*fmt)) {
			spec->precision = wskip_atoi(&fmt);
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

		case 'w':
			spec->type = FORMAT_TYPE_WSTR;
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

		default:
			spec->type = FORMAT_TYPE_INVALID;
			return fmt - start;
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

static void wbufcopy(wchar_t *ws1, const wchar_t *ws2, int len) {
	int i;
	for (i = 0; i < len; i++) {
		ws1[i] = ws2[i];
	}
}

int vwsnprintwf(wchar_t *buf, size_t size, const wchar_t *fmt, va_list args) {
	unsigned long long num;
	wchar_t *str, *end;
	struct printf_spec spec = {0};

	str = buf;
	end = buf + size;

	while (*fmt) {
		const wchar_t *old_fmt = fmt;
		int read = wformat_decode(fmt, &spec);

		fmt += read;

		switch(spec.type) {
			case FORMAT_TYPE_NONE: {
				int copy = read;
				
				if (str < end) {
					if (copy > end - str) {
						copy = end - str;
					}
					wbufcopy(str, old_fmt, copy);
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

			case FORMAT_TYPE_WSTR:
				str = wstring(str, end, va_arg(args, wchar_t*), spec);
				break;

			case FORMAT_TYPE_PERCENT_CHAR:
				if (str < end) {
					*str = '%';
				}
				++str;
				break;

			case FORMAT_TYPE_INVALID:
				goto out;

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

out:
	if (str < end) {
		*str = '\0';
	} else {
		end[-1] = '\0';
	}

	return str - buf;
}

int wsnprintwf(wchar_t *buf, size_t size, const wchar_t *fmt, ...) {
	va_list args;
	int i;

	va_start(args, fmt);
	i = vwsnprintwf(buf, size, fmt, args);
	va_end(args);

	return i;
}

int wsprintwf(wchar_t *buf, const wchar_t *fmt, ...) {
	va_list args;
	int i;

	va_start(args, fmt);
	i = vwsnprintwf(buf, 0x7fffffff >> 1, fmt, args);
	va_end(args);

	return i;
}

void test_wsnprintwf() {
	wchar_t out[512];
	wchar_t *expected;
	int len;

	expected = L"C:\\hzw";
	len = wsnprintwf(out, 512, L"%c:\\%s", 'C', "hzw");
	assert(wcscmp(expected, out) == 0);
	assert(len == 6);

	expected = L"C:\\hzw\\gomaster";
	len = wsnprintwf(out, 512, L"%c:\\%s\\%s", 'C', "hzw", "gomaster");
	assert(wcscmp(expected, out) == 0);
	assert(len == 15);

	expected = L"C:\\hzw\\gomaster\\gitmaster";
	len = wsnprintwf(out, 512, L"%c:\\%s\\%s\\%s", 'C', "hzw", "gomaster", "gitmaster");
	assert(wcscmp(expected, out) == 0);
	assert(len == 25);

	expected = L"C:\\hzw\\gomaster\\gitmaster\\log.tmp";
	len = wsnprintwf(out, 512, L"%c:\\%s\\%s\\%s\\%s.%s", 'C', "hzw", "gomaster", "gitmaster", "log", "tmp");
	assert(wcscmp(expected, out) == 0);
	assert(len == 33);

	expected = L"hello world";
	len = wsnprintwf(out, 512, L"%w", L"hello world");
	assert(wcscmp(expected, out) == 0);
	assert(len == 11);

	expected = L"hello world 14180";
	len = wsnprintwf(out, 512, L"%w %d", L"hello world", 14180);
	assert(wcscmp(expected, out) == 0);
	assert(len == 17);

	expected = L"14180 hello world";
	len = wsnprintwf(out, 512, L"%d %w", 14180, L"hello world");
	assert(wcscmp(expected, out) == 0);
	assert(len == 17);

	expected = L"hello world [14180]";
	len = wsnprintwf(out, 512, L"%w [%d]", L"hello world", 14180);
	assert(wcscmp(expected, out) == 0);
	assert(len == 19);

	expected = L"hello world 01:02";
	len = wsnprintwf(out, 512, L"%w %02d:%02d", L"hello world", 1, 2);
	assert(wcscmp(expected, out) == 0);
	assert(len == 17);
}
