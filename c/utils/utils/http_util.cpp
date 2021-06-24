#include <assert.h>
#include <string.h> // strcmp
#include <stdio.h> // sprintf

#include "memory.h"
#include "buffer1.h"

#define HEADER_MAX_LEN 512

bool add_header(char *buf, int len, char *name, char *value) {
	int buf_len = 0, name_len = 0, value_len = 0, header_len = 0, old_header_len = 0, offset = 0;
	char header[HEADER_MAX_LEN] = {0};
	char *p1 = NULL, *p2 = NULL;

	buf_len = strlen(buf);
	name_len = strlen(name);
	value_len = strlen(value);
	header_len = name_len + 2 + value_len + 2;

	if (header_len >= HEADER_MAX_LEN) return false;

	sprintf(header, "%s: ", name);
	p1 = strstr(buf, header);
	sprintf(header, "%s: %s\r\n", name, value);

	if (!p1) {
		if (buf_len + header_len >= len) return false;
		strcat(buf, header);
		return true;
	}

	p2 = strstr(p1, "\r\n");
	if (!p2) return false;
	p2 += 2;
	old_header_len = p2 - p1;
	if (!value_len) header_len = 0;
	offset = header_len - old_header_len;

	if (buf_len + offset >= len) return false;

	memcpy(p2 + offset, p2, buf + buf_len - p2 + 1);
	memcpy(p1, header, header_len);

	return true;
}

char lower_case(char c) {
	if (c >= 'A' && c <= 'Z') {
		return c + 0x20;
	}
	return c;
}

bool get_header(char *buf, char *name, char *value, int len) {
	char *s = NULL, *e = NULL, *pn = NULL;

	s = e = buf;
	while (*e) {
		if (*e == '\r' && *(e+1) == '\n') {
			pn = name;
			while (s < e && *s != ':' && *pn) {
				if (lower_case(*s) != lower_case(*pn)) {
					break;
				}
				s++;
				pn++;
			}

			if (*s == ':' && !*pn) {
				while (*(++s) == ' ');
				if (e - s >= len) return false;
				while (s < e) {
					*value++ = *s++;
				}
				*value = 0;
				return true;
			}

			e += 2;
			s = e;
			continue;
		}

		e++;
	}

	return false;
}

bool get_line_size(char *buf, int *size) {
	char *p = NULL;

	p = strstr(buf, "\r\n");
	if (!p) return false;
	p += 2;
	*size = p - buf;
	return true;
}

int delete_one_line(char *buf) {
	char *s = NULL, *e = NULL;

	s = strstr(buf, "\r\n");
	if (!s) return 0;
	s += 2;
	e = buf + strlen(buf);
	memcpy(buf, s, e - s + 1);
	return s - buf;
}

#define HTTP_RESPONSE_MAX_LINE_LEN (2 * 1024)
#define HTTP_RESPONSE_MAX_HEADERS_LEN (8 * 1024)

typedef enum http_parse_step_e {
	step_status_line,
	step_headers,
	step_fixed_body,
} http_parse_step_t;

typedef enum http_parse_result_e {
	result_error,
	result_block,
} http_parse_result_t;

typedef struct http_parser_s {
	http_parse_step_t step;
	buffer_t *buf;
} http_parser_t;

typedef struct http_session_s {
	int res_code;
	buffer_t *res_headers;
	http_parser_t *parser;
} http_session_t;

int (*tcp_read)(int sock, char *buf, int len);

http_parse_result_t http_read(http_session_t *session) {
    int result = 0;
    int line_size = 0;
    char *s = NULL, *e = NULL, *r = NULL;
    http_parser_t *parser = NULL;
    int len = 0;

    parser = session->parser;
    switch (parser->step) {
        case step_status_line:
step_status_line:
        if (buffer_full(parser->buf)) return result_error;
        result = tcp_read(0, buffer_writable(parser->buf), buffer_writable_len(parser->buf));
        if (result == 0) {
            return result_block;
        } else if (result < 0) {
            return result_error;
        } else {
            buffer_inc_len(parser->buf, result);

            r = buffer_readable(parser->buf);
            if (!get_line_size(r, &line_size)) {
                goto step_status_line;
            }

            s = strstr(r, " ");
            if (!s || s > r + line_size) return result_error;
            s += 1;
            e = strstr(s, " ");
            if (!e || e > r + line_size) return result_error;
            if (e - s != 3) return result_error;
            while (s < e) {
                if (*s < '0' || *s > '9') return result_error;
                session->res_code *= 10;
                session->res_code += *s++ - '0';
            }
            buffer_clear(parser->buf, line_size);
            parser->step = step_headers;
            // fall through
        }

    case step_headers:
step_headers:
        r = buffer_readable(parser->buf);
        while (get_line_size(r, &line_size)) {
            if (line_size == 2) {
                buffer_clear(parser->buf, line_size);
                parser->step = step_fixed_body;
                goto step_fixed_body;
            }

            if (line_size > buffer_writable_len(session->res_headers)) return result_error;
            buffer_copy(session->res_headers, parser->buf, line_size);
            buffer_clear(parser->buf, line_size);
        }

    if (buffer_full(parser->buf)) return result_error;
    result = tcp_read(0, buffer_writable(parser->buf), buffer_writable_len(parser->buf));
    if (result == 0) {
        return result_block;
    } else if (result < 0) {
        return result_error;
    } else {
        buffer_inc_len(parser->buf, result);
        goto step_headers;
    }

    case step_fixed_body:
step_fixed_body:
        break;
    }
}

void test_add_header() {
	char *buf = NULL;
	int len = 512;

	buf = (char*)HZW_MALLOC(len);
	memset(buf, 0x48, len);
	buf[0] = 0;

	strcat(buf, "GET / HTTP/1.1\r\n");

	add_header(buf, len, "Host", "www.baidu.com");
	assert(!strcmp(buf, "GET / HTTP/1.1\r\n" "Host: www.baidu.com\r\n"));
	add_header(buf, len, "User-Agent", "HZW bot");
	assert(!strcmp(buf, "GET / HTTP/1.1\r\n" "Host: www.baidu.com\r\n" "User-Agent: HZW bot\r\n"));
	add_header(buf, len, "Host", "www.baidu.com:80");
	assert(!strcmp(buf, "GET / HTTP/1.1\r\n" "Host: www.baidu.com:80\r\n" "User-Agent: HZW bot\r\n"));
	add_header(buf, len, "Host", "www.baidu.com");
	assert(!strcmp(buf, "GET / HTTP/1.1\r\n" "Host: www.baidu.com\r\n" "User-Agent: HZW bot\r\n"));
	
	add_header(buf, len, "Accept", "text/html");
	add_header(buf, len, "Accept-Encoding", "gzip, deflate, br");
	add_header(buf, len, "Accept-Language", "zh-CN,zh;q=0.9");
	add_header(buf, len, "Cache-Control", "max-age=0");
	assert(!strcmp(buf, "GET / HTTP/1.1\r\n"
		                             "Host: www.baidu.com\r\n"
								     "User-Agent: HZW bot\r\n"
									 "Accept: text/html\r\n"
									 "Accept-Encoding: gzip, deflate, br\r\n"
									 "Accept-Language: zh-CN,zh;q=0.9\r\n"
									 "Cache-Control: max-age=0\r\n"));

	add_header(buf, len, "Accept", "text/html,application/xhtml+xml");
	add_header(buf, len, "Accept-Encoding", "gzip, deflate");
	add_header(buf, len, "Accept-Language", "zh-CN");
	add_header(buf, len, "Cache-Control", "");
	assert(!strcmp(buf, "GET / HTTP/1.1\r\n"
		                             "Host: www.baidu.com\r\n"
								     "User-Agent: HZW bot\r\n"
									 "Accept: text/html,application/xhtml+xml\r\n"
									 "Accept-Encoding: gzip, deflate\r\n"
									 "Accept-Language: zh-CN\r\n"));

	add_header(buf, len, "User-Agent", "");
	assert(!strcmp(buf, "GET / HTTP/1.1\r\n"
		                             "Host: www.baidu.com\r\n"
									 "Accept: text/html,application/xhtml+xml\r\n"
									 "Accept-Encoding: gzip, deflate\r\n"
									 "Accept-Language: zh-CN\r\n"));

	add_header(buf, len, "Host", "");
	assert(!strcmp(buf, "GET / HTTP/1.1\r\n"
		                             "Accept: text/html,application/xhtml+xml\r\n"
									 "Accept-Encoding: gzip, deflate\r\n"
									 "Accept-Language: zh-CN\r\n"));

	add_header(buf, len, "Accept-Language", "");
	assert(!strcmp(buf, "GET / HTTP/1.1\r\n"
		                             "Accept: text/html,application/xhtml+xml\r\n"
									 "Accept-Encoding: gzip, deflate\r\n"));

	HzwFree(buf);
}

void test_get_header() {
	char *buf = NULL;
	int len = 512;
	char value10[10] = {0};
	char value100[100] = {0};

	buf = (char*)HZW_MALLOC(len);
	memset(buf, 0x48, len);
	buf[0] = 0;

	strcat(buf, "GET / HTTP/1.1\r\n");

	add_header(buf, len, "Host", "www.baidu.com");
	assert(!strcmp(buf, "GET / HTTP/1.1\r\n" "Host: www.baidu.com\r\n"));
	add_header(buf, len, "User-Agent", "HZW bot");
	assert(!strcmp(buf, "GET / HTTP/1.1\r\n" "Host: www.baidu.com\r\n" "User-Agent: HZW bot\r\n"));
	add_header(buf, len, "Accept", "text/html");
	add_header(buf, len, "Accept-Encoding", "gzip, deflate, br");
	add_header(buf, len, "Accept-Language", "zh-CN,zh;q=0.9");
	add_header(buf, len, "Cache-Control", "max-age=0");
	assert(!strcmp(buf, "GET / HTTP/1.1\r\n"
		                             "Host: www.baidu.com\r\n"
								     "User-Agent: HZW bot\r\n"
									 "Accept: text/html\r\n"
									 "Accept-Encoding: gzip, deflate, br\r\n"
									 "Accept-Language: zh-CN,zh;q=0.9\r\n"
									 "Cache-Control: max-age=0\r\n"));

	assert(!get_header(buf, "Host", value10, 10));
	assert(!get_header(buf, "Connection", value100, 100));
	assert(get_header(buf, "Host", value100, 100));
	assert(!strcmp(value100, "www.baidu.com"));
	assert(get_header(buf, "User-Agent", value100, 100));
	assert(!strcmp(value100, "HZW bot"));

	HzwFree(buf);

	buf = "GET / HTTP/1.1\r\n" 
		     "Host:www.baidu.com\r\n"
		     "user-agent:  HZW bot\r\n"
			 "Accept: text/html\r\n"
			 "AccePt-EncodInG:gzip, deflate, br\r\n"
			 "Accept-language:                                                                zh-CN,zh;q=0.9\r\n"
			 "Cache-Control: max-age=0\r\n";
	len = strlen(buf);

	assert(get_header(buf, "host", value100, 100));
	assert(!strcmp(value100, "www.baidu.com"));
	assert(get_header(buf, "User-Agent", value100, 100));
	assert(!strcmp(value100, "HZW bot"));
	assert(get_header(buf, "Accept-Encoding", value100, 100));
	assert(!strcmp(value100, "gzip, deflate, br"));
	assert(get_header(buf, "Accept-Language", value100, 100));
	assert(!strcmp(value100, "zh-CN,zh;q=0.9"));
}

void test_parse_line() {
	char *buf = NULL;
	int len = 512;
	int size = 0;

	buf = (char*)HZW_MALLOC(len);
	memset(buf, 0x48, len);
	buf[0] = 0;

	strcat(buf, "HTTP/1.1 200 OK\r\n");

	add_header(buf, len, "Host", "www.baidu.com");
	assert(!strcmp(buf, "HTTP/1.1 200 OK\r\n" "Host: www.baidu.com\r\n"));
	add_header(buf, len, "User-Agent", "HZW bot");
	assert(!strcmp(buf, "HTTP/1.1 200 OK\r\n" "Host: www.baidu.com\r\n" "User-Agent: HZW bot\r\n"));
	add_header(buf, len, "Accept", "text/html");
	add_header(buf, len, "Accept-Encoding", "gzip, deflate, br");
	add_header(buf, len, "Accept-Language", "zh-CN,zh;q=0.9");
	add_header(buf, len, "Cache-Control", "max-age=0");
	assert(!strcmp(buf, "HTTP/1.1 200 OK\r\n"
		                             "Host: www.baidu.com\r\n"
								     "User-Agent: HZW bot\r\n"
									 "Accept: text/html\r\n"
									 "Accept-Encoding: gzip, deflate, br\r\n"
									 "Accept-Language: zh-CN,zh;q=0.9\r\n"
									 "Cache-Control: max-age=0\r\n"));
	strcat(buf, "\r\n");

	while (get_line_size(buf, &size)) {
		assert(size == delete_one_line(buf));
	}

	HzwFree(buf);
}

int tcp_read1(int sock, char *buf, int len) {
	static char *sbuf = "HTTP/1.1 200 OK\r\n"
							       "Host: www.baidu.com\r\n"
							       "User-Agent: HZW bot\r\n"
							       "Accept: text/html\r\n"
							       "Accept-Encoding: gzip, deflate, br\r\n"
							       "Accept-Language: zh-CN,zh;q=0.9\r\n"
							       "Cache-Control: max-age=0\r\n"
							       "\r\n";
	*buf = *sbuf++;
	return 1;
}

void test_http_read1() {
    http_parser_t parser = {step_status_line, NULL};
    http_session_t session = {0};

    parser.buf = buffer_new(HTTP_RESPONSE_MAX_LINE_LEN);
    session.res_headers = buffer_new(HTTP_RESPONSE_MAX_HEADERS_LEN);
    session.parser = &parser;

    tcp_read = tcp_read1;
    http_read(&session);

	assert(session.res_code == 200);
    assert(!strcmp(buffer_readable(session.res_headers),  "Host: www.baidu.com\r\n"
                                                                                                 "User-Agent: HZW bot\r\n"
                                                                                                 "Accept: text/html\r\n"
                                                                                                 "Accept-Encoding: gzip, deflate, br\r\n"
                                                                                                 "Accept-Language: zh-CN,zh;q=0.9\r\n"
                                                                                                 "Cache-Control: max-age=0\r\n"));
}

void test_http_util() {
	test_add_header();
	test_get_header();
	test_parse_line();
	test_http_read1();
}
