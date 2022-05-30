#include <assert.h>
#include <string.h>
#include "buffer.h"

void h_buffer_append_header_test() {
    char *want = "Host:server.example.com\r\n"
        "Origin:http:://example.com\r\n"
        "Upgrade:websocket\r\n"
        "Connection:Upgrade\r\n"
        "Sec-Websocket-Key:dGhlIHNhbXBsZSBub25jZQ==\r\n"
        "Sec-Websocket-Version:3\r\n";
    h_buf buf = {0};

    h_buf_append_header(&buf, "Host", "server.example.com");
    assert(buf.data[buf.len] == 0);
    h_buf_append_header(&buf, "Origin", "http:://example.com");
    assert(buf.data[buf.len] == 0);
    h_buf_append_header(&buf, "Upgrade", "websocket");
    assert(buf.data[buf.len] == 0);
    h_buf_append_header(&buf, "Connection", "Upgrade");
    assert(buf.data[buf.len] == 0);
    h_buf_append_header(&buf, "Sec-Websocket-Key", "dGhlIHNhbXBsZSBub25jZQ==");
    assert(buf.data[buf.len] == 0);
    h_buf_append_header(&buf, "Sec-Websocket-Version", "3");
    assert(buf.data[buf.len] == 0);

    assert(strcmp(buf.data, want) == 0);
}

void h_buffer_delete_header_empty_test() {
    h_buf buf = {0};

    h_buf_delete_header(&buf, "Host");
    assert(buf.data == 0);
    assert(buf.cap == 0);
    assert(buf.len == 0);
}

void h_buffer_delete_header_one_test() {
    h_buf buf = {0};

    h_buf_append_header(&buf, "Host", "server.example.com");
    assert(buf.data[buf.len] == 0);
    h_buf_delete_header(&buf, "Host");
    assert(buf.len == 0);
    assert(buf.data[buf.len] == 0);

    h_buf_append_header(&buf, "Origin", "http:://example.com");
    assert(buf.data[buf.len] == 0);
    h_buf_delete_header(&buf, "Origin");
    assert(buf.len == 0);
    assert(buf.data[buf.len] == 0);
}

void h_buffer_delete_header_two_test() {
    h_buf buf = {0};

    h_buf_append_header(&buf, "Host", "server.example.com");
    assert(buf.data[buf.len] == 0);
    h_buf_append_header(&buf, "Origin", "http:://example.com");
    assert(buf.data[buf.len] == 0);
    h_buf_delete_header(&buf, "Host");
    assert(buf.data[buf.len] == 0);
    h_buf_delete_header(&buf, "Origin");
    assert(buf.len == 0);
    assert(buf.data[buf.len] == 0);

    h_buf_append_header(&buf, "Host", "server.example.com");
    assert(buf.data[buf.len] == 0);
    h_buf_append_header(&buf, "Origin", "http:://example.com");
    assert(buf.data[buf.len] == 0);
    h_buf_delete_header(&buf, "Origin");
    assert(buf.data[buf.len] == 0);
    h_buf_delete_header(&buf, "Host");
    assert(buf.len == 0);
    assert(buf.data[buf.len] == 0);

    h_buf_append_header(&buf, "Host", "server.example.com");
    assert(buf.data[buf.len] == 0);
    h_buf_append_header(&buf, "Origin", "http:://example.com");
    assert(buf.data[buf.len] == 0);
    h_buf_delete_header(&buf, "Origin");
    assert(buf.data[buf.len] == 0);
    h_buf_append_header(&buf, "Origin", "http:://example.com");
    assert(buf.data[buf.len] == 0);
    h_buf_delete_header(&buf, "Host");
    assert(buf.data[buf.len] == 0);
    h_buf_delete_header(&buf, "Origin");
    assert(buf.len == 0);
    assert(buf.data[buf.len] == 0);

    h_buf_append_header(&buf, "Host", "server.example.com");
    assert(buf.data[buf.len] == 0);
    h_buf_append_header(&buf, "Host", "server.example.com");
    assert(buf.data[buf.len] == 0);
    h_buf_delete_header(&buf, "Host");
    assert(buf.len == 0);
    assert(buf.data[buf.len] == 0);

    h_buf_append_header(&buf, "Host", "server.example.com");
    assert(buf.data[buf.len] == 0);
    h_buf_append_header(&buf, "Origin", "http:://example.com");
    assert(buf.data[buf.len] == 0);
    h_buf_append_header(&buf, "Host", "server.example.com");
    assert(buf.data[buf.len] == 0);
    h_buf_delete_header(&buf, "Origin");
    assert(buf.data[buf.len] == 0);
    h_buf_delete_header(&buf, "Host");
    assert(buf.len == 0);
    assert(buf.data[buf.len] == 0);

    h_buf_append_header(&buf, "Host", "server.example.com");
    assert(buf.data[buf.len] == 0);
    h_buf_append_header(&buf, "Origin", "http:://example.com");
    assert(buf.data[buf.len] == 0);
    h_buf_append_header(&buf, "Host", "server.example.com");
    assert(buf.data[buf.len] == 0);
    h_buf_delete_header(&buf, "Host");
    assert(buf.len != 0);
    assert(buf.data[buf.len] == 0);
    h_buf_delete_header(&buf, "Origin");
    assert(buf.len == 0);
    assert(buf.data[buf.len] == 0);
}

void h_buffer_write_test() {
    char *data;
    int len;
    h_buf buf = {0};
    
    data = "GET /chat HTTP/1.1";
    len = strlen(data);
    h_buf_write(&buf, data, len);
    assert(buf.cap == len);
    assert(buf.len == len);
    assert(strncmp(buf.data, data, buf.len) == 0);

    data = "\r";
    h_buf_write(&buf, data, strlen(data));
    assert(buf.cap == (len << 1));
    assert(buf.len == len + 1);
    assert(strncmp(buf.data + len, data, 1) == 0);

    data = "\n";
    h_buf_write(&buf, data, strlen(data));
    assert(buf.cap == (len << 1));
    assert(buf.len == len + 2);
    assert(strncmp(buf.data + len + 1, data, 1) == 0);
}

void buffer_test() {
    h_buffer_write_test();
    h_buffer_append_header_test();
    h_buffer_delete_header_empty_test();
    h_buffer_delete_header_one_test();
    h_buffer_delete_header_two_test();
}