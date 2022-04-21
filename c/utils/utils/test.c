#include <assert.h>
#include <string.h>
#include "test.h"

#define DEBUG_BUF_LEN (4 * 1024)

static char debug_buf[DEBUG_BUF_LEN];

void debug_buf_clear() {
    memset(debug_buf, 0, sizeof(debug_buf));
}

void debug_buf_append(const char *buf) {
    int len = 0;

    len = strlen(debug_buf);
    if (len != 0) {
        strcat(debug_buf, "\n");
    }
    strcat(debug_buf, buf);
}

void debug_buf_assert(const char *buf) {
    assert(strcmp(debug_buf, buf) == 0);
}

void test_run(TestItem *items, int len) {
    int i = 0;

    for (i = 0; i < len; i++) {
        if (items[i].setup) items[i].setup();
        items[i].func();
        if (items[i].teardown) items[i].teardown();
    }
}
