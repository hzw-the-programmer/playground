#include <assert.h>

#include "memory.h"

#define NULL 0

typedef struct item {
    int s, e;
} item;

bool is_digit(char c) {
    return c >= '0' && c <= '9';
}

int find_phones(const char * const str, item *items) {
    const char *p = NULL, *s = NULL;
    int num = 0, count = 0;

    p = str;
    while (*p) {
        if (!s && is_digit(*p)) {
            s = p;
        } else if (s && (!is_digit(*p) || !*(p + 1))) {
             if (is_digit(*p) && !*(p + 1)) {
                p++;
            }

            num = 7;

            if (p - s >= num) {
                count++;
                if (items) {
                    (*items).s = s - str;
                    (*items).e = p - str;
                    items++;
                }
            }

            if (!*p) break;

            s = NULL;
        }

        p++;
    }

    return count;
}

void find_phones_test_helper(char *str, char *results[], int len) {
    int count = 0;
    item *items = NULL;
    int i = 0;

    count = find_phones(str, NULL);
    assert(len == count);

    items = (item*)HZW_MALLOC((count + 1) * sizeof(items));
    items[count].s = 0xff;
    items[count].e = 0xff;

    count = find_phones(str, items);
    assert(len == count);

    for (i = 0; i < count; i++) {
        char *result = results[i];
        char *s = str + items[i].s;
        while (*result && *result == *s) {
            result++;
            s++;
        }
        assert(!*result);
        assert(s == str + items[i].e);
    }

    assert( items[count].s == 0xff);
    assert( items[count].e == 0xff);

    HzwFree(items);
}

void test_find_phones() {
    {
        char *str = "my phone is 12345678901. please contact me.";
        char *results[] = {
            "12345678901"
        };

        find_phones_test_helper(str, results, 1); 
    }

    {
        char *str = "m1y p12ho123ne is 12345678901. ple12345ase con123456tact me.";
        char *results[] = {
            "12345678901"
        };

        find_phones_test_helper(str, results, 1); 
    }

     {
        char *str = "m1y p12ho123ne is 1234567. ple12345ase con123456tact me.";
        char *results[] = {
            "1234567"
        };

        find_phones_test_helper(str, results, 1); 
    }

    {
        char *str = "m1y p12ho123ne is 123456. ple12345ase con123456tact me.";
        // char *results[] = {};

        find_phones_test_helper(str, NULL, 0); 
    }

    {
        char *str = "m1y p12ho123ne is +1234567. ple12345ase con123456tact me.";
        char *results[] = {
            "1234567"
        };

        find_phones_test_helper(str, results, 1); 
    }

    {
        char *str = "m1+1234+12345+123456y p12ho123n+1234567e is +1234567. ple12345ase con123456tact me.";
        char *results[] = {
            "1234567",
            "1234567",
        };

        find_phones_test_helper(str, results, 2); 
    }

    {
        char *str = "m1+1234+12345+123456y p12ho123n+1234567e is +1234567. ple12345ase c1234567on123456tact me.";
        char *results[] = {
            "1234567",
            "1234567",
            "1234567",
        };

        find_phones_test_helper(str, results, 3); 
    }

    {
        char *str = "m1+1234+12345+123456y p12ho123n+1234567e is +1234567. ple12345ase c1234567on123456tact 00123456789me.";
        char *results[] = {
            "1234567",
            "1234567",
            "1234567",
            "00123456789",
        };

        find_phones_test_helper(str, results, 4); 
    }

    {
        char *str = "m1+1234+12345+123456y p12ho123n+1234567e is +1234567. ple12345ase c1234567on123456tact 00123456789m+123456e+12+123+1234567+765432.";
        char *results[] = {
            "1234567",
            "1234567",
            "1234567",
            "00123456789",
            "1234567",
        };

        find_phones_test_helper(str, results, 5); 
    }

    {
        char *str = "m1+1234+12345+123456y p12ho123n+1234567e is +1234567. ple12345ase c1234567on123456tact 00123456789m+123456esdfwe+12++++123++1234567+765432.";
        char *results[] = {
            "1234567",
            "1234567",
            "1234567",
            "00123456789",
            "1234567",
        };

        find_phones_test_helper(str, results, 5); 
    }

    {
        char *str = "m1+1234+12345+123456y p12ho123n+1234567e is +1234567. ple12345ase c1234567on123456tact 00123456789m+123456esdfwe+12++++123++1234567+7654321";
        char *results[] = {
            "1234567",
            "1234567",
            "1234567",
            "00123456789",
            "1234567",
            "7654321",
        };

        find_phones_test_helper(str, results, 6); 
    }
}

void test_find_objects() {
    test_find_phones();
}
