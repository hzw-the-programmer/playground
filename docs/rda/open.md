static void draw(const char *text) {
    gdi_layer_clear(GDI_COLOR_WHITE);

    gui_set_font(&MMI_small_font);
    gui_set_text_color(gui_color(0, 0, 0));
	gui_move_text_cursor(50, 50);
	gui_printf("%s", text);

    gdi_layer_blt_previous(0, 0, UI_DEVICE_WIDTH-1, UI_DEVICE_HEIGHT-1);
}

static void lsk() {
}

static void key_1() {
}

static void key_2() {
    INT d;
    UINT8 path[64] = {0};
    UINT16 path_w[64] = {0};
    INT ret;
    FS_HANDLE h;

    d = MMI_FS_GetDrive(
        FS_DRIVE_V_REMOVABLE, 1, FS_NO_ALT_DRIVE);
    sprintf(path, "%c:/test1", d);
    mmi_asc_to_ucs2(path_w, path);
    h = MMI_FS_Open(path_w, FS_CREATE|FS_READ_WRITE);
    LOG("h=%d", h);

    sprintf(path, "%c:/test2", d);
    mmi_asc_to_ucs2(path_w, path);
    h = MMI_FS_Open(path_w, FS_CREATE|FS_READ_WRITE);
    LOG("h=%d", h);

    sprintf(path, "%c:/test3", d);
    mmi_asc_to_ucs2(path_w, path);
    h = MMI_FS_Open(path_w, FS_CREATE|FS_READ_WRITE);
    LOG("h=%d", h);

}

static void key_3() {
}

static void key_4() {
}

static void key_5() {
}

static void key_6() {
}

static void key_7() {
}

static void register_handlers() {
    SetLeftSoftkeyFunction(lsk, KEY_EVENT_UP);
    SetRightSoftkeyFunction(GoBackHistory, KEY_EVENT_UP);
	SetKeyHandler(key_1, KEY_1, KEY_EVENT_DOWN);
	SetKeyHandler(key_2, KEY_2, KEY_EVENT_DOWN);
	SetKeyHandler(key_3, KEY_3, KEY_EVENT_DOWN);
	SetKeyHandler(key_4, KEY_4, KEY_EVENT_DOWN);
	SetKeyHandler(key_5, KEY_5, KEY_EVENT_DOWN);
	SetKeyHandler(key_6, KEY_6, KEY_EVENT_DOWN);
	SetKeyHandler(key_7, KEY_7, KEY_EVENT_DOWN);
}

static void exit_app() {
}

static void enter_app() {
	EntryNewScreen(666, exit_app, enter_app, NULL);
    // no history
    // ExecuteCurrExitHandler_Ext -> GenericExitScreen
    //EntryNewScreen(666, exit_app, NULL, NULL);

	register_handlers();
	draw("");
}