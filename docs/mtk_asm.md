```
#define APP_ID APPLIB_MEM_AP_ID_TEST1
#define APP_STR_ID STR_GLOBAL_1
#define APP_MEM_SIZE (1.5 * 1024 * 1024)

#define HZW_GRP_ID
#define HZW_SCR_ID

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

static char* hzwEvtStr(U16 evtId) {
	switch (evtId) {
		case EVT_ID_POST_CB_RST:
			return "EVT_ID_POST_CB_RST";
    	case EVT_ID_POST_DEREG_CB:
			return "EVT_ID_POST_DEREG_CB";
    	case EVT_ID_SCENARIO_CHANGE:
			return "EVT_ID_SCENARIO_CHANGE";
    	case EVT_ID_GROUP_ENTER: /* Enter new group */
			return "EVT_ID_GROUP_ENTER";
    	case EVT_ID_GROUP_ACTIVE: /* active group */
			return "EVT_ID_GROUP_ACTIVE";
    	case EVT_ID_GROUP_INACTIVE: /* inactive group */
			return "EVT_ID_GROUP_INACTIVE";
    	case EVT_ID_GROUP_FOCUSED: /* focus a topest group */
			return "EVT_ID_GROUP_FOCUSED";
    	case EVT_ID_GROUP_DEFOCUSED: /* defocus a topest group */
			return "EVT_ID_GROUP_DEFOCUSED";
    	case EVT_ID_GROUP_GOBACK: /* close the active group; invoke the goback process:
                                   * We will send EVT_ID_GROUP_GOBACK to group proc first. 
                                   * send EVT_ID_SCRN_GOBACK/EVT_ID_GROUP_DELETE_REQ to the children (screens), 
                                   * and send EVT_ID_GROUP_INACTIVE to group proc.
                                   * Then finding the active group/screen and invoke the active process.
                                   * At last, we send EVT_ID_SCRN_DEINIT/EVT_ID_GROUP_DEINIT to all closed screens and groups.
                                   */
			return "EVT_ID_GROUP_GOBACK";
    	case EVT_ID_GROUP_GOBACK_IN_END_KEY: /* close the active group; invoke the goback process:
                                              * We will send EVT_ID_GROUP_GOBACK to group proc first. 
                                              * send EVT_ID_SCRN_GOBACK/EVT_ID_GROUP_DELETE_REQ to the children (screens), 
                                              * and send EVT_ID_GROUP_INACTIVE to group proc.
                                              * Then finding the active group/screen and invoke the active process.
                                              * At last, we send EVT_ID_SCRN_DEINIT/EVT_ID_GROUP_DEINIT to all closed screens and groups.
                                              */
			return "EVT_ID_GROUP_GOBACK_IN_END_KEY";
    	case EVT_ID_GROUP_DELETE_REQ: /* close the inactive group; invoke the delete process only:
                                       * We will send EVT_ID_GROUP_DELETE_REQ to group proc. 
                                       * send EVT_ID_SCRN_DELETE_REQ to the children (screens),
                                       * and send EVT_ID_SCRN_DEINIT/EVT_ID_GROUP_DEINIT to all closed screens and groups.
                                       * We will abort the delete process if group proc or screen leave_proc don't return MMI_RET_OK.
                                       */
			return "EVT_ID_GROUP_DELETE_REQ";
    	case EVT_ID_GROUP_DELETE_REQ_IN_END_KEY: /* close the inactive group; invoke the delete process only:
                                                  * We will send EVT_ID_GROUP_DELETE_REQ to group proc. 
                                                  * send EVT_ID_SCRN_DELETE_REQ to the children (screens),
                                                  * and send EVT_ID_SCRN_DEINIT/EVT_ID_GROUP_DEINIT to all closed screens and groups.
                                                  * We will abort the delete process if group proc or screen leave_proc don't return MMI_RET_OK.
                                                  */
			return "EVT_ID_GROUP_DELETE_REQ_IN_END_KEY";
    	case EVT_ID_GROUP_EXIT: /* Exit the group */
			return "EVT_ID_GROUP_EXIT";
    	case EVT_ID_GROUP_DEINIT: /* deinint group */
			return "EVT_ID_GROUP_DEINIT";
    	case EVT_ID_GROUP_REDRAW_START: /* Notificaiton for start to redraw a group */
			return "EVT_ID_GROUP_REDRAW_START";
    	case EVT_ID_GROUP_REDRAW_DONE: /* Notification for finish the group redrawing */
			return "EVT_ID_GROUP_REDRAW_DONE";
    	case EVT_ID_SCRN_INIT:
			return "EVT_ID_SCRN_INIT";
    	case EVT_ID_SCRN_ACTIVE:
			return "EVT_ID_SCRN_ACTIVE";
    	case EVT_ID_SCRN_INACTIVE:
			return "EVT_ID_SCRN_INACTIVE";
    	case EVT_ID_SCRN_GOBACK: /* close the active screen; invoke the goback process: 
                                  * We will send EVT_ID_SCRN_GOBACK to screen leave_proc and then executing the exit_proc.
                                  * At last, we send EVT_ID_SCRN_DEINIT active screen leave_proc.
                                  */
			return "EVT_ID_SCRN_GOBACK";
    	case EVT_ID_SCRN_GOBACK_IN_END_KEY: /* close the active screen; invoke the goback process: 
                                             * We will send EVT_ID_SCRN_GOBACK to screen leave_proc and then executing the exit_proc.
                                             * At last, we send EVT_ID_SCRN_DEINIT active screen leave_proc.
                                             */
			return "EVT_ID_SCRN_GOBACK_IN_END_KEY";
    	case EVT_ID_SCRN_DELETE_REQ: /* delete the inactive screen; invoke the delete process only:
                                      * We will send EVT_ID_SCRN_DELETE_REQ to screen leave_proc. If there are many screens,
                                      * we will send this event to each screen. At last, we EVT_ID_SCRN_DEINIT to all screen leave_proc.
                                      */
			return "EVT_ID_SCRN_DELETE_REQ";
    	case EVT_ID_SCRN_DELETE_REQ_IN_END_KEY: /* delete the inactive screen; invoke the delete process only:
                                                 * We will send EVT_ID_SCRN_DELETE_REQ to screen leave_proc. If there are many screens,
                                                 * we will send this event to each screen. At last, we EVT_ID_SCRN_DEINIT to all screen leave_proc.
                                                 */
			return "EVT_ID_SCRN_DELETE_REQ_IN_END_KEY";
    	case EVT_ID_SCRN_DEINIT: /* deinit the screen */
			return "EVT_ID_SCRN_DEINIT";
    	case EVT_ID_GROUP_POST_CALLER_NOTIFY:
			return "EVT_ID_GROUP_POST_CALLER_NOTIFY";
    	case EVT_ID_GROUP_POST_PARENT_NOTIFY:
			return "EVT_ID_GROUP_POST_PARENT_NOTIFY";
    	case EVT_ID_SCRN_POST_PARENT_NOTIFY:
			return "EVT_ID_SCRN_POST_PARENT_NOTIFY";
    	case EVT_ID_NO_CHILD_NOTIFY:
			return "EVT_ID_NO_CHILD_NOTIFY";
    	case EVT_ID_GOBACK_HISTORY:
			return "EVT_ID_GOBACK_HISTORY";
    	case EVT_ID_TAB_POST_LEFT_ARROW_NOTIFY:
			return "EVT_ID_TAB_POST_LEFT_ARROW_NOTIFY";
    	case EVT_ID_TAB_POST_RIGHT_ARROW_NOTIFY:
			return "EVT_ID_TAB_POST_RIGHT_ARROW_NOTIFY";
    	case EVT_ID_PRE_KEY:
			return "EVT_ID_PRE_KEY";
    	case EVT_ID_ON_KEY:
			return "EVT_ID_ON_KEY";
    	case EVT_ID_POST_KEY:
			return "EVT_ID_POST_KEY";
    	case EVT_ID_PRE_KEY_EVT_ROUTING:
			return "EVT_ID_PRE_KEY_EVT_ROUTING";
    	case EVT_ID_POST_KEY_EVT_ROUTING:
			return "EVT_ID_POST_KEY_EVT_ROUTING";
    	case EVT_ID_KEY_DEFAULT_HANDLER:
			return "EVT_ID_KEY_DEFAULT_HANDLER";
    	case EVT_ID_PRE_TOUCH_EVT_ROUTING:
			return "EVT_ID_PRE_TOUCH_EVT_ROUTING";
    	case EVT_ID_POST_TOUCH_EVT_ROUTING:
			return "EVT_ID_POST_TOUCH_EVT_ROUTING";
    	case EVT_ID_ON_TOUCH:
			return "EVT_ID_ON_TOUCH";
    	case EVT_ID_NMGR_DEFER:
			return "EVT_ID_NMGR_DEFER";
    	case EVT_ID_NMGR_PLAY_TONE:
			return "EVT_ID_NMGR_PLAY_TONE";
    	case EVT_ID_NMGR_PLAY_VIB:
			return "EVT_ID_NMGR_PLAY_VIB";
		case EVT_ID_NMGR_IDLE_CANCEL:
			return "EVT_ID_NMGR_IDLE_CANCEL";
    	case EVT_ID_ROOT_SCRN_FIRST_ENTER:
			return "EVT_ID_ROOT_SCRN_FIRST_ENTER";
    	case EVT_ID_ROOT_SCRN_COMM_ENTER:
			return "EVT_ID_ROOT_SCRN_COMM_ENTER";
    	case EVT_ID_ROOT_SCRN_COMM_POST_ENTER:
			return "EVT_ID_ROOT_SCRN_COMM_POST_ENTER";
    	case EVT_ID_SCRN_ENTER_SUCCESS_NOFITY:
			return "EVT_ID_SCRN_ENTER_SUCCESS_NOFITY";
    	case EVT_ID_NMGR_SUBLCD_NOTIFY:
			return "EVT_ID_NMGR_SUBLCD_NOTIFY";
    	case EVT_ID_SRV_INIT:
			return "EVT_ID_SRV_INIT";
    	case EVT_ID_SRV_DEINIT:
			return "EVT_ID_SRV_DEINIT";
    	case EVT_ID_GROUP_HIDE:
			return "EVT_ID_GROUP_HIDE";
    	case EVT_ID_GROUP_UNHIDE:
			return "EVT_ID_GROUP_UNHIDE";
    	case EVT_ID_SCRN_HIDE:
			return "EVT_ID_SCRN_HIDE";
    	case EVT_ID_SCRN_UNHIDE:
			return "EVT_ID_SCRN_UNHIDE";
    	case EVT_ID_POST_TO_GROUP:
			return "EVT_ID_POST_TO_GROUP";
    	case EVT_ID_POST_TO_SRV:
			return "EVT_ID_POST_TO_SRV";
    	case EVT_ID_SCENARIO_END:
			return "EVT_ID_SCENARIO_END";
    	case EVT_ID_APP_GROUP_POST_NOTIFY:
			return "EVT_ID_APP_GROUP_POST_NOTIFY";
    	case EVT_ID_APP_CONFIG:
			return "EVT_ID_APP_CONFIG";
    	case EVT_ID_DELETE_DANGLE_GROUP_REQ:
			return "EVT_ID_DELETE_DANGLE_GROUP_REQ";
	    case EVT_ID_WGUI_LSK_CLICK: /* left softkey event */
			return "EVT_ID_WGUI_LSK_CLICK";
    	case EVT_ID_WGUI_RSK_CLICK: /* right softkey event */
			return "EVT_ID_WGUI_RSK_CLICK";
    	case EVT_ID_WGUI_CSK_CLICK: /* center softkry event */
			return "EVT_ID_WGUI_CSK_CLICK";
		case EVT_ID_SCRN_HIGHLIGHT_CHANGE:
			return "EVT_ID_SCRN_HIGHLIGHT_CHANGE";
		case EVT_ID_SCRN_GET_CURR_PARENT_ID:
			return "EVT_ID_SCRN_GET_CURR_PARENT_ID";
    	case EVT_ID_PRE_PROTOCOL:
			return "EVT_ID_PRE_PROTOCOL";
    	case EVT_ID_POST_PROTOCOL:
			return "EVT_ID_POST_PROTOCOL";
    	case EVT_ID_APP_TERMINATED:
			return "EVT_ID_APP_TERMINATED";
    	case EVT_ID_APP_ENTER:
			return "EVT_ID_APP_ENTER";
    	case EVT_ID_APP_DEINIT:
			return "EVT_ID_APP_DEINIT";
    	case EVT_ID_APP_CLOSE_REQ:
			return "EVT_ID_APP_CLOSE_REQ";
    	case EVT_ID_APP_INIT:
			return "EVT_ID_APP_INIT";
    	case EVT_ID_APP_ACTIVE:
			return "EVT_ID_APP_ACTIVE";
	    case EVT_ID_APP_INACTIVE:
			return "EVT_ID_APP_INACTIVE";
    	case EVT_ID_APP_TOP_ACTIVE:
			return "EVT_ID_APP_TOP_ACTIVE";
    	case EVT_ID_APP_TOP_INACTIVE:
			return "EVT_ID_APP_TOP_INACTIVE";
    	case EVT_ID_APP_NO_CHILD:
			return "EVT_ID_APP_NO_CHILD";
    	case EVT_ID_APP_FORCE_CLOSE:
			return "EVT_ID_APP_FORCE_CLOSE";
    	case EVT_ID_APP_HIDE:
			return "EVT_ID_APP_HIDE";
    	case EVT_ID_APP_UNHIDE:
			return "EVT_ID_APP_UNHIDE";
    	case EVT_ID_APP_POST_UNHIDE:
			return "EVT_ID_APP_POST_UNHIDE";
    	case EVT_ID_APP_POST_ENTER:
			return "EVT_ID_APP_POST_ENTER";
    	case EVT_ID_NMGR_FORCE_DEFER_QUERY:
			return "EVT_ID_NMGR_FORCE_DEFER_QUERY";
    	case EVT_ID_MMI_TIMER_EXPIRY_PROC_EXT:
			return "EVT_ID_MMI_TIMER_EXPIRY_PROC_EXT";
    	case EVT_ID_MMI_TIMER_INIT_EXT:
			return "EVT_ID_MMI_TIMER_INIT_EXT";
    	case EVT_ID_OOM_HANDLE_FAIL:
			return "EVT_ID_OOM_HANDLE_FAIL";
    	case EVT_ID_PRE_ACTIVE_IDLE_APP_IN_END_KEY:
			return "EVT_ID_PRE_ACTIVE_IDLE_APP_IN_END_KEY";
    	case EVT_ID_POST_ACTIVE_IDLE_APP_IN_END_KEY:
			return "EVT_ID_POST_ACTIVE_IDLE_APP_IN_END_KEY";
    	case EVT_ID_MMI_AP_DCM_LOAD:
			return "EVT_ID_MMI_AP_DCM_LOAD";
    	case EVT_ID_MMI_AP_DCM_UNLOAD:
			return "EVT_ID_MMI_AP_DCM_UNLOAD";
    	case EVT_ID_SCENARIO_MAX:
			return "EVT_ID_SCENARIO_MAX";
		default:
			return "DEFAULT";
	}
}

static char *hzwKeyCodeStr(U16 keyCode) {
	switch (keyCode) {
		case KEY_LSK:
			return "KEY_LSK";
		case KEY_CSK:
			return "KEY_CSK";
		case KEY_RSK:
			return "KEY_RSK";
		case KEY_UP_ARROW:
			return "KEY_UP_ARROW";
		case KEY_DOWN_ARROW:
			return "KEY_DOWN_ARROW";
		case KEY_LEFT_ARROW:
			return "KEY_LEFT_ARROW";
		case KEY_RIGHT_ARROW:
			return "KEY_RIGHT_ARROW";
		case KEY_SEND:
			return "KEY_SEND";
		case KEY_END:
			return "KEY_END";
		case KEY_1:
			return "KEY_1";
		case KEY_2:
			return "KEY_2";
		case KEY_3:
			return "KEY_3";
		case KEY_4:
			return "KEY_4";
		case KEY_5:
			return "KEY_5";
		case KEY_6:
			return "KEY_6";
		case KEY_7:
			return "KEY_7";
		case KEY_8:
			return "KEY_8";
		case KEY_9:
			return "KEY_9";
		case KEY_STAR:
			return "KEY_STAR";
		case KEY_0:
			return "KEY_0";
		case KEY_POUND:
			return "KEY_POUND";
		default:
			return "DEFAULT";
	}
}

static char* hzwKeyTypeStr(U16 keyType) {
	switch (keyType) {
		case KEY_EVENT_DOWN:
			return "KEY_EVENT_DOWN";
		case KEY_EVENT_UP:
			return "KEY_EVENT_UP";
		default:
			return "DEFAULT";
	}
}

static char* hzwKeyEvtStr(U16 keyCode, U16 keyType) {
	static char str[50] = {0};

	sprintf(str, "%s %s", hzwKeyCodeStr(keyCode), hzwKeyTypeStr(keyType));

	return str;
}

static mmi_ret hzwGrpProc(mmi_event_struct* evt) {
	char *evtStr = hzwEvtStr(evt->evt_id);
	kal_int32 userData = evt->user_data;

	return MMI_RET_OK;
}

void hzwDraw(void *user_data) {
	kal_uint8 str[32];
	kal_uint16 wstr[32];

	sprintf(str, "App %d", user_data);
	mmi_asc_to_ucs2(wstr, str);
	gdi_layer_clear(GDI_COLOR_WHITE);

	gui_move_text_cursor(0, 0);
	gui_set_text_color(gui_color(0, 0, 0));
	gui_print_text(wstr);
}

static U16 hzwKeys[] = {
	KEY_LSK,
	KEY_CSK,
	KEY_RSK,
	KEY_UP_ARROW,
	KEY_DOWN_ARROW,
	KEY_LEFT_ARROW,
	KEY_RIGHT_ARROW,
	KEY_SEND,
	KEY_END,
	KEY_1,
	KEY_2,
	KEY_3,
	KEY_4,
	KEY_5,
	KEY_6,
	KEY_7,
	KEY_8,
	KEY_9,
	KEY_STAR,
	KEY_0,
	KEY_POUND,
};

void hzwKeyHandler() {
	U16 keyCode = 0, keyType = 0;
	char *evtStr = NULL;

	mmi_frm_get_key_info(&keyCode, &keyType);
	evtStr = hzwKeyEvtStr(keyCode, keyType);

    if (keyType != KEY_EVENT_UP) return;

    switch (keyCode) {
        case KEY_RSK:
        case KEY_END:
            mmi_frm_group_close(HZW_GRP_ID);
            break;
    }
}

static void hzwExitProc() {
	int i = 0;
}

static mmi_ret hzwLeaveProc(mmi_event_struct* evt) {
	char *evtStr = hzwEvtStr(evt->evt_id);
	kal_int32 userData = evt->user_data;

	return MMI_RET_OK;
}

static void hzwEntryProc(mmi_scrn_essential_struct *data) {
	if (!mmi_frm_scrn_enter(
			HZW_GRP_ID, 
			HZW_SCR_ID, 
			hzwExitProc, 
			hzwEntryProc, 
			MMI_FRM_FULL_SCRN)) {
		return;
	}

	mmi_frm_scrn_set_leave_proc(HZW_GRP_ID, HZW_SCR_ID, hzwLeaveProc);

	hzwDraw(data->user_data);
	mmi_frm_set_group_key_handler(hzwKeyHandler, hzwKeys, sizeof(hzwKeys) / sizeof(hzwKeys[0]), KEY_EVENT_DOWN);
	mmi_frm_set_group_key_handler(hzwKeyHandler, hzwKeys, sizeof(hzwKeys) / sizeof(hzwKeys[0]), KEY_EVENT_UP);
}

static mmi_ret hzwLeaveProc1(mmi_event_struct *evt) {
	char *evtStr = hzwEvtStr(evt->evt_id);
	kal_int32 userData = evt->user_data;

	return MMI_RET_OK;
}

void HzwLaunch(kal_int32 num) {
	mmi_frm_group_create(GRP_ID_ROOT, HZW_GRP_ID, hzwGrpProc, num);
	mmi_frm_group_enter(HZW_GRP_ID, MMI_FRM_NODE_SMART_CLOSE_FLAG);

	mmi_frm_scrn_first_enter_ex(
		HZW_GRP_ID,
		HZW_SCR_ID,
		hzwEntryProc,
		num,
		hzwLeaveProc1);
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
