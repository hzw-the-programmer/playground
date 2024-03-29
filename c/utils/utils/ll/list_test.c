#include <assert.h>
#include <string.h>
#include "list.h"
#include "../utils.h"

#define HLEN 100

typedef struct {
    int len;
    char h[HLEN];
} list_ctx_t;

static int list_ctx_len(void *data) {
    list_ctx_t *ctx = data;
    return ctx->len;
}

static void list_ctx_measure(
    void *data, int index,
    bool focused, const rect_t *rect,
    int *w, int *h) {
    
    list_ctx_t *ctx = data;
    
    *w = rect->w;
    *h = ctx->h[index];
}

void test_list_measure_layout_helper(
    list_ctx_t *ctx,
    const rect_t *rect,
    rect_t *rects, int len, rect_t *wanted) {
    
    list_t list;
    list_adapter_t adapter = {0};
    int i;

    adapter.data = ctx;
    adapter.len = list_ctx_len;
    adapter.measure = list_ctx_measure;
    list = list_new(rect, rects, len, &adapter);

    list_measure(&list);
    list_layout(&list);

    for (i = 0; i < list.len; i++) {
        assert(list.rects[i].x == wanted[i].x);
        assert(list.rects[i].y == wanted[i].y);
        assert(list.rects[i].w == wanted[i].w);
        assert(list.rects[i].h == wanted[i].h);
    }
}

void test_list_measure_layout_1() {
    list_ctx_t ctx = {0, {10, 10, 10}};
    rect_t rect = {0, 0, 60, 30};
    rect_t rects[3] = {0};

    rect_t wanted[] = {
        {0, 0, 60, 10},
        {0, 10, 60, 10},
        {0, 20, 60, 10},
    };

    test_list_measure_layout_helper(&ctx, &rect, rects, ARRAY_SIZE(rects), wanted);
}

void test_list_measure_layout_2() {
    list_ctx_t ctx = {0, {11, 11, 11}};
    rect_t rect = {0, 0, 60, 30};
    rect_t rects[3] = {0};

    rect_t wanted[] = {
        {0, 0, 60, 11},
        {0, 11, 60, 11},
        {0, 22, 60, 11},
    };

    test_list_measure_layout_helper(&ctx, &rect, rects, ARRAY_SIZE(rects), wanted);
}

void test_list_measure_layout_3() {
    list_ctx_t ctx = {0, {12, 9, 13}};
    rect_t rect = {0, 0, 60, 30};
    rect_t rects[3] = {0};

    rect_t wanted[] = {
        {0, 0, 60, 12},
        {0, 12, 60, 10},
        {0, 22, 60, 13},
    };
    
    test_list_measure_layout_helper(&ctx, &rect, rects, ARRAY_SIZE(rects), wanted);
}

void test_list_down() {
    rect_t rect = {0, 0, 60, 30};
    list_ctx_t ctx = {7, {10, 11, 12, 13, 14, 15, 16}};
    rect_t rects[3];
    list_adapter_t adapter = {0};
    list_t list;

    struct {
        int start;
        int current;
        rect_t rects[3];
    } wanted[] = {
        {
            0,
            0,
            {
                {0, 0, 60, 10},
                {0, 10, 60, 11},
                {0, 21, 60, 12},
            },
        },
        {
            0,
            1,
            {
                {0, 0, 60, 10},
                {0, 10, 60, 11},
                {0, 21, 60, 12},
            },
        },
        {
            0,
            2,
            {
                {0, -3, 60, 10},
                {0, 7, 60, 11},
                {0, 18, 60, 12},
            },
        },
        {
            1,
            3,
            {
                {0, -6, 60, 11},
                {0, 5, 60, 12},
                {0, 17, 60, 13},
            },
        },
        {
            2,
            4,
            {
                {0, -9, 60, 12},
                {0, 3, 60, 13},
                {0, 16, 60, 14},
            },
        },
        {
            3,
            5,
            {
                {0, -12, 60, 13},
                {0, 1, 60, 14},
                {0, 15, 60, 15},
            },
        },
        {
            4,
            6,
            {
                {0, -15, 60, 14},
                {0, -1, 60, 15},
                {0, 14, 60, 16},
            },
        },
        {
            0,
            0,
            {
                {0, 0, 60, 10},
                {0, 10, 60, 11},
                {0, 21, 60, 12},
            },
        },
    };

    int i, j;

    adapter.data = &ctx;
    adapter.len = list_ctx_len;
    adapter.measure = list_ctx_measure;
    list = list_new(&rect, rects, ARRAY_SIZE(rects), &adapter);

    list_measure(&list);
    list_layout(&list);

    for (i = 0; i < ARRAY_SIZE(wanted); i++) {
        for (j = 0; j < list.len; j++) {
            assert(list.start == wanted[i].start);
            assert(list.current == wanted[i].current);
            assert(list.rects[j].x == wanted[i].rects[j].x);
            assert(list.rects[j].y == wanted[i].rects[j].y);
            assert(list.rects[j].w == wanted[i].rects[j].w);
            assert(list.rects[j].h == wanted[i].rects[j].h);
        }

        list_down(&list);
    }
}

void test_list_up() {
    rect_t rect = {0, 0, 60, 30};
    rect_t rects[3];
    list_ctx_t ctx = {7, {10, 11, 12, 13, 14, 15, 16}};
    list_adapter_t adapter;
    list_t list;

    rect_t wanted[][ARRAY_SIZE(rects)] = {
        {
            {0, -15, 60, 14},
            {0, -1, 60, 15},
            {0, 14, 60, 16},
        },
        {
            {0, -14, 60, 14},
            {0, 0, 60, 15},
            {0, 15, 60, 16},
        },
        {
            {0, 0, 60, 14},
            {0, 14, 60, 15},
            {0, 29, 60, 16},
        },
        {
            {0, 0, 60, 13},
            {0, 13, 60, 14},
            {0, 27, 60, 15},
        },
        {
            {0, 0, 60, 12},
            {0, 12, 60, 13},
            {0, 25, 60, 14},
        },
        {
            {0, 0, 60, 11},
            {0, 11, 60, 12},
            {0, 23, 60, 13},
        },
        {
            {0, 0, 60, 10},
            {0, 10, 60, 11},
            {0, 21, 60, 12},
        },
        {
            {0, -15, 60, 14},
            {0, -1, 60, 15},
            {0, 14, 60, 16},
        },
    };

    int i, j;

    adapter.data = &ctx;
    adapter.len = list_ctx_len;
    adapter.measure = list_ctx_measure;
    
    list.rect = rect;

    list.rects = rects;
    list.len = ARRAY_SIZE(rects);

    list.adapter = adapter;

    list.offset = 0;

    list_jump_to_bottom(&list);
    
    for (i = 0; i < ARRAY_SIZE(wanted); i++) {
        for (j = 0; j < list.len; j++) {
            assert(list.rects[j].x == wanted[i][j].x);
            assert(list.rects[j].y == wanted[i][j].y);
            assert(list.rects[j].w == wanted[i][j].w);
            assert(list.rects[j].h == wanted[i][j].h);
        }

        list_up(&list);
    }
}

void test_list() {
    test_list_measure_layout_1();
    test_list_measure_layout_2();
    test_list_measure_layout_3();
    test_list_down();
    test_list_up();
}
