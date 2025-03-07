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
extern "C" void test_mw();
extern "C" void test_buffer2();
void test_varints();
void test_vwsnprintf();
void test_vsnprintf();
void test_preprocessor();
void test_param();
void test_shift();
void test_align();
extern "C" void test_log();
void test_linear_layout();
extern "C" void test_insertion_sort();
extern "C" void test_input();
extern "C" void test_merge_sort();
extern "C" void test_socket();
extern "C" void test_buf3();
extern "C" void test_array2d();
extern "C" void test_macro();
extern "C" void test_pointer();
extern "C" void test_history();
extern "C" void macro_test();
extern "C" void ll_test();
extern "C" void slice_test();
extern "C" void split_test();
extern "C" void buffer_test();
extern "C" void grid_test();
extern "C" void scroll_bar_test();
extern "C" void canvas_test();
extern "C" void test_list();
#include "./zlib/zlib_test.h"
#include "./mini_gzip/mini_gzip_test.h"
#include "sha256/sha256_test.h"

#include "mbedtls/bignum.h"
#include "mbedtls/entropy.h"
#include "mbedtls/ctr_drbg.h"
#include "mbedtls/rsa.h"

extern "C" {
void test_msg();
void test_msg2();
void test_queue_1();
void test_heap();
void test_socket();
}

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
    test_mw();
    test_buffer2();
    test_varints();
    test_vwsnprintf();
    test_vsnprintf();
    test_preprocessor();
    test_param();
    test_shift();
    test_align();
    test_log();
    test_linear_layout();
    test_insertion_sort();
    test_input();
    test_merge_sort();
    test_socket();
    test_buf3();
    test_array2d();
    //test_macro();
    test_pointer();
    test_history();
    macro_test();
    ll_test();
    slice_test();
    split_test();
    buffer_test();
    grid_test();
    scroll_bar_test();
    canvas_test();
    test_list();
    test_zlib();
    test_mini_gzip();
    test_sha256();
    mbedtls_rsa_self_test(1);

#if defined(MBEDTLS_SELF_TEST)
    mbedtls_mpi_self_test(1);
    mbedtls_entropy_self_test(1);
    mbedtls_ctr_drbg_self_test(1);
#endif

    test_msg();
    test_msg2();
    test_queue_1();
    test_heap();
    test_socket();

	MemoryCheck();

	scanf("%d", &i);
}
