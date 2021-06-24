#include <assert.h>
#include "utarray.h"

void test_utarray() {
	UT_array *a = NULL;
	UT_icd icd = {sizeof(int), NULL, NULL, NULL};
	
	utarray_new(a, &icd);
	
	for (int i = 0; i < 8; i++) {
		utarray_push_back(a, &i);
	}

	for (int i = 0; i < utarray_len(a); i++) {
		assert(i == *(int*)utarray_eltptr(a, i));
	}

	utarray_free(a);
}
