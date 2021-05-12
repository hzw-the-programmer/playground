```
#define HZW_GRP_ID
#define HZW_SCR_ID

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

	return MMI_RET_OK;
}

static S16 gTextY = 0;
static U16 gText[100] = {0};

void hzwDraw() {
    S32 w, h;

    gui_measure_string(gText, &w, &h);
    if (!h || gTextY + h > UI_DEVICE_HEIGHT) {
        gdi_layer_clear(GDI_COLOR_WHITE);
        gTextY = 0;
    }

	gui_move_text_cursor(0, gTextY);
	gui_set_text_color(gui_color(0, 0, 0));
	gui_print_text(gText);

    gTextY += h;

    gui_BLT_double_buffer(0, 0, UI_DEVICE_WIDTH, UI_DEVICE_HEIGHT);
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

typedef struct HzwTimer {
    U8 id;
    U32 start;
    U32 delay;
    void (*cb)(int id);
    MMI_BOOL oneshot;
    struct HzwTimer *next;
} HzwTimer;

#include "fix_size_mem.h"
static U8 gTimerId = 1;
static HzwTimer *gTimer = NULL;

static HzwTimer* hzwTimerNew(U32 delay, void(*cb)(U8 id), MMI_BOOL oneshot) {
    HzwTimer *timer = NULL;

    timer = mmi_frm_malloc(sizeof(HzwTimer));
    if (!timer) return NULL;

    timer->id = gTimerId++;
    if (!gTimerId) gTimerId = 1;
    timer->start = kal_ticks_to_milli_secs(kal_get_systicks());
    timer->delay = delay;
    timer->cb = cb;
    timer->oneshot = oneshot;
    timer->next = NULL;

    return timer;
}

static void hzwTimerAdd(HzwTimer *timer) {
    HzwTimer **tmp = &gTimer;

    while (*tmp) {
        tmp = &(*tmp)->next;
    }

    *tmp = timer;
}

static void hzwTimerRemove(HzwTimer *timer) {
    HzwTimer **tmp = &gTimer;

    while (*tmp) {
        if ((*tmp)->id == timer->id) {
            *tmp = timer->next;
            timer->next = NULL;
            break;
        } else {
            tmp = &(*tmp)->next;
        }
    }
}

static HzwTimer* hzwTimerGet(U8 id) {
    HzwTimer *timer = gTimer;

    while (timer) {
        if (timer->id == id) {
            return timer;
        } else {
            timer = timer->next;
        }
    }

    return NULL;
}

static HzwTimer* hzwTimerRecent() {
    U32 recentExpire = 0, expire = 0;
    HzwTimer *recentTimer = NULL, *timer = NULL;

    timer = gTimer;

    if (!timer) return NULL;

    recentExpire = timer->start + timer->delay;
    recentTimer = timer;
    while (timer->next) {
        timer = timer->next;
        expire = timer->start + timer->delay;
        if (expire < recentExpire) {
            recentExpire = expire;
            recentTimer = timer;
        }
    }

    return recentTimer;
}

void hzwTimerCb() {
    HzwTimer *timer = NULL;
    U32 current = 0;

    current = kal_ticks_to_milli_secs(kal_get_systicks());
    while ((timer = hzwTimerRecent()) && (timer->start + timer->delay) <= current) {
        timer->cb(timer->id);
        if (timer->oneshot) {
            hzwTimerRemove(timer);
            mmi_frm_free(timer);
        } else {
            timer->start = current;
        }
    }

    if (timer) {
        gui_start_timer((timer->start + timer->delay) - current, hzwTimerCb);
    }
}

U8 hzwTimerStart(U32 delay, void(*cb)(U8 id), MMI_BOOL oneshot) {
    HzwTimer *newTimer = NULL, *recentTimer = NULL;
    int newExpire = 0, recentExpire = 0;

    newTimer = hzwTimerNew(delay, cb, oneshot);
    if (!newTimer) return 0;

    recentTimer = hzwTimerRecent();
    if (recentTimer) {
        recentExpire = recentTimer->start + recentTimer->delay;
    }

    newExpire = newTimer->start + newTimer->delay;
    
    if (!recentExpire) {
        gui_start_timer(newTimer->delay, hzwTimerCb);
    } else if (recentExpire > newExpire) {
        gui_cancel_timer(hzwTimerCb);
        gui_start_timer(newTimer->delay, hzwTimerCb);
    }

    hzwTimerAdd(newTimer);

    return newTimer->id;
}

void hzwTimerStop(U8 id) {
    HzwTimer *timer = NULL, *recentTimer = NULL;
    U32 current = 0;

    current = kal_ticks_to_milli_secs(kal_get_systicks());
    recentTimer = hzwTimerRecent();
    if (recentTimer && recentTimer->id == id) {
        gui_cancel_timer(hzwTimerCb);
        hzwTimerRemove(recentTimer);
        mmi_frm_free(recentTimer);
        recentTimer = hzwTimerRecent();
        if (recentTimer) {
            gui_start_timer((recentTimer->start + recentTimer->delay) - current, hzwTimerCb);
        }
        return;
    }

    timer = hzwTimerGet(id);
    hzwTimerRemove(timer);
    mmi_frm_free(timer);
}

void hzwTimerDebug(char *prefix, U8 id) {
    HzwTimer *timer = NULL;
    U32 current = 0, diff = 0;

    current = kal_ticks_to_milli_secs(kal_get_systicks());
    timer = hzwTimerGet(id);
    diff =  current - (timer->start + timer->delay);

    kal_wsprintf(gText, "%s:%d:%d", prefix, id, diff);
    hzwDraw();
}

void hzwTimerCb1(U8 id) {
    hzwTimerDebug("hzwTimerCb1", id);
}

void hzwTimerCb2(U8 id) {
    hzwTimerDebug("hzwTimerCb2", id);
}

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

        case KEY_1: {
            U8 i = 255;
            while (i != 254) {
                i++;
            }
            i += 1;
            break;
        }

        case KEY_2: {
            static U8 id = 0;
            if (id) {
                hzwTimerStop(id);
                id = 0;
            } else {
                id = hzwTimerStart(1000, hzwTimerCb1, MMI_FALSE);
            }
            break;
        }

        case KEY_3: {
            static U8 id = 0;
            if (id) {
                hzwTimerStop(id);
                id = 0;
            } else {
                id = hzwTimerStart(2000, hzwTimerCb2, MMI_FALSE);
            }
            break;
        }

        case KEY_4: {
            hzwTimerStart(1000, hzwTimerCb2, MMI_TRUE);
            hzwTimerStart(1000, hzwTimerCb2, MMI_TRUE);
            break;
        }
    }
}

static void hzwExitProc() {
	int i = 0;
}

static mmi_ret hzwLeaveProc(mmi_event_struct* evt) {
	char *evtStr = hzwEvtStr(evt->evt_id);

	return MMI_RET_OK;
}

static void hzwEntryProc() {
	if (!mmi_frm_scrn_enter(
			HZW_GRP_ID, 
			HZW_SCR_ID, 
			hzwExitProc, 
			hzwEntryProc, 
			MMI_FRM_FULL_SCRN)) {
		return;
	}

	mmi_frm_scrn_set_leave_proc(HZW_GRP_ID, HZW_SCR_ID, hzwLeaveProc);

	hzwDraw();
	mmi_frm_set_group_key_handler(hzwKeyHandler, hzwKeys, sizeof(hzwKeys) / sizeof(hzwKeys[0]), KEY_EVENT_DOWN);
	mmi_frm_set_group_key_handler(hzwKeyHandler, hzwKeys, sizeof(hzwKeys) / sizeof(hzwKeys[0]), KEY_EVENT_UP);
}

void HzwLaunch() {
	mmi_frm_group_create(GRP_ID_ROOT, HZW_GRP_ID, hzwGrpProc, NULL);
	mmi_frm_group_enter(HZW_GRP_ID, MMI_FRM_NODE_SMART_CLOSE_FLAG);

	mmi_frm_scrn_first_enter(
		HZW_GRP_ID,
		HZW_SCR_ID,
		hzwEntryProc,
		NULL);
}
```
