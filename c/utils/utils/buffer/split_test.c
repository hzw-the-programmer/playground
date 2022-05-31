#include <string.h>
#include <assert.h>
#include "split.h"

void h_split_next_test_1() {
    char *data;
    h_slice s;
    h_split split;

    data = "";
    s = h_slice_new(data, strlen(data));
    split = h_split_new(s, '\n');
    
    s = h_split_next(&split);
    assert(s.len == 0);
    assert(s.data == data);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
    
    s = h_split_next(&split);
    assert(s.len == 0);
    assert(s.data == 0);
    assert(split.s.len == 0);
    assert(split.s.data == 0);

    data = "\n";
    s = h_slice_new(data, strlen(data));
    split = h_split_new(s, '\n');
   
    s = h_split_next(&split);
    assert(s.len == 0);
    assert(s.data == data);
    assert(split.s.len == 0);
    assert(split.s.data == data);
    
    s = h_split_next(&split);
    assert(s.len == 0);
    assert(s.data == data);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
    
    s = h_split_next(&split);
    assert(s.len == 0);
    assert(s.data == 0);
    assert(split.s.len == 0);
    assert(split.s.data == 0);

    data = "\n\n";
    s = h_slice_new(data, strlen(data));
    split = h_split_new(s, '\n');
    
    s = h_split_next(&split);
    assert(s.len == 0);
    assert(s.data == data);
    assert(split.s.len == 1);
    assert(split.s.data == data + 1);
    
    s = h_split_next(&split);
    assert(s.len == 0);
    assert(s.data == data + 1);
    assert(split.s.len == 0);
    assert(split.s.data == data + 1);
    
    s = h_split_next(&split);
    assert(s.len == 0);
    assert(s.data == data + 1);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
    
    s = h_split_next(&split);
    assert(s.len == 0);
    assert(s.data == 0);
    assert(split.s.len == 0);
    assert(split.s.data == 0);

    data = "\n\n\n";
    s = h_slice_new(data, strlen(data));
    split = h_split_new(s, '\n');
    
    s = h_split_next(&split);
    assert(s.len == 0);
    assert(s.data == data);
    assert(split.s.len == 2);
    assert(split.s.data == data + 1);
    
    s = h_split_next(&split);
    assert(s.len == 0);
    assert(s.data == data + 1);
    assert(split.s.len == 1);
    assert(split.s.data == data + 2);
    
    s = h_split_next(&split);
    assert(s.len == 0);
    assert(s.data == data + 2);
    assert(split.s.len == 0);
    assert(split.s.data == data + 2);

    s = h_split_next(&split);
    assert(s.len == 0);
    assert(s.data == data + 2);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
   
    s = h_split_next(&split);
    assert(s.len == 0);
    assert(s.data == 0);
    assert(split.s.len == 0);
    assert(split.s.data == 0);

    data = "a";
    s = h_slice_new(data, strlen(data));
    split = h_split_new(s, '\n');
    
    s = h_split_next(&split);
    assert(s.len == 1);
    assert(s.data == data);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
    
    s = h_split_next(&split);
    assert(s.len == 0);
    assert(s.data == 0);
    assert(split.s.len == 0);
    assert(split.s.data == 0);

    data = "\na";
    s = h_slice_new(data, strlen(data));
    split = h_split_new(s, '\n');
    
    s = h_split_next(&split);
    assert(s.len == 0);
    assert(s.data == data);
    assert(split.s.len == 1);
    assert(split.s.data == data + 1);
    
    s = h_split_next(&split);
    assert(s.len == 1);
    assert(s.data == data + 1);
    assert(split.s.len == 0);
    assert(split.s.data == 0);

    s = h_split_next(&split);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
    assert(split.s.len == 0);
    assert(split.s.data == 0);

    data = "a\n";
    s = h_slice_new(data, strlen(data));
    split = h_split_new(s, '\n');
    
    s = h_split_next(&split);
    assert(s.len == 1);
    assert(s.data == data);
    assert(split.s.len == 0);
    assert(split.s.data == data + 1);
    
    s = h_split_next(&split);
    assert(s.len == 0);
    assert(s.data == data + 1);
    assert(split.s.len == 0);
    assert(split.s.data == 0);

    s = h_split_next(&split);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
    assert(split.s.len == 0);
    assert(split.s.data == 0);

    data = "\na\n";
    s = h_slice_new(data, strlen(data));
    split = h_split_new(s, '\n');
    
    s = h_split_next(&split);
    assert(s.len == 0);
    assert(s.data == data);
    assert(split.s.len == 2);
    assert(split.s.data == data + 1);
    
    s = h_split_next(&split);
    assert(s.len == 1);
    assert(s.data == data + 1);
    assert(split.s.len == 0);
    assert(split.s.data == data + 2);

    s = h_split_next(&split);
    assert(s.len == 0);
    assert(s.data == data + 2);
    assert(split.s.len == 0);
    assert(split.s.data == 0);

    s = h_split_next(&split);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
    assert(split.s.len == 0);
    assert(split.s.data == 0);
}

void split_test() {
    h_split_next_test_1();
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
