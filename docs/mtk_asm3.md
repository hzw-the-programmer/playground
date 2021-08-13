```
static void *pool;

void asked_to_free() {
	applib_mem_ap_free(pool);
	pool = NULL;
	applib_mem_ap_notify_stop_finished(APPLIB_MEM_AP_ID_LETSCHAT, KAL_TRUE);
}

void test_alloc() {
	kal_uint32 pool_size;
	kal_uint32 total_left_size;
	kal_uint32 total_left_size1;
	kal_uint32 max_alloc_size;

	pool_size = mmi_res_get_asm_pool_size();
	total_left_size = applib_mem_ap_get_total_left_size_int();	
	max_alloc_size = applib_mem_ap_get_max_alloc_size_int();
	total_left_size1 = mmi_frm_asm_get_total_left_size_r(APPLIB_MEM_AP_ID_LETSCHAT);

	applib_mem_ap_register(APPLIB_MEM_AP_ID_LETSCHAT,
		STR_GLOBAL_1,
		0,
		asked_to_free);

	pool = applib_mem_ap_alloc(APPLIB_MEM_AP_ID_LETSCHAT, total_left_size1);

	pool_size = mmi_res_get_asm_pool_size();
	total_left_size = applib_mem_ap_get_total_left_size_int();	
	max_alloc_size = applib_mem_ap_get_max_alloc_size_int();
	total_left_size1 = mmi_frm_asm_get_total_left_size_r(APPLIB_MEM_AP_ID_LETSCHAT);
}
```
