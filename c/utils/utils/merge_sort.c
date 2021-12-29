#include <assert.h>

#define ARRAY_SIZE(arr) (sizeof(arr) / sizeof(arr[0]))

void merge(int *a, int begin, int middle, int end, int *b) {
    int i, j, k;

    i = begin, j = middle;
    for (k = begin; k <= end; k++) {
        if (i < middle && (j > end || a[i] <= a[j])) {
            b[k] = a[i++];
        } else {
            b[k] = a[j++];
        }
    }

    for (k = begin; k <= end; k++) {
        a[k] = b[k];
    }
}

void merge_sort(int *a, int begin, int end, int *b) {
    int middle;
    
    if (end <= begin) return;

    middle = begin + (end + 1 - begin) / 2;

    merge_sort(a, begin, middle - 1, b);
    merge_sort(a, middle, end, b);
    merge(a, begin, middle, end, b);
}

void test_merge() {
    int a[] = {3, 7, 9, 0, 1, 2, 4, 5, 6, 8};
    int b[ARRAY_SIZE(a)];
    int i;

    merge(a, 0, 3, ARRAY_SIZE(a) - 1, b);
    
    for (i = 0; i < ARRAY_SIZE(a); i++)
    {
        assert(a[i] == i);
    }
}

void test_merge_sort_1() {
    int a[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
    int b[ARRAY_SIZE(a)];
    int i;

    merge_sort(a, 0, ARRAY_SIZE(a) - 1, b);
    
    for (i = 0; i < ARRAY_SIZE(a); i++)
    {
        assert(a[i] == i);
    }
}

void test_merge_sort_2() {
    int a[] = {9, 8, 7, 6, 5, 4, 3, 2, 1, 0};
    int b[ARRAY_SIZE(a)];
    int i;

    merge_sort(a, 0, ARRAY_SIZE(a) - 1, b);
    
    for (i = 0; i < ARRAY_SIZE(a); i++)
    {
        assert(a[i] == i);
    }
}

void test_merge_sort_3() {
    int a[] = {3, 4, 2, 1, 7, 5, 8, 9, 0, 6};
    int b[ARRAY_SIZE(a)];
    int i;

    merge_sort(a, 0, ARRAY_SIZE(a) - 1, b);
    
    for (i = 0; i < ARRAY_SIZE(a); i++)
    {
        assert(a[i] == i);
    }
}

void test_merge_sort() {
    test_merge();
    test_merge_sort_1();
    test_merge_sort_2();
    test_merge_sort_3();
}
