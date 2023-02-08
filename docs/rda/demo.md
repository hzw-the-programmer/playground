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

static void u642wstr(U16 *buf, unsigned long long num) {
	int i;
	unsigned char t;
	for (i = 0; i < sizeof(unsigned long long); i++) {
		t = num>>((sizeof(unsigned long long) - 1 - i)*8);
		*buf++ = n2h(t>>4);
		*buf++ = n2h(t&0x0f);
	}
}

static void pfw(U16 *buf, ...) {
	unsigned long long num;
	va_list va;
	va_start(va, buf);
	num = va_arg(va, unsigned long long);
	u642wstr(buf, num);
	va_end(va);
}

static void u642str(U8 *buf, unsigned long long num) {
	int i;
	unsigned char t;
	for (i = 0; i < sizeof(unsigned long long); i++) {
		t = num>>((sizeof(unsigned long long) - 1 - i)*8);
		*buf++ = n2h(t>>4);
		*buf++ = n2h(t&0x0f);
	}
}

static void pf_(U8 *buf, S32 len, const U8 *format, va_list va) {
	unsigned long long num;
	unsigned long h, l;

	while (*format) {
		if (*format != '%') {
			*buf++ = *format++;
			continue;
		}
		format++;

		switch (*format) {
			case 'l':
			format++;
			if (*format == 'l') {
				format++;
			}
			break;
		}

		switch (*format) {
			case 'd':
			format++;
			num = va_arg(va, unsigned long);
			l = va_arg(va, unsigned long);
			num = (num<<32)|l;
			u642str(buf, num);
			break;
		}
	}
}

static void pf(U8 *buf, S32 len, const U8 *format, ...) {
	va_list va;
	va_start(va, format);
	pf_(buf, len, format, va);
	va_end(va);
}

static void paround(U8 *buf, U8 *s) {
	int i;
	U8 t;

	for (i = 0; i < 64; i++) {
		t = *(s+i);
		*buf++ = n2h(t>>4);
		*buf++ = n2h(t&0x0f);
	}
}

static U32 pstack(U8 *buf, U32 a, U16 b, U8 c) {
	U32 d = 0xdddddddd;
	U16 e = 0xeeee;
	U8 f = 0xff;

	paround(buf, &f);
}

static void multiline(S32 ox, S32 oy, U16 *buf, U16 inc) {
	U16 *p = buf;
	S32 l = UCS2Strlen(buf);
	U16 t;
	S32 w, h, max_w = 0, x = ox, y = oy; 

	while (p < buf + l) {
	    t = p[inc];
	    p[inc] = 0;
	    gui_measure_string(p, &w, &h);
	    if (max_w < w) max_w = w;
	    gui_move_text_cursor(x, y);
	    gui_print_text(p);
		p[inc] = t;

		p+=inc;
		y+=h;
		if (y+h >= UI_device_height) {
			y = oy;
			x += max_w + 10;
			max_w = 0;
		}
	}
}

#define BUF_LEN 256
#define SEP 4

#define SPLIT(num) (long)(num>>32), (long)(num)

static void enter_app()
{
	U16 unicode[BUF_LEN] = {0};
	U8 ascii[BUF_LEN] = {0};
	
	unsigned long long num = 0x1234567890123456;
	//unsigned long long num = 0xffffffffffffffff;

    //EntryNewScreen(666, exit_app, enter_app, NULL);
    // no history
    // ExecuteCurrExitHandler_Ext -> GenericExitScreen
    EntryNewScreen(666, exit_app, NULL, NULL);
    SetLeftSoftkeyFunction(lsk, KEY_EVENT_UP);
    SetRightSoftkeyFunction(GoBackHistory, KEY_EVENT_UP);

#if 1
    //pf(ascii, BUF_LEN, "xxxx%lld", num);
    //pf(ascii, BUF_LEN, "xxxx%lld", SPLIT(num));
    pstack(ascii, 0xaaaaaaaa, 0xbbbb, 0xcc);
    AnsiiToUnicodeString(unicode, ascii);
#else
	pfw(unicode, num);
#endif

    gdi_layer_clear(GDI_COLOR_WHITE);

    gui_set_font(&MMI_small_font);
    gui_set_text_color(gui_color(0, 0, 0));
	multiline(10, 10, unicode, SEP);

    gdi_layer_blt_previous(0, 0, UI_DEVICE_WIDTH-1, UI_DEVICE_HEIGHT-1);
}

###
0x04BFEECB  ff cc cc cc cc cc cc cc cc ee ee cc cc cc cc cc cc cc cc cc cc dd dd dd dd cc cc cc cc 70 f1
0x04BFEEEA  bf 04 94 c5 93 01 e0 ef bf 04 aa aa aa aa bb bb 00 00 cc 00 00 00 44 f2 bf 04 4c f2 bf 04 0b