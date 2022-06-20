#include <assert.h>
#include <string.h>
#include "buffer.h"

void buffer_append_header_test() {
    char *want = "Host:server.example.com\r\n"
        "Origin:http:://example.com\r\n"
        "Upgrade:websocket\r\n"
        "Connection:Upgrade\r\n"
        "Sec-Websocket-Key:dGhlIHNhbXBsZSBub25jZQ==\r\n"
        "Sec-Websocket-Version:3\r\n";
    buf_t buf = {0};

    buf_append_header(&buf, "Host", "server.example.com");
    assert(buf.data[buf.len] == 0);
    buf_append_header(&buf, "Origin", "http:://example.com");
    assert(buf.data[buf.len] == 0);
    buf_append_header(&buf, "Upgrade", "websocket");
    assert(buf.data[buf.len] == 0);
    buf_append_header(&buf, "Connection", "Upgrade");
    assert(buf.data[buf.len] == 0);
    buf_append_header(&buf, "Sec-Websocket-Key", "dGhlIHNhbXBsZSBub25jZQ==");
    assert(buf.data[buf.len] == 0);
    buf_append_header(&buf, "Sec-Websocket-Version", "3");
    assert(buf.data[buf.len] == 0);

    assert(strcmp(buf.data, want) == 0);
}

void buffer_delete_header_empty_test() {
    buf_t buf = {0};

    buf_delete_header(&buf, "Host");
    assert(buf.data == 0);
    assert(buf.cap == 0);
    assert(buf.len == 0);
}

void buffer_delete_header_one_test() {
    buf_t buf = {0};

    buf_append_header(&buf, "Host", "server.example.com");
    assert(buf.data[buf.len] == 0);
    buf_delete_header(&buf, "Host");
    assert(buf.len == 0);
    assert(buf.data[buf.len] == 0);

    buf_append_header(&buf, "Origin", "http:://example.com");
    assert(buf.data[buf.len] == 0);
    buf_delete_header(&buf, "Origin");
    assert(buf.len == 0);
    assert(buf.data[buf.len] == 0);
}

void buffer_delete_header_two_test() {
    buf_t buf = {0};

    buf_append_header(&buf, "Host", "server.example.com");
    assert(buf.data[buf.len] == 0);
    buf_append_header(&buf, "Origin", "http:://example.com");
    assert(buf.data[buf.len] == 0);
    buf_delete_header(&buf, "Host");
    assert(buf.data[buf.len] == 0);
    buf_delete_header(&buf, "Origin");
    assert(buf.len == 0);
    assert(buf.data[buf.len] == 0);

    buf_append_header(&buf, "Host", "server.example.com");
    assert(buf.data[buf.len] == 0);
    buf_append_header(&buf, "Origin", "http:://example.com");
    assert(buf.data[buf.len] == 0);
    buf_delete_header(&buf, "Origin");
    assert(buf.data[buf.len] == 0);
    buf_delete_header(&buf, "Host");
    assert(buf.len == 0);
    assert(buf.data[buf.len] == 0);

    buf_append_header(&buf, "Host", "server.example.com");
    assert(buf.data[buf.len] == 0);
    buf_append_header(&buf, "Origin", "http:://example.com");
    assert(buf.data[buf.len] == 0);
    buf_delete_header(&buf, "Origin");
    assert(buf.data[buf.len] == 0);
    buf_append_header(&buf, "Origin", "http:://example.com");
    assert(buf.data[buf.len] == 0);
    buf_delete_header(&buf, "Host");
    assert(buf.data[buf.len] == 0);
    buf_delete_header(&buf, "Origin");
    assert(buf.len == 0);
    assert(buf.data[buf.len] == 0);

    buf_append_header(&buf, "Host", "server.example.com");
    assert(buf.data[buf.len] == 0);
    buf_append_header(&buf, "Host", "server.example.com");
    assert(buf.data[buf.len] == 0);
    buf_delete_header(&buf, "Host");
    assert(buf.len == 0);
    assert(buf.data[buf.len] == 0);

    buf_append_header(&buf, "Host", "server.example.com");
    assert(buf.data[buf.len] == 0);
    buf_append_header(&buf, "Origin", "http:://example.com");
    assert(buf.data[buf.len] == 0);
    buf_append_header(&buf, "Host", "server.example.com");
    assert(buf.data[buf.len] == 0);
    buf_delete_header(&buf, "Origin");
    assert(buf.data[buf.len] == 0);
    buf_delete_header(&buf, "Host");
    assert(buf.len == 0);
    assert(buf.data[buf.len] == 0);

    buf_append_header(&buf, "Host", "server.example.com");
    assert(buf.data[buf.len] == 0);
    buf_append_header(&buf, "Origin", "http:://example.com");
    assert(buf.data[buf.len] == 0);
    buf_append_header(&buf, "Host", "server.example.com");
    assert(buf.data[buf.len] == 0);
    buf_delete_header(&buf, "Host");
    assert(buf.len != 0);
    assert(buf.data[buf.len] == 0);
    buf_delete_header(&buf, "Origin");
    assert(buf.len == 0);
    assert(buf.data[buf.len] == 0);
}

void buffer_write_test() {
    char *data;
    int len;
    buf_t buf = {0};
    
    data = "GET /chat HTTP/1.1";
    len = strlen(data);
    buf_write(&buf, data, len);
    assert(buf.cap == len);
    assert(buf.len == len);
    assert(strncmp(buf.data, data, buf.len) == 0);

    data = "\r";
    buf_write(&buf, data, strlen(data));
    assert(buf.cap == (len << 1));
    assert(buf.len == len + 1);
    assert(strncmp(buf.data + len, data, 1) == 0);

    data = "\n";
    buf_write(&buf, data, strlen(data));
    assert(buf.cap == (len << 1));
    assert(buf.len == len + 2);
    assert(strncmp(buf.data + len + 1, data, 1) == 0);
}

void buffer_test() {
    buffer_write_test();
    buffer_append_header_test();
    buffer_delete_header_empty_test();
    buffer_delete_header_one_test();
    buffer_delete_header_two_test();
}