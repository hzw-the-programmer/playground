#include <stdlib.h>
#include <string.h>
#include <assert.h>

#include "memory.h"

typedef struct {
    int index;
    int len;
    int cap;
    //char buf[1];
    char buf[4];
} Input;

Input* input_new(int cap) {
    Input *input;
    int size;

    size = sizeof(Input) + (cap - 4);
    input = HZW_MALLOC(size);
    memset(input, 0, size);
    input->cap = cap;

    return input;
}

void input_destroy(Input *input) {
    HzwFree(input);
}

int input_empty(Input *input) {
    return input->len == 0;
}

int input_full(Input *input) {
    return input->len == input->cap - 1;
}

int input_start(Input *input) {
    return input->index == 0;
}

int input_end(Input *input) {
    return input->index == input->len;
}

void input_insert(Input *input, char c) {
    if (input_full(input)) {
        return;
    }

    if (input->index < input->len) {
        memmove(input->buf + input->index + 1, input->buf + input->index, input->len - input->index);
    }

    input->buf[input->index++] = c;
    input->len++;
    input->buf[input->len] = 0;
}

void input_delete(Input *input) {
    if (input_start(input)) {
        return;
    }

    if (input->index < input->len) {
        memmove(input->buf + input->index - 1, input->buf + input->index, input->len - input->index);
    }

    input->index--;
    input->len--;
    input->buf[input->len] = 0;
}

void input_clear(Input *input) {
    int cap;
    int size;

    cap = input->cap;
    size = sizeof(Input) + (cap - 4);
    memset(input, 0, size);
    input->cap = cap;
}

void input_forward(Input *input) {
    if (input_end(input)) {
        return;
    }

    input->index++;
}

void input_forward_times(Input *input, int times) {
    int i;

    for (i = 0; i < times; i++) {
        input_forward(input);
    }
}

void input_backward(Input *input) {
    if (input_start(input)) {
        return;
    }

    input->index--;
}

void input_backward_times(Input *input, int times) {
    int i;

    for (i = 0; i < times; i++) {
        input_backward(input);
    }
}

void test_input_1() {
    Input *input;
    int cap = 10 + 1;

    input = input_new(cap);
    assert(input->index == 0);
    assert(input->len == 0);
    assert(input->cap == cap);

    memset(input->buf, 'h', cap);

    input_insert(input, '0');
    assert(strcmp(input->buf, "0") == 0);
    input_insert(input, '1');
    assert(strcmp(input->buf, "01") == 0);
    input_insert(input, '2');
    assert(strcmp(input->buf, "012") == 0);
    input_insert(input, '3');
    assert(strcmp(input->buf, "0123") == 0);
    input_insert(input, '4');
    assert(strcmp(input->buf, "01234") == 0);
    input_insert(input, '5');
    assert(strcmp(input->buf, "012345") == 0);
    input_insert(input, '6');
    assert(strcmp(input->buf, "0123456") == 0);
    input_insert(input, '7');
    assert(strcmp(input->buf, "01234567") == 0);
    input_insert(input, '8');
    assert(strcmp(input->buf, "012345678") == 0);
    input_insert(input, '9');
    assert(strcmp(input->buf, "0123456789") == 0);
    input_insert(input, '9');
    assert(strcmp(input->buf, "0123456789") == 0);

    input_delete(input);
    assert(strcmp(input->buf, "012345678") == 0);
    input_delete(input);
    assert(strcmp(input->buf, "01234567") == 0);
    input_delete(input);
    assert(strcmp(input->buf, "0123456") == 0);
    input_delete(input);
    assert(strcmp(input->buf, "012345") == 0);
    input_delete(input);
    assert(strcmp(input->buf, "01234") == 0);
    input_delete(input);
    assert(strcmp(input->buf, "0123") == 0);
    input_delete(input);
    assert(strcmp(input->buf, "012") == 0);
    input_delete(input);
    assert(strcmp(input->buf, "01") == 0);
    input_delete(input);
    assert(strcmp(input->buf, "0") == 0);
    input_delete(input);
    assert(strcmp(input->buf, "") == 0);
    input_delete(input);
    assert(strcmp(input->buf, "") == 0);

    input_destroy(input);
}

void test_input_2() {
    Input *input;
    int cap = 10 + 1;
    int i;

    input = input_new(cap);
    assert(input->index == 0);
    assert(input->len == 0);
    assert(input->cap == cap);

    for (i = 0; i < cap; i++) {
        input_insert(input, '0' + i);
    }

    assert(strcmp(input->buf, "0123456789") == 0);

    input_backward_times(input, 3);
    input_delete(input);
    assert(strcmp(input->buf, "012345789") == 0);

    input_backward_times(input, 2);
    input_delete(input);
    assert(strcmp(input->buf, "01245789") == 0);

    input_backward_times(input, 4);
    assert(input->index == 0);

    input_forward_times(input, 3);
    input_insert(input, '3');
    assert(strcmp(input->buf, "012345789") == 0);

    input_forward_times(input, 2);
    input_insert(input, '6');
    assert(strcmp(input->buf, "0123456789") == 0);

    input_insert(input, '6');
    assert(strcmp(input->buf, "0123456789") == 0);

    input_clear(input);
    assert(input->index == 0);
    assert(input->len == 0);
    assert(input->cap == cap);

    input_destroy(input);
}

void test_input_3() {
    Input *input;
    int cap = 10 + 1;
    int i;

    input = input_new(cap);
    assert(input_start(input) == 1);
    assert(input_empty(input) == 1);
    assert(input_end(input) == 1);
    assert(input_full(input) == 0);

    for (i = 0; i < cap; i++) {
        input_insert(input, '0' + i);
    }

    assert(input_start(input) == 0);
    assert(input_empty(input) == 0);
    assert(input_end(input) == 1);
    assert(input_full(input) == 1);

    input_backward_times(input, cap);
    assert(input_start(input) == 1);
    assert(input_empty(input) == 0);
    assert(input_end(input) == 0);
    assert(input_full(input) == 1);

    input_forward_times(input, 3);
    assert(input_start(input) == 0);
    assert(input_empty(input) == 0);
    assert(input_end(input) == 0);
    assert(input_full(input) == 1);

    input_delete(input);

    assert(input_start(input) == 0);
    assert(input_empty(input) == 0);
    assert(input_end(input) == 0);
    assert(input_full(input) == 0);
}

void test_input_4() {
    Input *input;
    int cap = 10 + 1;
    int i, j;

    input = input_new(cap + 2);
    assert(input->index == 0);
    assert(input->len == 0);
    assert(input->cap == cap + 2);

    input->buf[cap] = 'B';
    input->buf[cap + 1] = 'J';
    input->cap = cap;

    for (i = 0; i < 10; i++) {
        for (j = 0; j < cap; j++) {
            input_insert(input, '0' + j);
        }
    
        assert(strcmp(input->buf, "0123456789") == 0);

        input_backward_times(input, 3);
        input_delete(input);
        assert(strcmp(input->buf, "012345789") == 0);

        input_backward_times(input, 2);
        input_delete(input);
        assert(strcmp(input->buf, "01245789") == 0);

        input_backward_times(input, 4);
        assert(input->index == 0);

        input_forward_times(input, 3);
        input_insert(input, '3');
        assert(strcmp(input->buf, "012345789") == 0);

        input_forward_times(input, 2);
        input_insert(input, '6');
        assert(strcmp(input->buf, "0123456789") == 0);

        input_clear(input);
        assert(input->index == 0);
        assert(input->len == 0);
        assert(input->cap == cap);

        assert(input->buf[cap] == 'B');
        assert(input->buf[cap + 1] == 'J');
    }

    input_destroy(input);
}

void test_input() {
    test_input_1();
    test_input_2();
    test_input_3();
    test_input_4();
}
