```
#define HZW_GRP_ID
#define HZW_SCR_ID

typedef struct {
	MMI_BOOL inited;
} HzwState;

typedef enum {
	HZW_ACTION_INIT,
	HZW_ACTION_DEINIT,
} HzwActionCode;

typedef struct {
	HzwActionCode code;
	void *param;
} HzwAction;

typedef MMI_BOOL (*HzwReducer)(HzwAction *action, HzwState *state);
typedef void (*HzwSubscriber)();

#define HZW_MAX_REDUCERS 5

typedef struct {
	HzwReducer reducers[HZW_MAX_REDUCERS];
	int len;
} HzwReducerMgr;

static HzwReducerMgr hzwReducerMgr = {0};

static MMI_BOOL hzwAddReducer(HzwReducer reducer) {
	int i = 0;
	int len = hzwReducerMgr.len;
	HzwReducer *reducers = hzwReducerMgr.reducers;

	for (i = 0; i < len; i++) {
		if (reducers[i] == reducer) {
			return MMI_FALSE;
		}
	}

	MMI_ASSERT(len < HZW_MAX_REDUCERS);

	reducers[hzwReducerMgr.len++] = reducer;

	return MMI_TRUE;
}

static MMI_BOOL hzwRemoveReducer(HzwReducer reducer) {
	int i = 0;
	int len = hzwReducerMgr.len;
	HzwReducer *reducers = hzwReducerMgr.reducers;

	for (i = 0; i < len; i++) {
		if (reducers[i] == reducer) {
			memmove(reducers+i, reducers+i+1, sizeof(HzwReducer)*(len - i));
			reducers[--hzwReducerMgr.len] = NULL;
			return MMI_TRUE;
		}
	}

	return MMI_FALSE;
}

static MMI_BOOL hzwTestReducer1(HzwAction *action, HzwState *state) {
}

static MMI_BOOL hzwTestReducer2(HzwAction *action, HzwState *state) {
}

static MMI_BOOL hzwTestReducer3(HzwAction *action, HzwState *state) {
}

static MMI_BOOL hzwTestReducer4(HzwAction *action, HzwState *state) {
}

static MMI_BOOL hzwTestReducer5(HzwAction *action, HzwState *state) {
}

static MMI_BOOL hzwTestReducer6(HzwAction *action, HzwState *state) {
}

static void hzwTestReducerMgr() {
	hzwAddReducer(hzwTestReducer1);
	hzwAddReducer(hzwTestReducer1);
	hzwAddReducer(hzwTestReducer2);
	hzwAddReducer(hzwTestReducer2);
	hzwAddReducer(hzwTestReducer3);
	hzwAddReducer(hzwTestReducer3);
	hzwAddReducer(hzwTestReducer3);
	hzwAddReducer(hzwTestReducer4);
	hzwAddReducer(hzwTestReducer4);
	hzwAddReducer(hzwTestReducer5);
	//hzwAddReducer(hzwTestReducer6);

	hzwRemoveReducer(hzwTestReducer5);
	hzwRemoveReducer(hzwTestReducer1);
	hzwRemoveReducer(hzwTestReducer3);
	hzwRemoveReducer(hzwTestReducer4);
	hzwRemoveReducer(hzwTestReducer2);
	hzwRemoveReducer(hzwTestReducer2);
}

MMI_BOOL hzwRootReducer(HzwAction *action, HzwState *state) {
	switch (action->code) {
		case HZW_ACTION_INIT:
			state->inited = MMI_TRUE;
			break;
		case HZW_ACTION_DEINIT:
			state->inited = MMI_FALSE;
			break;
		default:
			return MMI_FALSE;
	}

	return MMI_TRUE;
}

static HzwState hzwState = {0};

char *hzwActionStr(HzwActionCode code) {
	switch (code) {
		case HZW_ACTION_INIT:
			return "HZW_ACTION_INIT";
		case HZW_ACTION_DEINIT:
			return "HZW_ACTION_DEINIT";
	}
}

static void hzwDispatch(HzwAction *action) {
	int i = 0;
	MMI_BOOL changed = MMI_FALSE;

	for (i = 0; i < hzwReducerMgr.len; i++) {
		if (hzwReducerMgr.reducers[i](action, &hzwState)) {
			changed = MMI_TRUE;
		}
	}

	printf("hzwDispatch: action=%s, changed=%d\n", hzwActionStr(action->code), changed);

	if (!changed) {
		return;
	}
}

char *hzwEvtStr(U16 evtId) {
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
			return "default";
	}
}

void hzwInit() {
	HzwAction action = {0};

	hzwAddReducer(hzwRootReducer);

	action.code = HZW_ACTION_INIT;
	action.param = NULL;
	hzwDispatch(&action);
}

void hzwDeinit() {
	HzwAction action = {0};

	action.code = HZW_ACTION_DEINIT;
	action.param = NULL;
	hzwDispatch(&action);

	hzwRemoveReducer(hzwRootReducer);
}

mmi_ret hzwGrpProc(mmi_event_struct *evt) {
	char *evtStr = hzwEvtStr(evt->evt_id);

	return MMI_RET_OK;
}

void hzwExitProc() {
	MMI_ID id = 0;
}

mmi_ret hzwLeaveProc(mmi_event_struct* evt) {
	char *evtStr = hzwEvtStr(evt->evt_id);

	return MMI_RET_OK;
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

void hzwRunInBg() {
	mmi_frm_group_close(HZW_GRP_ID);
}

void hzwExit() {
	hzwDeinit();
	mmi_frm_group_close(HZW_GRP_ID);
}

void hzwGrpKeyHandler() {
	U16 keyCode = 0, keyType = 0;

	mmi_frm_get_key_info(&keyCode, &keyType);

	switch (keyCode) {
		case KEY_RSK:
		case KEY_END:
			if (keyType == KEY_EVENT_UP) {
				hzwRunInBg();
			}
			break;
		case KEY_POUND:
			if (keyType == KEY_EVENT_UP) {
				hzwExit();
			}
			break;
	}
}

void hzwDraw() {
	gdi_layer_clear(GDI_COLOR_WHITE);
}

void hzwEntryProc() {
	if (!mmi_frm_scrn_enter(
			HZW_GRP_ID,
			HZW_SCR_ID,
			hzwExitProc,
			hzwEntryProc,
			MMI_FRM_FULL_SCRN)) {
		return;
	}

	mmi_frm_scrn_set_leave_proc(HZW_GRP_ID, HZW_SCR_ID, hzwLeaveProc);

	mmi_frm_set_group_key_handler(hzwGrpKeyHandler, hzwKeys, sizeof(hzwKeys)/sizeof(hzwKeys[0]), KEY_EVENT_DOWN);
	mmi_frm_set_group_key_handler(hzwGrpKeyHandler, hzwKeys, sizeof(hzwKeys)/sizeof(hzwKeys[0]), KEY_EVENT_UP);

	hzwDraw();
}

void hzwLaunch() {
	//hzwTestReducerMgr();

	mmi_frm_group_create(GRP_ID_ROOT, HZW_GRP_ID, hzwGrpProc, NULL);
	mmi_frm_group_enter(HZW_GRP_ID, MMI_FRM_NODE_SMART_CLOSE_FLAG);

	if (!hzwState.inited) {
		hzwInit();
	}

	hzwEntryProc();
}
```
