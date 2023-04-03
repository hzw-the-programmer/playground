static void exit_setting()
{
}

static void enter_setting()
{
    EntryNewScreen(777, exit_setting, enter_setting, NULL);
    gdi_layer_clear(GDI_COLOR_RED);
    SetRightSoftkeyFunction(GoBackHistory, KEY_EVENT_UP);
    gdi_layer_blt_previous(0, 0, UI_DEVICE_WIDTH-1, UI_DEVICE_HEIGHT-1);
}

static void lsk() {
    //enter_setting();
	S16 phone, card;
	phone = MMI_FS_GetDrive(FS_DRIVE_V_NORMAL, 2, FS_DRIVE_I_SYSTEM | FS_DRIVE_V_NORMAL);
	card = MMI_FS_GetDrive(FS_DRIVE_V_REMOVABLE, 1, FS_NO_ALT_DRIVE);
    // phone=69,card=68
    // phone=-4,card=-4 // usb mode
	mmi_trace(MMI_TRACE_LEVEL_1, "phone=%d,card=%d", phone, card);
}

static void exit_app() {
}

static void enter_app() {
    EntryNewScreen(666, exit_app, enter_app, NULL);
    // no history
    // ExecuteCurrExitHandler_Ext -> GenericExitScreen
    //EntryNewScreen(666, exit_app, NULL, NULL);
    SetLeftSoftkeyFunction(lsk, KEY_EVENT_UP);
    SetRightSoftkeyFunction(GoBackHistory, KEY_EVENT_UP);

    gdi_layer_clear(GDI_COLOR_WHITE);

    gui_set_font(&MMI_small_font);
    gui_set_text_color(gui_color(0, 0, 0));

    gdi_layer_blt_previous(0, 0, UI_DEVICE_WIDTH-1, UI_DEVICE_HEIGHT-1);
}