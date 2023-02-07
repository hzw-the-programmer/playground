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

static void lsk()
{
    enter_setting();
}

static void exit_app()
{
}

static char n2h(char n) {
	if (n < 10) return n+0x30;
	else return n-10+0x61;
}

static void u642str(U16 *buf, unsigned long long num) {
	int i;
	unsigned char t;
	for (i = 0; i < sizeof(unsigned long long); i++) {
		t = num>>((sizeof(unsigned long long) - 1 - i)*8);
		*buf++ = n2h(t>>4);
		*buf++ = n2h(t&0x0f);
	}
}

static void pf(U16 *buf, ...) {
	unsigned long long num;
	va_list va;
	va_start(va, buf);
	num = va_arg(va, unsigned long long);
	u642str(buf, num);
	va_end(va);
}

static void enter_app()
{
	U16 buf[32] = {0};
	//unsigned long long num = 0x1234567890123456;
	unsigned long long num = 0xffffffffffffffff;

    //EntryNewScreen(666, exit_app, enter_app, NULL);
    // no history
    // ExecuteCurrExitHandler_Ext -> GenericExitScreen
    EntryNewScreen(666, exit_app, NULL, NULL);

    gdi_layer_clear(GDI_COLOR_WHITE);
    gui_set_font(&MMI_small_font);
    gui_set_text_color(gui_color(0, 0, 0));
    gui_move_text_cursor(10, 10);
    pf(buf, num);
    gui_print_text(buf);

    SetLeftSoftkeyFunction(lsk, KEY_EVENT_UP);
    SetRightSoftkeyFunction(GoBackHistory, KEY_EVENT_UP);

    gdi_layer_blt_previous(0, 0, UI_DEVICE_WIDTH-1, UI_DEVICE_HEIGHT-1);
}
