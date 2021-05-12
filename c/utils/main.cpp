#include <stdio.h>
#include "memory.h"

void test_itoa();
void test_utf8();
void test_buffer();
void test_hStrcat();
void test_range_insert();
void test_redux();
void test_ucs2();
void test_struct();
void test_ht();
void test_wsprintf();
void test_snprintf();
void test_wsnprintf();
void test_wsnprintwf();
void test_template();
void test_http_parser();
void test_struct2();
void test_file();
void test_utarray();
void testEmoji();
void testHex();
void testInt();
void testSll();
void test_http_util();
void test_queue();
void test_slice();
void test_hs();
void test_find_objects();
void test_buffer1();
void test_max();
void test_print_buf();
void test_c_enum();
void test_ctl();
void test_stack();
void test_activity();
void test_string();
void test_apply_gravity();
void test_bitfield();
void test_redux1();
void test_sizeof();
void testContact();
void test_offsetof();

int main(int argc, char *args) {
	int i;
	
	test_itoa();
	test_utf8();
	test_buffer();
	test_hStrcat();
	//test_range_insert();
	test_redux();
	test_ucs2();
	test_struct();
	test_ht();
	test_wsprintf();
	test_snprintf();
	test_wsnprintf();
	test_wsnprintwf();
	test_template();
	test_http_parser();
	test_struct2();
	test_file();
	test_utarray();
	testEmoji();
	testHex();
	testInt();
	testSll();
	test_http_util();
	test_queue();
	test_slice();
    test_hs();
    test_find_objects();
    test_buffer1();
    test_max();
    test_print_buf();
    test_c_enum();
    test_ctl();
    test_stack();
    test_activity();
    test_string();
    test_apply_gravity();
	test_bitfield();
    test_redux1();
    test_sizeof();
    testContact();
    test_offsetof();

	MemoryCheck();

	scanf("%d", &i);
}
