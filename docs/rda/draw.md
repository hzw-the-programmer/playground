// setup_UI_wrappers
static void draw() {
	UI_font_type font = &MMI_small_font; //MMI_medium_font
	color_t text_color = {255, 255, 255, 255};
	color_t rect_color = {255, 0, 0, 255};
	S32 x = 50, y = 50, r = 10, w, h, xoff, yoff, bw, bh;
	UI_string_type str = L"2";

	//UI_measure_string(str, &w, &h);
	Get_CharBoundingBox('2', &w, &h, &xoff, &yoff, &bw, &bh);

    gdi_layer_clear(GDI_COLOR_WHITE);

	gdi_draw_solid_circle(x, y, r, gdi_act_color_from_rgb(0, 0, 0, 200));

	UI_set_font(font);
	UI_set_text_color(text_color);

	x-=bw>>1;
	y-=bh>>1;
	UI_draw_rectangle(x, y, x+bw, y+bh, rect_color);
	UI_move_text_cursor(x, y);
	UI_print_text(str);

	gdi_layer_blt_previous(0, 0, UI_DEVICE_WIDTH-1, UI_DEVICE_HEIGHT-1);
}

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

	draw();
}
