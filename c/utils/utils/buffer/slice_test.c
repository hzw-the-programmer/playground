#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "slice.h"
#include "../utils.h"

void h_slice_ltrim_test_1() {
    struct {
        char *data;
        char *want;
    } tests[] = {
        {"123   ", "123   "},
        {" 123   ", "123   "},
        {"\r 123   ", "123   "},
        {" \r 123   ", "123   "},
    };
    int i;

    for (i = 0; i < ARRAY_SIZE(tests); i++) {
        h_slice s;

        s = h_slice_new(tests[i].data, strlen(tests[i].data));
        s = h_slice_ltrim_space(s);
        assert(strncmp(s.data, tests[i].want, s.len) == 0);
    }
}

void h_slice_ltrim_test_2() {
    char *data;
    h_slice s;

    data = "";
    s = h_slice_new(data, strlen(data));
    s = h_slice_ltrim_space(s);
    assert(s.len == 0);
    assert(s.data == data);
    assert(s.data[0] == 0);

    data = " ";
    s = h_slice_new(data, strlen(data));
    s = h_slice_ltrim_space(s);
    assert(s.len == 0);
    assert(s.data == data);
    assert(s.data[0] == ' ');

    data = " \r";
    s = h_slice_new(data, strlen(data));
    s = h_slice_ltrim_space(s);
    assert(s.len == 0);
    assert(s.data == data + 1);
    assert(s.data[0] == '\r');

    data = " \ra";
    s = h_slice_new(data, strlen(data));
    s = h_slice_ltrim_space(s);
    assert(s.len == 1);
    assert(s.data == data + 2);
    assert(s.data[0] == 'a');

    data = "a\r ";
    s = h_slice_new(data, strlen(data));
    s = h_slice_ltrim_space(s);
    assert(s.len == 3);
    assert(s.data == data);
    assert(s.data[0] == 'a');

    data = "a\r";
    s = h_slice_new(data, strlen(data));
    s = h_slice_ltrim_space(s);
    assert(s.len == 2);
    assert(s.data == data);
    assert(s.data[0] == 'a');

    data = "a";
    s = h_slice_new(data, strlen(data));
    s = h_slice_ltrim_space(s);
    assert(s.len == 1);
    assert(s.data == data);
    assert(s.data[0] == 'a');

    data = " \ra\r ";
    s = h_slice_new(data, strlen(data));
    s = h_slice_ltrim_space(s);
    assert(s.len == 3);
    assert(s.data == data + 2);
    assert(s.data[0] == 'a');

    data = "\ra\r ";
    s = h_slice_new(data, strlen(data));
    s = h_slice_ltrim_space(s);
    assert(s.len == 3);
    assert(s.data == data + 1);
    assert(s.data[0] == 'a');
}

void h_slice_rtrim_test_1() {
    struct {
        char *data;
        char *want;
    } tests[] = {
        {"   123", "   123"},
        {"   123 ", "   123"},
        {"   123 \r", "   123"},
        {"   123 \r ", "   123"},
    };
    int i;

    for (i = 0; i < ARRAY_SIZE(tests); i++) {
        h_slice s;

        s = h_slice_new(tests[i].data, strlen(tests[i].data));
        s = h_slice_rtrim_space(s);
        assert(strncmp(s.data, tests[i].want, s.len) == 0);
    }
}

void h_slice_rtrim_test_2() {
    char *data;
    h_slice s;

    data = "";
    s = h_slice_new(data, strlen(data));
    s = h_slice_rtrim_space(s);
    assert(s.len == 0);
    assert(s.data == data);
    assert(s.data[0] == 0);

    data = " ";
    s = h_slice_new(data, strlen(data));
    s = h_slice_rtrim_space(s);
    assert(s.len == 0);
    assert(s.data == data);
    assert(s.data[0] == ' ');

    data = " \r";
    s = h_slice_new(data, strlen(data));
    s = h_slice_rtrim_space(s);
    assert(s.len == 0);
    assert(s.data == data);
    assert(s.data[0] == ' ');

    data = " \ra";
    s = h_slice_new(data, strlen(data));
    s = h_slice_rtrim_space(s);
    assert(s.len == 3);
    assert(s.data == data);
    assert(s.data[0] == ' ');

    data = "a\r ";
    s = h_slice_new(data, strlen(data));
    s = h_slice_rtrim_space(s);
    assert(s.len == 1);
    assert(s.data == data);
    assert(s.data[0] == 'a');

    data = "a\r";
    s = h_slice_new(data, strlen(data));
    s = h_slice_rtrim_space(s);
    assert(s.len == 1);
    assert(s.data == data);
    assert(s.data[0] == 'a');

    data = "a";
    s = h_slice_new(data, strlen(data));
    s = h_slice_rtrim_space(s);
    assert(s.len == 1);
    assert(s.data == data);
    assert(s.data[0] == 'a');

    data = " \ra\r ";
    s = h_slice_new(data, strlen(data));
    s = h_slice_rtrim_space(s);
    assert(s.len == 3);
    assert(s.data == data);
    assert(s.data[0] == ' ');

    data = "\ra\r ";
    s = h_slice_new(data, strlen(data));
    s = h_slice_rtrim_space(s);
    assert(s.len == 2);
    assert(s.data == data);
    assert(s.data[0] == '\r');
}

void h_slice_trim_test_1() {
    struct {
        char *data;
        char *want;
    } tests[] = {
        {"123", "123"},
        {" 123 ", "123"},
        {"\r 123 \r", "123"},
        {" \r 123 \r ", "123"},
    };
    int i;

    for (i = 0; i < ARRAY_SIZE(tests); i++) {
        h_slice s;

        s = h_slice_new(tests[i].data, strlen(tests[i].data));
        s = h_slice_trim_space(s);
        assert(strncmp(s.data, tests[i].want, s.len) == 0);
    }
}

void h_slice_trim_test_2() {
    char *data;
    h_slice s;

    data = "";
    s = h_slice_new(data, strlen(data));
    s = h_slice_trim_space(s);
    assert(s.len == 0);
    assert(s.data == data);
    assert(s.data[0] == 0);

    data = " ";
    s = h_slice_new(data, strlen(data));
    s = h_slice_trim_space(s);
    assert(s.len == 0);
    assert(s.data == data);
    assert(s.data[0] == ' ');

    data = " \r";
    s = h_slice_new(data, strlen(data));
    s = h_slice_trim_space(s);
    assert(s.len == 0);
    assert(s.data == data + 1);
    assert(s.data[0] == '\r');

    data = " \ra";
    s = h_slice_new(data, strlen(data));
    s = h_slice_trim_space(s);
    assert(s.len == 1);
    assert(s.data == data + 2);
    assert(s.data[0] == 'a');

    data = "a\r ";
    s = h_slice_new(data, strlen(data));
    s = h_slice_trim_space(s);
    assert(s.len == 1);
    assert(s.data == data);
    assert(s.data[0] == 'a');

    data = "a\r";
    s = h_slice_new(data, strlen(data));
    s = h_slice_trim_space(s);
    assert(s.len == 1);
    assert(s.data == data);
    assert(s.data[0] == 'a');

    data = "a";
    s = h_slice_new(data, strlen(data));
    s = h_slice_trim_space(s);
    assert(s.len == 1);
    assert(s.data == data);
    assert(s.data[0] == 'a');

    data = " \ra\r ";
    s = h_slice_new(data, strlen(data));
    s = h_slice_trim_space(s);
    assert(s.len == 1);
    assert(s.data == data + 2);
    assert(s.data[0] == 'a');

    data = "\ra\r ";
    s = h_slice_new(data, strlen(data));
    s = h_slice_trim_space(s);
    assert(s.len == 1);
    assert(s.data == data + 1);
    assert(s.data[0] == 'a');
}

void slice_test() {
    h_slice_ltrim_test_1();
    h_slice_ltrim_test_2();
    h_slice_rtrim_test_1();
    h_slice_rtrim_test_2();
    h_slice_trim_test_1();
    h_slice_trim_test_2();
}
