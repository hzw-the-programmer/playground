```
typedef kal_uint8 a8[8];
typedef struct {
	a8 a;
} container;
void func(a8 a) {
	container c;

	memcpy(c.a, a, sizeof(a8));
}
static void test() {
	kal_int32 s = sizeof(a8);
    kal_int32 sc = sizeof(container);
    kal_int32 num = 0xFFFFFDF4;
	a8 a;
	container c;
	kal_int8 i;

	for (i =0; i < s; i++) {
		a[i] = i;
		c.a[i] = i;
	}
	func(a);
}
```
