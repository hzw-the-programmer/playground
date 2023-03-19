#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "slice.h"
#include "utils.h"

void slice_ltrim_test_1() {
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
        slice_t s;

        s = slice_new(tests[i].data, strlen(tests[i].data));
        s = slice_ltrim_space(s);
        assert(strncmp(s.ptr, tests[i].want, s.len) == 0);
    }
}

void slice_ltrim_test_2() {
    char *data;
    slice_t s;

    data = "";
    s = slice_new(data, strlen(data));
    s = slice_ltrim_space(s);
    assert(s.len == 0);
    assert(s.ptr == data);
    assert(s.ptr[0] == 0);

    data = " ";
    s = slice_new(data, strlen(data));
    s = slice_ltrim_space(s);
    assert(s.len == 0);
    assert(s.ptr == data);
    assert(s.ptr[0] == ' ');

    data = " \r";
    s = slice_new(data, strlen(data));
    s = slice_ltrim_space(s);
    assert(s.len == 0);
    assert(s.ptr == data + 1);
    assert(s.ptr[0] == '\r');

    data = " \ra";
    s = slice_new(data, strlen(data));
    s = slice_ltrim_space(s);
    assert(s.len == 1);
    assert(s.ptr == data + 2);
    assert(s.ptr[0] == 'a');

    data = "a\r ";
    s = slice_new(data, strlen(data));
    s = slice_ltrim_space(s);
    assert(s.len == 3);
    assert(s.ptr == data);
    assert(s.ptr[0] == 'a');

    data = "a\r";
    s = slice_new(data, strlen(data));
    s = slice_ltrim_space(s);
    assert(s.len == 2);
    assert(s.ptr == data);
    assert(s.ptr[0] == 'a');

    data = "a";
    s = slice_new(data, strlen(data));
    s = slice_ltrim_space(s);
    assert(s.len == 1);
    assert(s.ptr == data);
    assert(s.ptr[0] == 'a');

    data = " \ra\r ";
    s = slice_new(data, strlen(data));
    s = slice_ltrim_space(s);
    assert(s.len == 3);
    assert(s.ptr == data + 2);
    assert(s.ptr[0] == 'a');

    data = "\ra\r ";
    s = slice_new(data, strlen(data));
    s = slice_ltrim_space(s);
    assert(s.len == 3);
    assert(s.ptr == data + 1);
    assert(s.ptr[0] == 'a');
}

void slice_rtrim_test_1() {
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
        slice_t s;

        s = slice_new(tests[i].data, strlen(tests[i].data));
        s = slice_rtrim_space(s);
        assert(strncmp(s.ptr, tests[i].want, s.len) == 0);
    }
}

void slice_rtrim_test_2() {
    char *data;
    slice_t s;

    data = "";
    s = slice_new(data, strlen(data));
    s = slice_rtrim_space(s);
    assert(s.len == 0);
    assert(s.ptr == data);
    assert(s.ptr[0] == 0);

    data = " ";
    s = slice_new(data, strlen(data));
    s = slice_rtrim_space(s);
    assert(s.len == 0);
    assert(s.ptr == data);
    assert(s.ptr[0] == ' ');

    data = " \r";
    s = slice_new(data, strlen(data));
    s = slice_rtrim_space(s);
    assert(s.len == 0);
    assert(s.ptr == data);
    assert(s.ptr[0] == ' ');

    data = " \ra";
    s = slice_new(data, strlen(data));
    s = slice_rtrim_space(s);
    assert(s.len == 3);
    assert(s.ptr == data);
    assert(s.ptr[0] == ' ');

    data = "a\r ";
    s = slice_new(data, strlen(data));
    s = slice_rtrim_space(s);
    assert(s.len == 1);
    assert(s.ptr == data);
    assert(s.ptr[0] == 'a');

    data = "a\r";
    s = slice_new(data, strlen(data));
    s = slice_rtrim_space(s);
    assert(s.len == 1);
    assert(s.ptr == data);
    assert(s.ptr[0] == 'a');

    data = "a";
    s = slice_new(data, strlen(data));
    s = slice_rtrim_space(s);
    assert(s.len == 1);
    assert(s.ptr == data);
    assert(s.ptr[0] == 'a');

    data = " \ra\r ";
    s = slice_new(data, strlen(data));
    s = slice_rtrim_space(s);
    assert(s.len == 3);
    assert(s.ptr == data);
    assert(s.ptr[0] == ' ');

    data = "\ra\r ";
    s = slice_new(data, strlen(data));
    s = slice_rtrim_space(s);
    assert(s.len == 2);
    assert(s.ptr == data);
    assert(s.ptr[0] == '\r');
}

void slice_trim_test_1() {
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
        slice_t s;

        s = slice_new(tests[i].data, strlen(tests[i].data));
        s = slice_trim_space(s);
        assert(strncmp(s.ptr, tests[i].want, s.len) == 0);
    }
}

void slice_trim_test_2() {
    char *data;
    slice_t s;

    data = "";
    s = slice_new(data, strlen(data));
    s = slice_trim_space(s);
    assert(s.len == 0);
    assert(s.ptr == data);
    assert(s.ptr[0] == 0);

    data = " ";
    s = slice_new(data, strlen(data));
    s = slice_trim_space(s);
    assert(s.len == 0);
    assert(s.ptr == data);
    assert(s.ptr[0] == ' ');

    data = " \r";
    s = slice_new(data, strlen(data));
    s = slice_trim_space(s);
    assert(s.len == 0);
    assert(s.ptr == data + 1);
    assert(s.ptr[0] == '\r');

    data = " \ra";
    s = slice_new(data, strlen(data));
    s = slice_trim_space(s);
    assert(s.len == 1);
    assert(s.ptr == data + 2);
    assert(s.ptr[0] == 'a');

    data = "a\r ";
    s = slice_new(data, strlen(data));
    s = slice_trim_space(s);
    assert(s.len == 1);
    assert(s.ptr == data);
    assert(s.ptr[0] == 'a');

    data = "a\r";
    s = slice_new(data, strlen(data));
    s = slice_trim_space(s);
    assert(s.len == 1);
    assert(s.ptr == data);
    assert(s.ptr[0] == 'a');

    data = "a";
    s = slice_new(data, strlen(data));
    s = slice_trim_space(s);
    assert(s.len == 1);
    assert(s.ptr == data);
    assert(s.ptr[0] == 'a');

    data = " \ra\r ";
    s = slice_new(data, strlen(data));
    s = slice_trim_space(s);
    assert(s.len == 1);
    assert(s.ptr == data + 2);
    assert(s.ptr[0] == 'a');

    data = "\ra\r ";
    s = slice_new(data, strlen(data));
    s = slice_trim_space(s);
    assert(s.len == 1);
    assert(s.ptr == data + 1);
    assert(s.ptr[0] == 'a');
}

void slice_to_uint64_test() {
    struct {
        char *str;
        uint64_t n;
    } tests[] = {
        {
            "1234",
            1234,
        },
        {
            "1234abc",
            1234,
        },
    };
    int i;

    for (i = 0; i < ARRAY_SIZE(tests); i++) {
        assert(slice_to_uint64(slice_new(tests[i].str, strlen(tests[i].str))) == tests[i].n);
    }
}

static void slice_slice_test() {
    slice_t s, ss, r;

    s.ptr = "\r\n";
    s.len = strlen(s.ptr);
    
    ss.ptr = "\r";
    ss.len = strlen(ss.ptr);
    r = slice_slice(ss, s);
    assert(!r.ptr && !r.len);

    ss.ptr = "\r\n";
    ss.len = strlen(ss.ptr);
    r = slice_slice(ss, s);
    assert(r.ptr == ss.ptr && r.len == 2);

    ss.ptr = "\r\r\n";
    ss.len = strlen(ss.ptr);
    r = slice_slice(ss, s);
    assert(r.ptr == ss.ptr+1 && r.len == 2);

    ss.ptr = "\rb\r\r\n";
    ss.len = strlen(ss.ptr);
    r = slice_slice(ss, s);
    assert(r.ptr == ss.ptr+3 && r.len == 2);
}

void slice_test() {
    slice_ltrim_test_1();
    slice_ltrim_test_2();
    slice_rtrim_test_1();
    slice_rtrim_test_2();
    slice_trim_test_1();
    slice_trim_test_2();
    slice_to_uint64_test();
    slice_slice_test();
}
