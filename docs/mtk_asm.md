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

```
MoDIS.exe!app_mem_release_success_cb()  行1019	C
MoDIS.exe!mmi_frm_asmi_invoke_success_callback_internal(mmi_scenario_id scen_id=MMI_SCENARIO_ID_DEFAULT, void * arg=0x00000000)  行1219 + 0x16 字节	C
MoDIS.exe!control_app_screen(mmi_scenario_id noti_scen_id=MMI_SCENARIO_ID_DEFAULT, mmi_scen_attr_prio_enum scrn_priority=MMI_SCEN_SCRN_PRIO_MEDIUM, behavior=MMI_NOTI_SCRN_BEHA_DISPLAY, kal_bool (mmi_scenario_id, void *)* scrn_func_ptr=0x004fb4f0, void * arg=0x00000000, action=MMI_BEHAVIOR_SCRN_DEFER_FIRST, mmi_frm_nmgr_alert_struct * alert_info=0x00000000, mmi_event_notify_enum event_type=MMI_EVENT_LAUNCH_APP)  行4604 + 0xd 字节	C
MoDIS.exe!control_notification(unsigned char notification_type='', notification_info_struct * noti_info=0x0d89fbf8)  行5153 + 0x2e 字节	C
MoDIS.exe!mmi_frm_nmgr_notify_by_app(mmi_scenario_id scenario_id=MMI_SCENARIO_ID_DEFAULT, mmi_event_notify_enum type=MMI_EVENT_LAUNCH_APP, kal_bool (mmi_scenario_id, void * screen_entry=0x004fb4f0, void * arg=0x00000000)  行2201 + 0xb 字节	C
MoDIS.exe!mmi_frm_asmi_invoke_success_callback(unsigned short app_id=5417)  行1253 + 0x10 字节	C
MoDIS.exe!asm_core_v10_release_mem(applib_mem_ap_stop_type type=MEM_AP_STOP_TYPE_FORCE)  行3842 + 0xd 字节	C
MoDIS.exe!mmi_frm_appmem_stop_finished_handler(unsigned int app_id=5416, unsigned int string_id=2, kal_bool result=KAL_TRUE)  行1422 + 0x7 字节	C
MoDIS.exe!applib_mem_ap_notify_stop_finished(unsigned int app_id=5416, kal_bool result=KAL_TRUE)  行2728 + 0x15 字节	C
MoDIS.exe!app_mem_release_cb()  行1014 + 0xe 字节	C
MoDIS.exe!applib_mem_ap_stop_app_by_MMI_task(unsigned int app_id=5416, applib_mem_ap_stop_type stop_type=MEM_AP_STOP_TYPE_FORCE)  行2486 + 0x12 字节	C
MoDIS.exe!asm_core_v10_release_mem(applib_mem_ap_stop_type type=MEM_AP_STOP_TYPE_FORCE)  行3885 + 0x1c 字节	C
MoDIS.exe!asm_core_v10_launch_proc(_mmi_event_struct * evt=0x025f8940)  行3799 + 0x7 字节	C
MoDIS.exe!mmi_frm_invoke_post_event()  行1651 + 0x10 字节	C
MoDIS.exe!mmi_key_handle(mmi_key_evt_struct * mmi_evt_p=0x0d89fe20)  行4674	C
MoDIS.exe!dev_key_handle(dev_key_evt_struct * dev_evt_p=0x0d89fe40)  行4549 + 0x9 字节	C
MoDIS.exe!mmi_frm_key_handle(void * paraBuff=0x087b3fa4)  行3433 + 0x9 字节	C
MoDIS.exe!mmi_frm_execute_current_protocol_handler(unsigned short eventID=14829, void * MsgStruct=0x087b3fa4, int mod_src=25, void * Message=0x0d89fefc)  行682 + 0x11 字节	C
MoDIS.exe!MMI_task(task_entry_struct * entry_param=0x087ceee4)  行2745 + 0x16 字节	C
MoDIS.exe!_osc_platform_thread_create()  + 0x2a0 字节	
msvcr90d.dll!_callthreadstartex()  行348 + 0xf 字节	C
msvcr90d.dll!_threadstartex(void * ptd=0x0c9e3670)  行331	C
kernel32.dll!76c06359() 	
[下面的框架可能不正确和/或缺失，没有为 kernel32.dll 加载符号]	
ntdll.dll!770487a4() 	
ntdll.dll!77048774() 	
```
