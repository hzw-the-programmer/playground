#include <assert.h>

#define ARRAY_SIZE(arr) (sizeof(arr) / sizeof(arr[0]))

#if 1
void merge(int *temp, int begin, int middle, int end, int *arr) {
    int i, j, k;

    for (i = begin, j = begin, k = middle; i < end; i++) {
        if (j < middle && (k >= end || temp[j] < temp[k])) {
            arr[i] = temp[j++];
        } else {
            arr[i] = temp[k++];
        }
    }
}

void copy(int *arr, int begin, int end, int *temp) {
    int i;

    for (i = begin; i < end; i++) {
        temp[i] = arr[i];
    }
}

void split_merge(int *temp, int begin, int end, int *arr) {
    int middle;
    
    if (end - begin <= 1) return;

    middle = (end + begin) >> 1;

    split_merge(arr, begin, middle, temp);
    split_merge(arr, middle, end, temp);

    merge(temp, begin, middle, end, arr);
}

void merge_sort(int *arr, int begin, int end, int *temp) {
    copy(arr, begin, end, temp);
    split_merge(temp, begin, end, arr);
}
#else
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
#endif

void test_merge() {
#if 1
    int temp[] = {3, 7, 9, 0, 1, 2, 4, 5, 6, 8};
    int target[ARRAY_SIZE(temp)];
    int i;

    merge(temp, 0, 3, ARRAY_SIZE(temp), target);

    for (i = 0; i < ARRAY_SIZE(target); i++)
    {
        assert(target[i] == i);
    }
#else
    int a[] = {3, 7, 9, 0, 1, 2, 4, 5, 6, 8};
    int b[ARRAY_SIZE(a)];
    int i;

    merge(a, 0, 3, ARRAY_SIZE(a) - 1, b);
    
    for (i = 0; i < ARRAY_SIZE(a); i++)
    {
        assert(a[i] == i);
    }
#endif
}

void test_copy() {
    int target[] = {3, 7, 9, 0, 1, 2, 4, 5, 6, 8};
    int temp[ARRAY_SIZE(target)];
    int i;

    copy(target, 0, ARRAY_SIZE(target), temp);

    for (i = 0; i < ARRAY_SIZE(target); i++)
    {
        assert(target[i] == temp[i]);
    }
}

void test_merge_sort_1() {
#if 1
    int a[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
    int b[ARRAY_SIZE(a)];
    int i;

    merge_sort(a, 0, ARRAY_SIZE(a), b);
    
    for (i = 0; i < ARRAY_SIZE(a); i++)
    {
        assert(a[i] == i);
    }
#else
    int a[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
    int b[ARRAY_SIZE(a)];
    int i;

    merge_sort(a, 0, ARRAY_SIZE(a) - 1, b);
    
    for (i = 0; i < ARRAY_SIZE(a); i++)
    {
        assert(a[i] == i);
    }
#endif
}

void test_merge_sort_2() {
#if 1
    int a[] = {9, 8, 7, 6, 5, 4, 3, 2, 1, 0};
    int b[ARRAY_SIZE(a)];
    int i;

    merge_sort(a, 0, ARRAY_SIZE(a), b);
    
    for (i = 0; i < ARRAY_SIZE(a); i++)
    {
        assert(a[i] == i);
    }
#else
    int a[] = {9, 8, 7, 6, 5, 4, 3, 2, 1, 0};
    int b[ARRAY_SIZE(a)];
    int i;

    merge_sort(a, 0, ARRAY_SIZE(a) - 1, b);
    
    for (i = 0; i < ARRAY_SIZE(a); i++)
    {
        assert(a[i] == i);
    }
#endif
}

void test_merge_sort_3() {
#if 1
    int a[] = {3, 4, 2, 1, 7, 5, 8, 9, 0, 6};
    int b[ARRAY_SIZE(a)];
    int i;

    merge_sort(a, 0, ARRAY_SIZE(a), b);
    
    for (i = 0; i < ARRAY_SIZE(a); i++)
    {
        assert(a[i] == i);
    }
#else
    int a[] = {3, 4, 2, 1, 7, 5, 8, 9, 0, 6};
    int b[ARRAY_SIZE(a)];
    int i;

    merge_sort(a, 0, ARRAY_SIZE(a) - 1, b);
    
    for (i = 0; i < ARRAY_SIZE(a); i++)
    {
        assert(a[i] == i);
    }
#endif
}

void test_merge_sort() {
    test_merge();
    test_copy();
    test_merge_sort_1();
    test_merge_sort_2();
    test_merge_sort_3();
}
