#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "slice.h"
#include "../utils.h"

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

void slice_ltrim_test() {
    struct {
        char *buf;
        char *want;
    } tests[] = {
        {"123   ", "123   "},
        {" 123   ", "123   "},
        {"\r 123   ", "123   "},
        {" \r 123   ", "123   "},
    };
    int i;

    for (i = 0; i < ARRAY_SIZE(tests); i++) {
        Slice s;

        s = slice_new(tests[i].buf, strlen(tests[i].buf));
        s = slice_ltrim(s);
        assert(strncmp(s.buf, tests[i].want, s.len) == 0);
    }
}

void slice_rtrim_test() {
    struct {
        char *buf;
        char *want;
    } tests[] = {
        {"   123", "   123"},
        {"   123 ", "   123"},
        {"   123 \r", "   123"},
        {"   123 \r ", "   123"},
    };
    int i;

    for (i = 0; i < ARRAY_SIZE(tests); i++) {
        Slice s;

        s = slice_new(tests[i].buf, strlen(tests[i].buf));
        s = slice_rtrim(s);
        assert(strncmp(s.buf, tests[i].want, s.len) == 0);
    }
}

void slice_trim_test() {
    struct {
        char *buf;
        char *want;
    } tests[] = {
        {"123", "123"},
        {" 123 ", "123"},
        {"\r 123 \r", "123"},
        {" \r 123 \r ", "123"},
    };
    int i;

    for (i = 0; i < ARRAY_SIZE(tests); i++) {
        Slice s;

        s = slice_new(tests[i].buf, strlen(tests[i].buf));
        s = slice_trim(s);
        assert(strncmp(s.buf, tests[i].want, s.len) == 0);
    }
}

void slice_split_next_test_1() {
    char *buf = "abcd";
    Slice s;
    SliceSplit split;

    s = slice_new(buf, strlen(buf));
    split = slice_split(s, ':');
    s = slice_split_next(&split);
    assert(s.len == 4);
    assert(strncmp(s.buf, "abcd", s.len) == 0);

    assert(slice_split_next(&split).buf == 0);
}

void slice_split_next_test_2() {
    char *buf = "abcd:";
    Slice s;
    SliceSplit split;

    s = slice_new(buf, strlen(buf));
    split = slice_split(s, ':');
    
    s = slice_split_next(&split);
    assert(s.len == 4);
    assert(strncmp(s.buf, "abcd", s.len) == 0);

    s = slice_split_next(&split);
    assert(s.buf[0] == ':');
    assert(s.len == 0);

    assert(slice_split_next(&split).buf == 0);
}

void slice_headers_test() {
    char **headers;
    char *buf;
    int len, i;
    Slice s;
    SliceSplit lines;

    init(keys, values, ARRAY_SIZE(keys), &headers, &buf, &len);

    s = slice_new(buf, len);
    lines = slice_split(s, '\n');
    for (i = 0; ;i++) {
        Slice line, s;
        SliceSplit kvs;

        line = slice_split_next(&lines);

        if (line.len == 0) {
            break;
        }

        line = slice_trim(line);
        assert(line.len == strlen(headers[i]));
        assert(strncmp(line.buf, headers[i], line.len) == 0);

        kvs = slice_split(line, ':');
        s = slice_split_next(&kvs);
        s = slice_trim(s);
        assert(s.len == strlen(keys[i]));
        assert(strncmp(s.buf, keys[i], s.len) == 0);

        s = slice_split_next(&kvs);
        s = slice_trim(s);
        assert(s.len == strlen(values[i]));
        assert(strncmp(s.buf, values[i], s.len) == 0);
    }

    deinit(headers, ARRAY_SIZE(keys), buf);
}

void slice_test() {
    slice_ltrim_test();
    slice_rtrim_test();
    slice_trim_test();
    slice_split_next_test_1();
    slice_split_next_test_2();
    slice_headers_test();
}
