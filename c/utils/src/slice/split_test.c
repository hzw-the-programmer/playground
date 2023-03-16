#include <string.h>
#include <assert.h>
#include "split.h"
#include "utils.h"
#include "mem/mem.h"
#include "buffer/buffer.h"
#include "socket/sock_mock.h"

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
#define CRNL "\r\n"
#define MAX_BUF 1024

static void write_http(buf_t *buf, char **headers, int headers_len, char *body, int body_len) {
    int i;
    slice_t sep = {CRNL, strlen(CRNL)};
    slice_t slice;

    for (i = 0; i < headers_len; i++) {
        slice.data = headers[i];
        slice.len = strlen(headers[i]);
        buf_write(buf, &slice);
        buf_write(buf, &sep);
    }
    buf_write(buf, &sep);

    if (body_len <= 0) {
        return;
    }
    slice.data = body;
    slice.len = body_len;
    buf_write(buf, &slice);
}

static void split_next_ext_test_1() {
    int i;
    split_t split;
    slice_t line;
    char *t = "hello world!";
    slice_t body = {t, strlen(t)};
    buf_t *buf;

    buf = buf_new(MAX_BUF);
    assert(buf);

    // without body
    write_http(buf, headers, ARRAY_SIZE(headers), NULL, 0);

    split = split_new_ext(buf_read_ptr(buf), buf_buffered(buf), CRNL, strlen(CRNL));
    for (i = 0; i < ARRAY_SIZE(headers); i++) {
        line = split_next_ext(&split);
        assert(line.len == strlen(headers[i]) && !strncmp(line.data, headers[i], line.len));
    }
    line = split_next_ext(&split);
    assert(line.len == 0 && line.data);
    line = split_next_ext(&split);
    assert(line.len == 0 && !line.data);

    // with body
    buf_write(buf, &body);

    split = split_new_ext(buf_read_ptr(buf), buf_buffered(buf), CRNL, strlen(CRNL));
    for (i = 0; i < ARRAY_SIZE(headers); i++) {
        line = split_next_ext(&split);
        assert(line.len == strlen(headers[i]) && !strncmp(line.data, headers[i], line.len));
    }
    line = split_next_ext(&split);
    assert(line.len == 0 && line.data);
    assert(split.s.len == body.len && !strncmp(split.s.data, body.data, split.s.len));
    line = split_next_ext(&split);
    assert(line.len == 0 && !line.data);
    assert(split.s.len == body.len && !strncmp(split.s.data, body.data, split.s.len));

    // parse
    split = split_new_ext(buf_read_ptr(buf), buf_buffered(buf), CRNL, strlen(CRNL));
    i = 0;
    while (1) {
        line = split_next_ext(&split);
        if (line.len != 0) {
            assert(line.len == strlen(headers[i]) && !strncmp(line.data, headers[i], line.len));
            i++;
        } else {
            if (line.data) {
                assert(split.s.len == body.len && !strncmp(split.s.data, body.data, split.s.len));
                assert(i == ARRAY_SIZE(headers));
            } else {
                assert(split.s.len == body.len && !strncmp(split.s.data, body.data, split.s.len));
                break;
            }
        }
    }

    free(buf);
}

typedef struct sock_s {
    int fd;
    buf_t *send_buf;
    buf_t *recv_buf;
    void *arg;
    void (*cb)(struct sock_s*);
} sock_t;

// assert(line.len == strlen(headers[i]) && !strncmp(line.data, headers[i], line.len));
// i++;

typedef enum {
    DISCONNECTED,
    CONNECTING,
    IDLE,
    FIRSTLINE,
    HEADERS,
    BODY,
    TRAILER,
    WRITING,
} STATE_T;

typedef struct {
    STATE_T state;
    int count;
} http_test_arg;

static int http_slice_test_cb(void *arg, slice_t *slice) {
    sock_t *sock = arg;
    http_test_arg *targ = sock->arg;

    if (slice->len != 0) {
        if (targ->state == FIRSTLINE) {
            targ->state = HEADERS;
        }
        assert(slice->len == strlen(headers[targ->count]) && !strncmp(slice->data, headers[targ->count], slice->len));
        targ->count++;
        return 1;
    }
    
    if (slice->data) {
        targ->state = BODY;
        assert(targ->count == ARRAY_SIZE(headers));
        return 0;
    }

    return 0;
}

static void http_test_cb(sock_t *sock) {
    http_test_arg *arg = sock->arg;
    buf_t *buf = sock->recv_buf;

    if (arg->state == FIRSTLINE || arg->state == HEADERS) {
        buf_split(buf, CRNL, strlen(CRNL), http_slice_test_cb, sock);    
    } else {
        //assert(buf->w == strlen(body) && !strncmp(sock.recv->ptr, body, sock.recv->w));
        //assert(buf_buffered(sock.recv) == strlen(body) && !strncmp(buf_read_ptr(sock.recv), body, buf_buffered(sock.recv)));
    }
}

static void split_next_ext_test_2() {
    char *body = "hello\r\n world!";
    int n, j;
    mock_sock_ctx_t *mctx;
    mock_sock_t *msock;
    sock_t sock = {0};
    http_test_arg arg = {FIRSTLINE, 0};

    mctx = mock_sock_ctx_new(2, MAX_BUF);
    msock = &mctx->sock[0];
    write_http(msock->buf, headers, ARRAY_SIZE(headers), body, strlen(body));

    sock.arg = &arg;
    sock.cb = http_test_cb;
    sock.recv_buf = buf_new(MAX_BUF);
    assert(sock.recv_buf);

    for (j = 1; j <= MAX_BUF; j++) {
        msock->buf->r = 0;
        msock->n = j;
        sock.recv_buf->w = 0;
        arg.state = FIRSTLINE;
        arg.count = 0;
        
        while (1) {
            n = mock_sock_recv(msock, buf_write_ptr(sock.recv_buf), buf_available(sock.recv_buf));
            if (n > 0) {
                buf_write_inc(sock.recv_buf, n);
                sock.cb(&sock);
            } else {
                break;
            }
        }
    }

    mock_sock_ctx_free(mctx);
    free(sock.recv_buf);
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
    split_next_ext_test_2();
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
