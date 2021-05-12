#include <stdio.h>
#include <assert.h>

typedef void* QUEUE[2];

static void test_array() {
	int a[10] = {0};

	printf("a=%p, &a=%p\n", a, &a);
	assert((int)a == (int)&a);
	printf("a+1=%p, &a+1=%p\n", a+1, &a+1);
	assert((int)(&a+1) - (int)(a+1) == 9 * sizeof(int));
}

static void test_queue1() {	
	QUEUE q = {0};

	printf("q=%p, &q=%p\n", q, &q);
	assert((int)q == (int)&q);
	printf("q+1=%p, &q+1=%p\n", q+1, &q+1);
	assert((int)(&q+1) - (int)(q+1) == 1 * sizeof(void*));
}

void test_queue() {
	QUEUE q = {0};
	QUEUE *qp = &q;
	int i, j;

	test_array();
	test_queue1();
	
	q[0] = &i;
	q[1] = &j;

	assert(qp[0][0] == q[0]);
	assert(qp[0][1] == q[1]);

	// qp[1] is invalid
}
