#include <string.h>
#include <assert.h>
#include "split.h"
#include "utils.h"

void split_next_test_1() {
    char *data;
    slice_t s;
    split_t split;

    data = "";
    s = slice_new(data, strlen(data));
    split = split_new(s, '\n');
    
    s = split_next(&split);
    assert(s.len == 0);
    assert(s.data == data);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
    
    s = split_next(&split);
    assert(s.len == 0);
    assert(s.data == 0);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
}

void split_next_test_2() {
    char *data;
    slice_t s;
    split_t split;

    data = "\n";
    s = slice_new(data, strlen(data));
    split = split_new(s, '\n');
   
    s = split_next(&split);
    assert(s.len == 0);
    assert(s.data == data);
    assert(split.s.len == 0);
    assert(split.s.data == data);
    
    s = split_next(&split);
    assert(s.len == 0);
    assert(s.data == data);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
    
    s = split_next(&split);
    assert(s.len == 0);
    assert(s.data == 0);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
}

void split_next_test_3() {
    char *data;
    slice_t s;
    split_t split;

    data = "\n\n";
    s = slice_new(data, strlen(data));
    split = split_new(s, '\n');
    
    s = split_next(&split);
    assert(s.len == 0);
    assert(s.data == data);
    assert(split.s.len == 1);
    assert(split.s.data == data + 1);
    
    s = split_next(&split);
    assert(s.len == 0);
    assert(s.data == data + 1);
    assert(split.s.len == 0);
    assert(split.s.data == data + 1);
    
    s = split_next(&split);
    assert(s.len == 0);
    assert(s.data == data + 1);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
    
    s = split_next(&split);
    assert(s.len == 0);
    assert(s.data == 0);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
}

void split_next_test_4() {
    char *data;
    slice_t s;
    split_t split;

    data = "\n\n\n";
    s = slice_new(data, strlen(data));
    split = split_new(s, '\n');
    
    s = split_next(&split);
    assert(s.len == 0);
    assert(s.data == data);
    assert(split.s.len == 2);
    assert(split.s.data == data + 1);
    
    s = split_next(&split);
    assert(s.len == 0);
    assert(s.data == data + 1);
    assert(split.s.len == 1);
    assert(split.s.data == data + 2);
    
    s = split_next(&split);
    assert(s.len == 0);
    assert(s.data == data + 2);
    assert(split.s.len == 0);
    assert(split.s.data == data + 2);

    s = split_next(&split);
    assert(s.len == 0);
    assert(s.data == data + 2);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
   
    s = split_next(&split);
    assert(s.len == 0);
    assert(s.data == 0);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
}

void split_next_test_5() {
    char *data;
    slice_t s;
    split_t split;

    data = "a";
    s = slice_new(data, strlen(data));
    split = split_new(s, '\n');
    
    s = split_next(&split);
    assert(s.len == 1);
    assert(s.data == data);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
    
    s = split_next(&split);
    assert(s.len == 0);
    assert(s.data == 0);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
}

void split_next_test_6() {
    char *data;
    slice_t s;
    split_t split;

    data = "\na";
    s = slice_new(data, strlen(data));
    split = split_new(s, '\n');
    
    s = split_next(&split);
    assert(s.len == 0);
    assert(s.data == data);
    assert(split.s.len == 1);
    assert(split.s.data == data + 1);
    
    s = split_next(&split);
    assert(s.len == 1);
    assert(s.data == data + 1);
    assert(split.s.len == 0);
    assert(split.s.data == 0);

    s = split_next(&split);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
}

void split_next_test_7() {
    char *data;
    slice_t s;
    split_t split;

    data = "a\n";
    s = slice_new(data, strlen(data));
    split = split_new(s, '\n');
    
    s = split_next(&split);
    assert(s.len == 1);
    assert(s.data == data);
    assert(split.s.len == 0);
    assert(split.s.data == data + 1);
    
    s = split_next(&split);
    assert(s.len == 0);
    assert(s.data == data + 1);
    assert(split.s.len == 0);
    assert(split.s.data == 0);

    s = split_next(&split);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
}

void split_next_test_8() {
    char *data;
    slice_t s;
    split_t split;

    data = "\na\n";
    s = slice_new(data, strlen(data));
    split = split_new(s, '\n');
    
    s = split_next(&split);
    assert(s.len == 0);
    assert(s.data == data);
    assert(split.s.len == 2);
    assert(split.s.data == data + 1);
    
    s = split_next(&split);
    assert(s.len == 1);
    assert(s.data == data + 1);
    assert(split.s.len == 0);
    assert(split.s.data == data + 2);

    s = split_next(&split);
    assert(s.len == 0);
    assert(s.data == data + 2);
    assert(split.s.len == 0);
    assert(split.s.data == 0);

    s = split_next(&split);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
}

void split_next_test_9() {
    char *data;
    slice_t s;
    split_t split;

    data = "\nab\nc\nde\n";
    s = slice_new(data, strlen(data));
    split = split_new(s, '\n');

    s = split_next(&split);
    assert(s.len == 0);
    assert(s.data == data);
    assert(split.s.len == 8);
    assert(split.s.data == data + 1);

    s = split_next(&split);
    assert(s.len == 2);
    assert(s.data == data + 1);
    assert(split.s.len == 5);
    assert(split.s.data == data + 4);

    s = split_next(&split);
    assert(s.len == 1);
    assert(s.data == data + 4);
    assert(split.s.len == 3);
    assert(split.s.data == data + 6);

    s = split_next(&split);
    assert(s.len == 2);
    assert(s.data == data + 6);
    assert(split.s.len == 0);
    assert(split.s.data == data + 8);

    s = split_next(&split);
    assert(s.len == 0);
    assert(s.data == data + 8);
    assert(split.s.len == 0);
    assert(split.s.data == 0);

    s = split_next(&split);
    assert(s.len == 0);
    assert(s.data == 0);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
}

void split_next_test_helper(char *data, char **want) {
    slice_t s;
    split_t split;

    s = slice_new(data, strlen(data));
    split = split_new(s, '\n');

    while (*want) {
        s = split_next(&split);
        assert(s.len == strlen(*want));
        assert(s.data != 0);
        assert(strncmp(s.data, *want, s.len) == 0);
        want++;
    }

    s = split_next(&split);
    assert(s.len == 0);
    assert(s.data == 0);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
}

void split_next_test_10() {
    struct {
        char *data;
        char *want[100];
    } tests[] = {
        {
            "ab\nc\nde",
            {
                "ab", "c", "de",
            }
        },
        {
            "ab\nc\nde\n",
            {
                "ab", "c", "de", "",
            }
        },
        {
            "\nab\nc\nde",
            {
                "", "ab", "c", "de",
            }
        },
        {
            "\n\nabcd\n\n\nefg\nhijk\n\nl\n",
            {
                "", "", "abcd", "", "", "efg", "hijk", "", "l", "",
            }
        },
    };
    int i;

    for (i = 0; i < ARRAY_SIZE(tests); i++) {
        split_next_test_helper(tests[i].data, tests[i].want);
    }
}

static char* headers[] = {
    "HTTP/1.1 200 OK",
    "Accept-Ranges: bytes",
    "Cache-Control: private, no-cache, no-store, proxy-revalidate, no-transform",
    "Connection: keep-alive",
    "Content-Length: 2381",
    "Content-Type: text/html",
    "Date: Sat, 11 Mar 2023 08:52:44 GMT",
    "Etag: \"588604f8-94d\"",
    "Last-Modified: Mon, 23 Jan 2017 13:28:24 GMT",
    "Pragma: no-cache",
    "Server: bfe/1.0.8.18",
    "Set-Cookie: BDORZ=27315; max-age=86400; domain=.baidu.com; path=/",
};
#define SEP "\r\n"
#define MAX_BUF 1024

static int buf_init(char *buf, int buf_len, char *body, int body_len) {
    int i, len, total_len = 0;
    for (i = 0; i < ARRAY_SIZE(headers); i++) {
        len = strlen(headers[i]);
        assert(total_len + len <= buf_len);
        memcpy(buf + total_len, headers[i], len);
        total_len += len;
        
        len = strlen(SEP);
        assert(total_len + len <= buf_len);
        memcpy(buf + total_len, SEP, len);
        total_len += len;
    }
    
    len = strlen(SEP);
    assert(total_len + len <= buf_len);
    memcpy(buf + total_len, SEP, len);
    total_len += len;

    if (body_len <= 0) {
        return total_len;
    }

    len = body_len;
    assert(total_len + len <= buf_len);
    memcpy(buf + total_len, body, len);
    total_len += len;

    return total_len;
}

static void split_next_ext_test_1() {
    int i;
    char buf[MAX_BUF];
    split_t split;
    slice_t line;
    char *body = "hello world!";

    // without body
    i = buf_init(buf, MAX_BUF, NULL, 0);
    split = split_new_ext(buf, i, SEP, strlen(SEP));
    for (i = 0; i < ARRAY_SIZE(headers); i++) {
        line = split_next_ext(&split);
        assert(line.len == strlen(headers[i]) && !strncmp(line.data, headers[i], line.len));
    }
    line = split_next_ext(&split);
    assert(line.len == 0 && line.data);
    line = split_next_ext(&split);
    assert(line.len == 0 && !line.data);

    // with body
    i = buf_init(buf, MAX_BUF, body, strlen(body));
    split = split_new_ext(buf, i, SEP, strlen(SEP));
    for (i = 0; i < ARRAY_SIZE(headers); i++) {
        line = split_next_ext(&split);
        assert(line.len == strlen(headers[i]) && !strncmp(line.data, headers[i], line.len));
    }
    line = split_next_ext(&split);
    assert(line.len == 0 && line.data);
    assert(split.s.len == strlen(body) && !strncmp(split.s.data, body, split.s.len));
    line = split_next_ext(&split);
    assert(line.len == 0 && !line.data);
    assert(split.s.len == strlen(body) && !strncmp(split.s.data, body, split.s.len));

    // parse
    i = buf_init(buf, MAX_BUF, body, strlen(body));
    split = split_new_ext(buf, i, SEP, strlen(SEP));
    i = 0;
    while (1) {
        line = split_next_ext(&split);
        if (line.len != 0) {
            assert(line.len == strlen(headers[i]) && !strncmp(line.data, headers[i], line.len));
            i++;
        } else {
            if (line.data) {
                assert(split.s.len == strlen(body) && !strncmp(split.s.data, body, split.s.len));
                assert(i == ARRAY_SIZE(headers));
            } else {
                assert(split.s.len == strlen(body) && !strncmp(split.s.data, body, split.s.len));
                break;
            }
        }
    }
}

void split_test() {
    split_next_test_1();
    split_next_test_2();
    split_next_test_3();
    split_next_test_4();
    split_next_test_5();
    split_next_test_6();
    split_next_test_7();
    split_next_test_8();
    split_next_test_9();
    split_next_test_10();
    
    split_next_ext_test_1();
}

#if 0
void h_split_next_test_2() {
    char *data = "abcd:";
    h_slice s;
    h_slice_split split;

    s = h_slice_new(data, strlen(data));
    split = h_slice_split_new(s, ':');
    
    s = h_slice_split_next(&split);
    assert(s.len == 4);
    assert(strncmp(s.data, "abcd", s.len) == 0);

    s = h_slice_split_next(&split);
    assert(s.data[0] == ':');
    assert(s.len == 0);

    assert(h_slice_split_next(&split).data == 0);
}

void h_slice_split_next_test_3() {
    char *data = ":a:b:c:";
    h_slice s;
    h_slice_split split;

    s = h_slice_new(data, strlen(data));
    split = h_slice_split_new(s, ':');
    
    s = h_slice_split_next(&split);
    assert(s.len == 0);
    assert(s.data[s.len] == ':');

    s = h_slice_split_next(&split);
    assert(s.len == 1);
    assert(s.data[0] == 'a');

    s = h_slice_split_next(&split);
    assert(s.len == 1);
    assert(s.data[0] == 'b');

    s = h_slice_split_next(&split);
    assert(s.len == 1);
    assert(s.data[0] == 'c');

    s = h_slice_split_next(&split);
    assert(s.len == 0);
    assert(s.data[s.len] == ':');

    s = h_slice_split_next(&split);
    assert(s.len == 0);
    assert(s.data == 0);
}

void h_slice_split_next_test_4() {
    char *data = "a:b:c:";
    h_slice s;
    h_slice_split split;

    s = h_slice_new(data, strlen(data));
    split = h_slice_split_new(s, ':');

    s = h_slice_split_next(&split);
    assert(s.len == 1);
    assert(s.data[0] == 'a');

    s = h_slice_split_next(&split);
    assert(s.len == 1);
    assert(s.data[0] == 'b');

    s = h_slice_split_next(&split);
    assert(s.len == 1);
    assert(s.data[0] == 'c');

    s = h_slice_split_next(&split);
    assert(s.len == 0);
    assert(s.data[s.len] == ':');

    s = h_slice_split_next(&split);
    assert(s.len == 0);
    assert(s.data == 0);
}

void h_slice_split_next_test_5() {
    char *data = "a:b:c";
    h_slice s;
    h_slice_split split;

    s = h_slice_new(data, strlen(data));
    split = h_slice_split_new(s, ':');

    s = h_slice_split_next(&split);
    assert(s.len == 1);
    assert(s.data[0] == 'a');

    s = h_slice_split_next(&split);
    assert(s.len == 1);
    assert(s.data[0] == 'b');

    s = h_slice_split_next(&split);
    assert(s.len == 1);
    assert(s.data[0] == 'c');

    s = h_slice_split_next(&split);
    assert(s.len == 0);
    assert(s.data == 0);
}

void h_slice_split_next_test_6() {
    char *data = ":";
    h_slice s;
    h_slice_split split;

    s = h_slice_new(data, strlen(data));
    split = h_slice_split_new(s, ':');

    s = h_slice_split_next(&split);
    assert(s.len == 0);
    assert(s.data[0] == ':');

    s = h_slice_split_next(&split);
    assert(s.len == 0);
    assert(s.data[0] == ':');

    s = h_slice_split_next(&split);
    assert(s.len == 0);
    assert(s.data == 0);
}

void h_slice_split_next_test_7() {
    char *data = "   ";
    h_slice s;
    h_slice_split split;

    s = h_slice_new(data, strlen(data));
    split = h_slice_split_new(s, ' ');

    s = h_slice_split_next(&split);
    assert(s.len == 0);
    assert(s.data[0] == ' ');

    s = h_slice_split_next(&split);
    assert(s.len == 0);
    assert(s.data[0] == ' ');

    s = h_slice_split_next(&split);
    assert(s.len == 0);
    assert(s.data[0] == ' ');

    s = h_slice_split_next(&split);
    assert(s.len == 0);
    assert(s.data[0] == ' ');

    s = h_slice_split_next(&split);
    assert(s.len == 0);
    assert(s.data == 0);
}

void h_slice_headers_test() {
    char **headers;
    char *buf;
    int len, i;
    h_slice s;
    h_split lines;

    init(keys, values, ARRAY_SIZE(keys), &headers, &buf, &len);

    s = h_slice_new(buf, len);
    lines = h_slice_split_new(s, '\n');
    for (i = 0; ;i++) {
        h_slice line, s;
        h_slice_split kvs;

        line = h_slice_split_next(&lines);

        if (line.len == 0) {
            break;
        }

        line = h_slice_trim_space(line);
        assert(line.len == strlen(headers[i]));
        assert(strncmp(line.data, headers[i], line.len) == 0);

        kvs = h_slice_split_new(line, ':');
        s = h_slice_split_next(&kvs);
        s = h_slice_trim_space(s);
        assert(s.len == strlen(keys[i]));
        assert(strncmp(s.data, keys[i], s.len) == 0);

        s = h_slice_split_next(&kvs);
        s = h_slice_trim_space(s);
        assert(s.len == strlen(values[i]));
        assert(strncmp(s.data, values[i], s.len) == 0);
    }

    deinit(headers, ARRAY_SIZE(keys), buf);
}

#define KVS ": "
#define CRLF "\r\n"

char* keys[] = {
    "Host",
    "Upgrade",
};

char* values[] = {
    "www.example.com",
    "websocket",
};

static void init(char **keys, char **values, int len, char ***pheaders, char **pbuf, int *plen) {
    char *buf;
    int i, buf_len = 0;

    *pheaders = malloc(sizeof(char*) * len);
    for (i = 0; i < len; i++) {
        int header_len = strlen(keys[i]) + strlen(KVS) + strlen(values[i]);
        (*pheaders)[i] = malloc(header_len + 1);
        (*pheaders)[i][0] = 0;
        strcat((*pheaders)[i], keys[i]);
        strcat((*pheaders)[i], KVS);
        strcat((*pheaders)[i], values[i]);
        
        buf_len += header_len + strlen(KVS);
    }
    
    buf = malloc(buf_len+1);
    buf[0] = 0;
    for (i = 0; i < len; i++) {
        strcat(buf, (*pheaders)[i]);
        strcat(buf, CRLF);
    }

    *pbuf = buf;
    *plen = buf_len;
}

static void deinit(char **headers, int len, char *buf) {
    int i;

    for (i = 0; i < len; i++) {
        free(headers[i]);
    }
    free(headers);
    free(buf);
}
#endif
