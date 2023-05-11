static void draw(const char *text) {
    gdi_layer_clear(GDI_COLOR_RED);

    gui_set_font(&MMI_small_font);
    gui_set_text_color(gui_color(0, 0, 0));
	gui_move_text_cursor(50, 50);
	gui_printf("%s", text);

    gdi_layer_blt_previous(0, 0, UI_DEVICE_WIDTH-1, UI_DEVICE_HEIGHT-1);
}

static void draw_pop() {
    gdi_handle base;
    gdi_handle handle;

    gdi_layer_get_base_handle(&base);
    gdi_layer_create(0, 0, UI_DEVICE_WIDTH, UI_DEVICE_HEIGHT, &handle);
    gdi_layer_push_and_set_active(handle);
    gdi_effect_alpha_blending_rect(base,
        0, 0, UI_DEVICE_WIDTH-1, UI_DEVICE_HEIGHT-1,
        (100 - 75) * 256 / 100, 0, 0, 0);
    gdi_draw_solid_rect(0, UI_DEVICE_HEIGHT>>1, UI_DEVICE_WIDTH-1, UI_DEVICE_HEIGHT-1, GDI_COLOR_WHITE);
    gdi_layer_pop_and_restore_active();
    gdi_layer_blt_ext(handle, 0, 0, 0, 0, 0, UI_DEVICE_WIDTH-1, UI_DEVICE_HEIGHT-1);
    gdi_layer_free(handle);
}

static void timer_cb() {
    LOG("");
    draw_pop();
    StartTimer(KEY_TIMER_ID11, 1000, timer_cb);
}

static void lsk() {
}

static void key_1() {
}

static void key_2() {
    gdi_handle handle;

    gdi_layer_get_active(&handle);
    gdi_effect_alpha_blending_rect(handle,
        0, 0, UI_DEVICE_WIDTH-1, UI_DEVICE_HEIGHT-1,
        (100 - 75) * 256 / 100, 0, 0, 0);
    gdi_layer_blt_previous(0, 0, UI_DEVICE_WIDTH-1, UI_DEVICE_HEIGHT-1);
}

static void key_3() {
    timer_cb();
}

static void key_4() {
}

static void key_5() {
    static int i;
    LOG("msg %d", i++);
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
    LOG("");
    StopTimer(KEY_TIMER_ID11);
}

static void enter_app() {
	EntryNewScreen(666, exit_app, enter_app, NULL);
    // no history
    // ExecuteCurrExitHandler_Ext -> GenericExitScreen
    //EntryNewScreen(666, exit_app, NULL, NULL);

	register_handlers();
	draw("");
}