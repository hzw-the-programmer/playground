```
#define HZW_GRP_ID
#define HZW_SCR_ID
#define HZW_LOG_DIR L"hzw\\"
#define HZW_LOG_BUF_LEN 2048

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
			return "default";
	}
}

void hzwLog(char *file, int line, char *fmt, ...) {
	static FS_HANDLE handle = FS_NO_ERROR;

	int result = FS_NO_ERROR;
	WCHAR pathName[100] = {0};
	kal_uint32 type, serial, altMask;
	int try;
	applib_time_struct dateTime;
	char buf[HZW_LOG_BUF_LEN] = {0};
	kal_uint32 len, written;
	va_list vl;

	applib_dt_get_date_time(&dateTime);

	if (handle <= FS_NO_ERROR) {
		try = 2;
		while (try--) {
			type = FS_DRIVE_V_REMOVABLE;
			serial = 1;
			altMask = FS_NO_ALT_DRIVE;

			if (try == 0) {
				type = FS_DRIVE_V_NORMAL;
				serial = 2;
				altMask = FS_DRIVE_V_NORMAL;	
			}

			result = FS_GetDrive(type, serial, altMask);
			MMI_ASSERT(result > FS_NO_ERROR);

			kal_wsprintf(pathName, "c:\\", result);
			kal_wstrcat(pathName, HZW_LOG_DIR);

			result = FS_Open(pathName, FS_OPEN_DIR | FS_READ_ONLY);
			if (result < FS_NO_ERROR) {
				result = FS_CreateDir(pathName);
				if (result >= FS_NO_ERROR) {
					break;
				}
			} else {
				FS_Close(result);
				break;
			}
		}

		MMI_ASSERT(try >= 0);

		kal_wsprintf(
			pathName + kal_wstrlen(pathName),
			"%04d%02d%02d_%02d%02d%02d.log",
			dateTime.nYear,
			dateTime.nMonth,
			dateTime.nDay,
			dateTime.nHour,
			dateTime.nMin,
			dateTime.nSec
		);

		result = FS_Open(pathName, FS_READ_WRITE);
		if (result < FS_NO_ERROR) {
			result = FS_Open(pathName, FS_READ_WRITE | FS_CREATE);
		}
		MMI_ASSERT(result > FS_NO_ERROR);

		handle = result;
	}

	file = strrchr(file, '\\') + 1;

	sprintf(buf,
		"%04d%02d%02d %02d%02d%02d %s %d: ",
		dateTime.nYear,
		dateTime.nMonth,
		dateTime.nDay,
		dateTime.nHour,
		dateTime.nMin,
		dateTime.nSec,
		file,
		line
	);

	len = strlen(buf);
	va_start(vl, fmt);
	vsnprintf(buf + len, HZW_LOG_BUF_LEN, fmt, vl);
	va_end(vl);

	len = strlen(buf);
	sprintf(buf + len, "\n");

	len = strlen(buf);
	result = FS_Write(handle, buf, len, &written);
	if (result < FS_NO_ERROR) {
		goto err;
	}
	MMI_ASSERT(written == len);

	result = FS_Commit(handle);
	if (result < FS_NO_ERROR) {
		goto err;
	}

	return;

err:
	FS_Close(handle);
	handle = FS_NO_ERROR;
}

#define HZW_LOG(fmt, ...) \
	hzwLog(__FILE__, __LINE__, fmt, ##__VA_ARGS__)

static mmi_ret hzwGrpProc(mmi_event_struct *evt) {
	char *evtStr = hzwEvtStr(evt->evt_id);

	HZW_LOG("hzwGrpProc: %s", evtStr);

	return MMI_RET_OK;
}

static void hzwExitProc() {
	HZW_LOG("hzwExitProc");
}

static void hzwEntryProc(mmi_scrn_essential_struct* scrn_data) {
	HZW_LOG("hzwEntryProc");

	MMI_ASSERT(scrn_data->group_id == HZW_GRP_ID);
	MMI_ASSERT(scrn_data->scrn_id == HZW_SCR_ID);

	if (!mmi_frm_scrn_enter(
			scrn_data->group_id, 
			scrn_data->scrn_id, 
			hzwExitProc, 
			(FuncPtr)hzwEntryProc, 
			MMI_FRM_FULL_SCRN)) {
		return;
	}

	gdi_layer_clear(GDI_COLOR_WHITE);
	gui_move_text_cursor(0, 0);
	gui_set_text_color(gui_color(0, 0, 0));
	gui_print_text(L"hzwEntryProc");
}

static void hzwLaunch() {
	mmi_frm_group_create(GRP_ID_ROOT, HZW_GRP_ID, hzwGrpProc, NULL);
	mmi_frm_group_enter(HZW_GRP_ID, MMI_FRM_NODE_SMART_CLOSE_FLAG);
	HZW_LOG("hzwLaunch: after group enter");

	mmi_frm_scrn_first_enter(
		HZW_GRP_ID,
		HZW_SCR_ID,
		(FuncPtr)hzwEntryProc,
		NULL);
	HZW_LOG("hzwLaunch: after screen enter");
}
```
