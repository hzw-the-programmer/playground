#include <string.h>
#include <assert.h>
#include "split.h"
#include "utils.h"
#include "mem/mem.h"
#include "buffer/buffer.h"
#include "socket/sock_mock.h"

void split_next_test_1() {
    char *data, *sep;
    int data_len, sep_len;
    split_t split;
    slice_t slice;

    data = "";
    data_len = 0;
    sep = "\r";
    sep_len = 1;

    split = split_new(slice_new(data, data_len), slice_new(sep, sep_len));
    
    slice = split_next(&split);
    assert(slice.len == 0);
    assert(slice.data == data);
    assert(split.slice.len == 0);
    assert(split.slice.data == NULL);
    
    slice = split_next(&split);
    assert(slice.len == 0);
    assert(slice.data == NULL);
    assert(split.slice.len == 0);
    assert(split.slice.data == NULL);
}

void split_next_test_2() {
    char *data, *sep;
    int data_len, sep_len;
    split_t split;
    slice_t slice;

    data = "\n";
    data_len = 1;
    sep = "\n";
    sep_len = 1;

    split = split_new(slice_new(data, data_len), slice_new(sep, sep_len));
   
    slice = split_next(&split);
    assert(slice.len == 0);
    assert(slice.data == data);
    assert(split.slice.len == 0);
    assert(split.slice.data == data + 1);
    
    slice = split_next(&split);
    assert(slice.len == 0);
    assert(slice.data == data + 1);
    assert(split.slice.len == 0);
    assert(split.slice.data == NULL);
    
    slice = split_next(&split);
    assert(slice.len == 0);
    assert(slice.data == NULL);
    assert(split.slice.len == 0);
    assert(split.slice.data == NULL);
}

void split_next_test_3() {
    char *data, *sep;
    int data_len, sep_len;
    split_t split;
    slice_t slice;

    data = "\n\n";
    data_len = 2;
    sep = "\n";
    sep_len = 1;

    split = split_new(slice_new(data, data_len), slice_new(sep, sep_len));
    
    slice = split_next(&split);
    assert(slice.len == 0);
    assert(slice.data == data);
    assert(split.slice.len == 1);
    assert(split.slice.data == data + 1);
    
    slice = split_next(&split);
    assert(slice.len == 0);
    assert(slice.data == data + 1);
    assert(split.slice.len == 0);
    assert(split.slice.data == data + 2);
    
    slice = split_next(&split);
    assert(slice.len == 0);
    assert(slice.data == data + 2);
    assert(split.slice.len == 0);
    assert(split.slice.data == NULL);
    
    slice = split_next(&split);
    assert(slice.len == 0);
    assert(slice.data == 0);
    assert(split.slice.len == 0);
    assert(split.slice.data == 0);
}

void split_next_test_4() {
    char *data, *sep;
    int data_len, sep_len;
    split_t split;
    slice_t slice;

    data = "\n\n\n";
    data_len = 3;
    sep = "\n";
    sep_len = 1;

    split = split_new(slice_new(data, data_len), slice_new(sep, sep_len));
    
    slice = split_next(&split);
    assert(slice.len == 0);
    assert(slice.data == data);
    assert(split.slice.len == 2);
    assert(split.slice.data == data + 1);
    
    slice = split_next(&split);
    assert(slice.len == 0);
    assert(slice.data == data + 1);
    assert(split.slice.len == 1);
    assert(split.slice.data == data + 2);
    
    slice = split_next(&split);
    assert(slice.len == 0);
    assert(slice.data == data + 2);
    assert(split.slice.len == 0);
    assert(split.slice.data == data + 3);

    slice = split_next(&split);
    assert(slice.len == 0);
    assert(slice.data == data + 3);
    assert(split.slice.len == 0);
    assert(split.slice.data == 0);
   
    slice = split_next(&split);
    assert(slice.len == 0);
    assert(slice.data == NULL);
    assert(split.slice.len == 0);
    assert(split.slice.data == NULL);
}

void split_next_test_5() {
    char *data, *sep;
    int data_len, sep_len;
    split_t split;
    slice_t slice;

    data = "a";
    data_len = 1;
    sep = "\n";
    sep_len = 1;

    split = split_new(slice_new(data, data_len), slice_new(sep, sep_len));
    
    slice = split_next(&split);
    assert(slice.len == 1);
    assert(slice.data == data);
    assert(split.slice.len == 0);
    assert(split.slice.data == 0);
    
    slice = split_next(&split);
    assert(slice.len == 0);
    assert(slice.data == NULL);
    assert(split.slice.len == 0);
    assert(split.slice.data == NULL);
}

void split_next_test_6() {
    char *data, *sep;
    int data_len, sep_len;
    split_t split;
    slice_t slice;

    data = "\na";
    data_len = 2;
    sep = "\n";
    sep_len = 1;

    split = split_new(slice_new(data, data_len), slice_new(sep, sep_len));
    
    slice = split_next(&split);
    assert(slice.len == 0);
    assert(slice.data == data);
    assert(split.slice.len == 1);
    assert(split.slice.data == data + 1);
    
    slice = split_next(&split);
    assert(slice.len == 1);
    assert(slice.data == data + 1);
    assert(split.slice.len == 0);
    assert(split.slice.data == 0);

    slice = split_next(&split);
    assert(slice.len == 0);
    assert(slice.data == NULL);
    assert(split.slice.len == 0);
    assert(split.slice.data == NULL);
}

void split_next_test_7() {
    char *data, *sep;
    int data_len, sep_len;
    split_t split;
    slice_t slice;

    data = "a\n";
    data_len = 2;
    sep = "\n";
    sep_len = 1;

    split = split_new(slice_new(data, data_len), slice_new(sep, sep_len));
    
    slice = split_next(&split);
    assert(slice.len == 1);
    assert(slice.data == data);
    assert(split.slice.len == 0);
    assert(split.slice.data == data + 2);
    
    slice = split_next(&split);
    assert(slice.len == 0);
    assert(slice.data == data + 2);
    assert(split.slice.len == 0);
    assert(split.slice.data == 0);

    slice = split_next(&split);
    assert(slice.len == 0);
    assert(slice.data == NULL);
    assert(split.slice.len == 0);
    assert(split.slice.data == NULL);
}

void split_next_test_8() {
    char *data, *sep;
    int data_len, sep_len;
    split_t split;
    slice_t slice;

    data = "\na\n";
    data_len = 3;
    sep = "\n";
    sep_len = 1;

    split = split_new(slice_new(data, data_len), slice_new(sep, sep_len));
    
    slice = split_next(&split);
    assert(slice.len == 0);
    assert(slice.data == data);
    assert(split.slice.len == 2);
    assert(split.slice.data == data + 1);
    
    slice = split_next(&split);
    assert(slice.len == 1);
    assert(slice.data == data + 1);
    assert(split.slice.len == 0);
    assert(split.slice.data == data + 3);

    slice = split_next(&split);
    assert(slice.len == 0);
    assert(slice.data == data + 3);
    assert(split.slice.len == 0);
    assert(split.slice.data == 0);

    slice = split_next(&split);
    assert(slice.len == 0);
    assert(slice.data == NULL);
    assert(split.slice.len == 0);
    assert(split.slice.data == NULL);
}

void split_next_test_9() {
    char *data, *sep;
    int data_len, sep_len;
    split_t split;
    slice_t slice;

    data = "\nab\nc\nde\n";
    data_len = 9;
    sep = "\n";
    sep_len = 1;

    split = split_new(slice_new(data, data_len), slice_new(sep, sep_len));

    slice = split_next(&split);
    assert(slice.len == 0);
    assert(slice.data == data);
    assert(split.slice.len == 8);
    assert(split.slice.data == data + 1);

    slice = split_next(&split);
    assert(slice.len == 2);
    assert(slice.data == data + 1);
    assert(split.slice.len == 5);
    assert(split.slice.data == data + 4);

    slice = split_next(&split);
    assert(slice.len == 1);
    assert(slice.data == data + 4);
    assert(split.slice.len == 3);
    assert(split.slice.data == data + 6);

    slice = split_next(&split);
    assert(slice.len == 2);
    assert(slice.data == data + 6);
    assert(split.slice.len == 0);
    assert(split.slice.data == data + 9);

    slice = split_next(&split);
    assert(slice.len == 0);
    assert(slice.data == data + 9);
    assert(split.slice.len == 0);
    assert(split.slice.data == NULL);

    slice = split_next(&split);
    assert(slice.len == 0);
    assert(slice.data == NULL);
    assert(split.slice.len == 0);
    assert(split.slice.data == NULL);
}

void split_next_test_helper(char *data, char **want) {
    split_t split;
    slice_t slice;

    split = split_new(slice_new(data, strlen(data)), slice_new("\n", 1));

    while (*want) {
        slice = split_next(&split);
        assert(slice.len == strlen(*want));
        assert(slice.data != NULL);
        assert(strncmp(slice.data, *want, slice.len) == 0);
        want++;
    }

    slice = split_next(&split);
    assert(slice.len == 0);
    assert(slice.data == NULL);
    assert(split.slice.len == 0);
    assert(split.slice.data == NULL);
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
}
