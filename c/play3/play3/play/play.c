#include "play.h"

void array_param(int a[16]);

int main()
{
	int a[16] = { 0 };
	array_param(a);
	return 0;
}

void array_param(int a[16]) {
	a[0] = 1;
	a[1] = 2;
}