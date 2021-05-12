//http://www.strudel.org.uk/itoa/

char *itoa(int value, char *result, int base) {
	char *chars = "zyxwvutsrqponmlkjihgfedcba9876543210123456789abcdefghijklmnopqrstuvwxyz";
	int temp_value;
	char *start = result, *end = result, temp_char;

	if (base < 2 || base > 36) {
		*result = 0;
		return result;
	}

	do {
		temp_value = value;
		value /= base;
		*end++ = chars[35 + temp_value - value * base];
	} while (value);

	if (temp_value < 0) {
		*end++ = '-';
	}

	*end-- = 0;

	while (start < end) {
		temp_char = *start;
		*start++ = *end;
		*end-- = temp_char;
	}

	return result;
}

int main(int argc, char *args) {
	char buf[20] = {0};
	
	itoa(0, buf, 10);
	itoa(1, buf, 10);
	itoa(12, buf, 10);
	itoa(123, buf, 10);
	itoa(1234, buf, 10);
	itoa(12345, buf, 10);
	itoa(-1, buf, 10);
	itoa(-12, buf, 10);
	itoa(-123, buf, 10);
	itoa(-1234, buf, 10);
	itoa(-12345, buf, 10);

	itoa(0, buf, 16);
	itoa(1, buf, 16);
	itoa(12, buf, 16);
	itoa(123, buf, 16);
	itoa(1234, buf, 16);
	itoa(12345, buf, 16);
	itoa(-1, buf, 16);
	itoa(-12, buf, 16);
	itoa(-123, buf, 16);
	itoa(-1234, buf, 16);
	itoa(-12345, buf, 16);
}
