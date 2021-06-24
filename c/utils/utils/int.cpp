#include <assert.h>

void testInt() {
	assert(2 == sizeof(short));
	assert(4 == sizeof(int));
	assert(4 == sizeof(long));
	assert(8 == sizeof(long long));

	{
		int i = 0x01020304;
		short s = (short)i;
		short *sp = (short*)&i;
		assert(0x0304 == s);
		assert(0x0304 == *sp);
	}
}
