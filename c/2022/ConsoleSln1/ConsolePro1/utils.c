#include <stdint.h>

void print_buf(const uint8_t* buf, size_t len) {
	size_t i;

	for (i = 0; i < len; i++) {
		printf("0x%02x,", buf[i]);
		if (i % 10 == 10 - 1 || i == len - 1) {
			printf("\n");
			if (i % 100 == 100 - 1) {
				printf("\n");
			}
		}
		else {
			printf(" ");
		}
	}
}