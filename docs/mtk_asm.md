```
#define APP_ID APPLIB_MEM_AP_ID_TEST1
#define APP_STR_ID STR_GLOBAL_1
#define APP_MEM_SIZE (400 * 1024)

typedef struct {
	KAL_ADM_ID adm_id;
	void *asm_pool;
} Context;

static Context ctx;

static void app_mem_release_cb() {
	kal_uint32 alloc_count;

	kal_prompt_trace(MOD_ABM, "app_mem_release_cb begin");

	alloc_count = applib_mem_ap_get_alloc_count();
	kal_prompt_trace(MOD_ABM, "alloc_count=%d", alloc_count);

	if (ctx.adm_id != 0) {
		kal_adm_delete(ctx.adm_id);
		ctx.adm_id = 0;
	}

	if (ctx.asm_pool != NULL) {
		applib_mem_ap_free(ctx.asm_pool);
		ctx.asm_pool = NULL;
	}

	alloc_count = applib_mem_ap_get_alloc_count();
	kal_prompt_trace(MOD_ABM, "alloc_count=%d", alloc_count);

	applib_mem_ap_notify_stop_finished(APP_ID, KAL_TRUE);

	kal_prompt_trace(MOD_ABM, "app_mem_release_cb end");
}

static void app_mem_release_success_cb() {
	kal_prompt_trace(MOD_ABM, "app_mem_release_success_cb");
}

static void asm_test() {
	kal_uint32 alloc_count;

	kal_prompt_trace(MOD_ABM, "asm_test begin");

	alloc_count = applib_mem_ap_get_alloc_count();
	kal_prompt_trace(MOD_ABM, "alloc_count=%d", alloc_count);

	applib_mem_ap_register(
		APP_ID,
		APP_STR_ID,
		0,
		app_mem_release_cb);

	alloc_count = applib_mem_ap_get_alloc_count();
	kal_prompt_trace(MOD_ABM, "alloc_count=%d", alloc_count);

	ctx.asm_pool = applib_mem_ap_alloc(APP_ID, APP_MEM_SIZE);

	alloc_count = applib_mem_ap_get_alloc_count();
	kal_prompt_trace(MOD_ABM, "alloc_count=%d", alloc_count);

	if (ctx.asm_pool == NULL) {
		mmi_frm_appmem_prompt_to_release_mem(
			APP_ID,
			0,
			APP_MEM_SIZE,
			app_mem_release_success_cb);
		return;
	}

	ctx.adm_id = kal_adm_create(ctx.asm_pool, APP_MEM_SIZE, NULL, KAL_FALSE);

	kal_prompt_trace(MOD_ABM, "asm_test end");
}
```
