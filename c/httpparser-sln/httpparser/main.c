#include "http_parser.h"

int main() {
    http_parser parser = {0};
    const char *buf = "GET / HT";
    size_t parsed;
    http_parser_settings settings_null = {0};

    http_parser_init(&parser, HTTP_REQUEST);
    parsed = http_parser_execute(&parser, &settings_null, buf, strlen(buf));
}