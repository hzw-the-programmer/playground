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
			return "default";
	}
}

static mmi_ret hzwGrpProc(mmi_event_struct *evt) {
	char *evtStr = hzwEvtStr(evt->evt_id);

	return MMI_RET_OK;
}

static void hzwExitProc() {

}

static void hzwEntryProc(mmi_scrn_essential_struct* scrn_data) {
	multi_line_input_box b = {0};
	S32 x = 0, y = 0, width = 240, height = 0;
	UI_string_type buffer = L"abcdefghijklmnopqrstuvwxyz1234567890abcdefghijklmnopqrstuvwxyz1";
		//L"abcdefghijklmnopqrstuvwxyz1234567890abcdefghijklmnopqrstuvwxyz1234567890";
		// L"abcdefghijklmnopqrstuvwxyz1234567890abcdefghijklmnopqrstuvwxyz1234567890"
		// L"abcdefghijklmnopqrstuvwxyz1234567890abcdefghijklmnopqrstuvwxyz1234567890";
	S32 text_length = kal_wstrlen(buffer) * 2;
	S32 length = text_length + 2;

	MMI_ASSERT(scrn_data->group_id == HZW_GRP_ID);
	MMI_ASSERT(scrn_data->scrn_id == HZW_SCR_ID);

	if (!mmi_frm_scrn_enter(
			scrn_data->group_id, 
			HZW_SCR_ID, 
			hzwExitProc, 
			(FuncPtr)hzwEntryProc, 
			MMI_FRM_FULL_SCRN)) {
		return;
	}

	x = 0;
	y = 0;
	width = UI_DEVICE_WIDTH - (UI_DEVICE_WIDTH >> 2);
	height = 0;

	gui_create_multi_line_input_box_set_buffer(
		&b,
		x,
		y,
		width,
		height,
		buffer,
		length,
		text_length,
		0);

	b.flags |= UI_MULTI_LINE_INPUT_BOX_DISABLE_SCROLLBAR
		| UI_MULTI_LINE_INPUT_BOX_DISABLE_CURSOR_DRAW;

	gui_show_multi_line_input_box_no_draw(&b);

	height = b.text_height;
	if (b.n_lines > 4) {
		height = b.max_line_height * 4 + 4;
		b.ext_flags |= GUI_MULTI_LINE_INPUT_BOX_TRUNCATE_CONTENT;
	}

	gui_move_multi_line_input_box(&b, (UI_DEVICE_WIDTH - width) >> 1, (UI_DEVICE_HEIGHT - height) >> 1);
	gui_resize_multi_line_input_box(&b, width, height);

#if 0
	b.flags |= UI_MULTI_LINE_INPUT_BOX_DISABLE_CURSOR_DRAW
		| UI_MULTI_LINE_INPUT_BOX_DISABLE_SCROLLBAR;
		// | UI_MULTI_LINE_INPUT_BOX_DISABLE_BACKGROUND;
	b.ext_flags |= GUI_MULTI_LINE_INPUT_BOX_TRUNCATE_CONTENT;

	gui_resize_multi_line_input_box(&b, width, 100);
#endif

	gui_show_multi_line_input_box(&b);
}

static void hzwLaunch() {
	mmi_frm_group_create(GRP_ID_ROOT, HZW_GRP_ID, hzwGrpProc, NULL);
	mmi_frm_group_enter(HZW_GRP_ID, MMI_FRM_NODE_SMART_CLOSE_FLAG);

	mmi_frm_scrn_first_enter(
		HZW_GRP_ID,
		HZW_SCR_ID,
		(FuncPtr)hzwEntryProc,
		NULL);
}
```

进入流程：
mmi_frm_group_enter触发三个事件：
EVT_ID_GROUP_ENTER
EVT_ID_GROUP_ACTIVE
EVT_ID_GROUP_FOCUSED

mmi_frm_scrn_first_enter调用：
hzwEntryProc

end键退出流程：
EVT_ID_GROUP_GOBACK_IN_END_KEY
hzwExitProc
EVT_ID_GROUP_DEFOCUSED
EVT_ID_GROUP_INACTIVE
EVT_ID_GROUP_EXIT
EVT_ID_GROUP_DEINIT
