```
#define APP_ID APPLIB_MEM_AP_ID_TEST1
#define APP_STR_ID STR_GLOBAL_1
#define APP_MEM_SIZE (500 * 1024)

static kal_bool app_prepare_asm();

typedef struct {
	void *asm_pool;
	kal_uint32 pool_size;
	KAL_ADM_ID adm_id;
	kal_uint32 app_id;
	kal_uint32 str_id;
} App;

static App apps[2];
static kal_uint8 current;

static void app_mem_release_cb() {
	kal_uint32 alloc_count;
	kal_uint8 pre = !current;
	App *app = &apps[pre];

	kal_prompt_trace(MOD_ABM, "app%d_mem_release_cb begin", pre);

	alloc_count = applib_mem_ap_get_alloc_count();
	kal_prompt_trace(MOD_ABM, "alloc_count=%d", alloc_count);

	if (app->adm_id != 0) {
		kal_adm_delete(app->adm_id);
		app->adm_id = 0;
	}

	if (app->asm_pool != NULL) {
		applib_mem_ap_free(app->asm_pool);
		app->asm_pool = NULL;
	}

	alloc_count = applib_mem_ap_get_alloc_count();
	kal_prompt_trace(MOD_ABM, "alloc_count=%d", alloc_count);

	applib_mem_ap_notify_stop_finished(app->app_id, KAL_TRUE);

	kal_prompt_trace(MOD_ABM, "app%d_mem_release_cb end", pre);
}

static void app_mem_release_success_cb() {
	kal_bool ret;

	kal_prompt_trace(MOD_ABM, "app%d_mem_release_success_cb begin", current);
	ret = app_prepare_asm();
	kal_prompt_trace(MOD_ABM, "app%d_mem_release_success_cb end: ret=%d", current, ret);
}

static kal_bool app_prepare_asm() {
	kal_uint32 alloc_count;
	App *app = &apps[current];

	kal_prompt_trace(MOD_ABM, "app%d_prepare_asm begin", current);

	applib_mem_ap_register(
		app->app_id,
		app->str_id,
		0,
		app_mem_release_cb);

	alloc_count = applib_mem_ap_get_alloc_count();
	kal_prompt_trace(MOD_ABM, "alloc_count=%d", alloc_count);

	app->asm_pool = applib_mem_ap_alloc(app->app_id, app->pool_size);

	alloc_count = applib_mem_ap_get_alloc_count();
	kal_prompt_trace(MOD_ABM, "alloc_count=%d", alloc_count);

	if (app->asm_pool == NULL) {
		mmi_frm_appmem_prompt_to_release_mem(
			app->app_id,
			0,
			app->pool_size,
			app_mem_release_success_cb);
		return KAL_FALSE;
	}

	app->adm_id = kal_adm_create(app->asm_pool, app->pool_size, NULL, KAL_FALSE);

	kal_prompt_trace(MOD_ABM, "app%d_prepare_asm end", current);

	return KAL_TRUE;
}

static void asm_test() {
	static kal_uint8 i;
	kal_bool ret;
	kal_uint8 j;

	for (j = 0; j < sizeof(apps) / sizeof(apps[0]); j++) {
		apps[j].app_id = APP_ID + j;
		apps[j].str_id = APP_STR_ID + j;
		apps[j].pool_size = (j + 1) * APP_MEM_SIZE;
	}

	current = i++ % 2 != 0;
	ret = app_prepare_asm();

	kal_prompt_trace(MOD_ABM, "app%d: ret=%d", current, ret);
}
```

```
app0_prepare_asm begin
alloc_count=1
alloc_count=2
app0_prepare_asm end
app0: ret=1

app1_prepare_asm begin
alloc_count=2
alloc_count=2
asm_core_v10_release_mem begin
asm_core_v10_release_mem next_idx=0, app_count=0
asm_core_v10_release_mem end
app1: ret=0

asm_core_v10_release_mem begin
asm_core_v10_release_mem next_idx=0, app_count=1
app0_mem_release_cb begin
alloc_count=1
alloc_count=0
asm_core_v10_release_mem begin
app1_mem_release_success_cb begin
app1_prepare_asm begin
alloc_count=0
alloc_count=1
app1_prepare_asm end
app1_mem_release_success_cb end: ret=1
app0_mem_release_cb end
```

```
app0_prepare_asm begin
alloc_count=2
alloc_count=2
asm_core_v10_release_mem begin
asm_core_v10_release_mem next_idx=0, app_count=0
asm_core_v10_release_mem end
app0: ret=0

asm_core_v10_release_mem begin
asm_core_v10_release_mem next_idx=0, app_count=1
app1_mem_release_cb begin
alloc_count=1
alloc_count=0
asm_core_v10_release_mem begin
app0_mem_release_success_cb begin
app0_prepare_asm begin
alloc_count=0
alloc_count=1
app0_prepare_asm end
app0_mem_release_success_cb end: ret=1
app1_mem_release_cb end
```
