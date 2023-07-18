// MMI_FS_CreateDir
// FS_MakeDir
// dir_namei
void log_wstr(UINT8 *tag, UINT16 *wstr) {
    UINT8 path[64] = {0};
    mmi_ucs2_n_to_asc(path, wstr, 63);
    LOG("%s:%s", tag, path);
}

static void check(UINT16 d) {
    UINT8 path[64] = {0};
    UINT16 path_w[64] = {0};
    INT ret;

    sprintf(path, "%c", d);
    mmi_asc_to_ucs2(path_w, path);
    ret = MMI_FS_IsExist(path_w); // 0
    LOG("%s=%d", path, ret);

    sprintf(path, "%c:", d);
    mmi_asc_to_ucs2(path_w, path);
    ret = MMI_FS_IsExist(path_w); // 0
    LOG("%s=%d", path, ret);

    sprintf(path, "%c:/", d);
    mmi_asc_to_ucs2(path_w, path);
    ret = MMI_FS_IsExist(path_w); // 1
    LOG("%s=%d", path, ret);

    sprintf(path, "%c:/h", d);
    mmi_asc_to_ucs2(path_w, path);
    ret = MMI_FS_IsExist(path_w); // 1
    LOG("%s=%d", path, ret);

    sprintf(path, "%c:/h/", d);
    mmi_asc_to_ucs2(path_w, path);
    ret = MMI_FS_IsExist(path_w); // 1
    LOG("%s=%d", path, ret);

    sprintf(path, "%c:/h/h", d);
    mmi_asc_to_ucs2(path_w, path);
    ret = MMI_FS_IsExist(path_w); // 1
    LOG("%s=%d", path, ret);

    sprintf(path, "%c:/h/h/", d);
    mmi_asc_to_ucs2(path_w, path);
    ret = MMI_FS_IsExist(path_w); // 1
    LOG("%s=%d", path, ret);
}

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
    INT phone;
    UINT8 path[64] = {0};
    UINT16 path_w[64] = {0};
    INT ret;

    phone = MMI_FS_GetDrive(
        FS_DRIVE_V_NORMAL,
        2,
        FS_DRIVE_I_SYSTEM | FS_DRIVE_V_NORMAL);
    sprintf(path, "%c:/parent/child", phone);
    LOG("p:%s", path);
    mmi_asc_to_ucs2(path_w, path);
    ret = MMI_FS_CreateDir(path_w); // will crash if parent does not exist
    LOG("ret:%d", ret);
}

static void key_3() {
    check(MMI_FS_GetDrive(
        FS_DRIVE_V_NORMAL,
        2,
        FS_DRIVE_I_SYSTEM | FS_DRIVE_V_NORMAL));
}

static void key_4() {
}

static void key_5() {
    check(MMI_FS_GetDrive(
        FS_DRIVE_V_REMOVABLE, 1, FS_NO_ALT_DRIVE));
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
