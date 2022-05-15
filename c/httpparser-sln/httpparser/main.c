#include "http_parser.h"
#include <string.h>
#include <assert.h>
#include <stdlib.h>

int header_field_cb (http_parser *p, const char *buf, size_t len);
int header_value_cb (http_parser *p, const char *buf, size_t len);
int headers_complete_cb (http_parser *p);

#define KV_LEN 100

typedef struct {
    char field[KV_LEN];
    char value[KV_LEN];
    int is_value;
    int len;
    int type;
} Context;

static Context gCtx;

int main() {
    http_parser parser = {0};
    http_parser_settings settings_null = {
        NULL,
        NULL,
        NULL,
        header_field_cb,
        header_value_cb,
        headers_complete_cb,
    };
    
    char *buf = "HTTP/1.1 200 OK\r\n"
         "Server: DCLK-AdSvr\r\n"
         "Content-Type: text/xml\r\n"
         "Content-Length: 100\r\n"
         "\r\n";
    int i = 0;
    size_t parsed;

    http_parser_init(&parser, HTTP_RESPONSE);
    
    for (i = 0; i < strlen(buf); i++) {
        parsed = http_parser_execute(&parser, &settings_null, buf + i, 1);
    }

    assert(parser.http_errno == 0);
    assert(gCtx.len == 100);
    assert(gCtx.type == 2);
}

void parse_interested_header() {
    if (!gCtx.is_value) {
        return;
    }

    if (strcmp(gCtx.field, "Content-Length") == 0) {
        gCtx.len = atoi(gCtx.value);
    } else if (strcmp(gCtx.field, "Content-Type") == 0) {
        if (strcmp(gCtx.value, "text/xml") == 0) {
            gCtx.type = 2;
        }
    }

    gCtx.field[0] = 0;
}

int header_field_cb (http_parser *p, const char *buf, size_t len) {
    parse_interested_header();
    gCtx.is_value = 0;
    strncat(gCtx.field, buf, len);
    return 0;
}

int header_value_cb (http_parser *p, const char *buf, size_t len) {
    if (!gCtx.is_value) {
        gCtx.value[0] = 0;
    }
    gCtx.is_value = 1;
    strncat(gCtx.value, buf, len);
    return 0;
}

int headers_complete_cb (http_parser *p) {
    parse_interested_header();
    return 0;
}
