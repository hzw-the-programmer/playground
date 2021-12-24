#include <assert.h>

#define ARRAY_SIZE(arr) (sizeof(arr) / sizeof(arr[0]))

void insertion_sort(int *arr, int len) {
    int i, j, k;

    for (i = 1; i < len; i++) {
        k = arr[i];
        j = i - 1;
        while (j >= 0 && arr[j] > k) {
            arr[j + 1] = arr[j];
            j--;
        }
        arr[j + 1] = k;
    }
}

void assert_array_equal(int *a, int *b, int len) {
    int i;

    for (i = 0; i < len; i++) {
        assert(a[i] == b[i]);
    }
}

void test_insertion_sort() {
    int src[] = {8, 1, 6, 3, 4, 5, 2, 7, 0, 9};
    int want[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};

    insertion_sort(src, ARRAY_SIZE(src));
    assert_array_equal(src, want, ARRAY_SIZE(src));
}
