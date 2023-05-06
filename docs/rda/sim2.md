#include "mtpnp_ad_resdef.h"
extern const U16 gIndexIconsImageList[];

#define SELECT_SIM_SCR_ID 1024

static U8 g_selected;
static U8 g_index_map[MMI_SIM_NUMBER];

static void draw(const char *text) {
    gdi_layer_clear(GDI_COLOR_WHITE);

    gui_set_font(&MMI_small_font);
    gui_set_text_color(gui_color(0, 0, 0));
	gui_move_text_cursor(50, 50);
	gui_printf("%s", text);

    gdi_layer_blt_previous(0, 0, UI_DEVICE_WIDTH-1, UI_DEVICE_HEIGHT-1);
}

void select_sim(void)
{
	LOG("selected sim: %d", g_selected);
}

static void select_sim_highlight_handler(int index)
{
	g_selected = g_index_map[index];
	SetLeftSoftkeyFunction(select_sim, KEY_EVENT_UP);
}

static void select_sim_entry()
{
	U8* guiBuffer;
	S32 i;
	S32 count = 0;
	U16 item_texts[MMI_SIM_NUMBER];

	for(i = 0; i < MMI_SIM_NUMBER; i++)
	{
		if(MTPNP_PFAL_Is_Card_Usable(i))
		{
			item_texts[count] = STRING_MTPNP_SIM1 + i;
			g_index_map[count] = i;
			count++;
		}
	}

	EntryNewScreen(SELECT_SIM_SCR_ID, NULL, select_sim_entry, NULL);
	guiBuffer = GetCurrGuiBuffer(SELECT_SIM_SCR_ID);
	RegisterHighlightHandler(select_sim_highlight_handler);

	ShowCategory15Screen(STR_GLOBAL_OPTIONS, 0,
		STR_GLOBAL_OK, 0, STR_GLOBAL_BACK, 0,
		count, item_texts, gIndexIconsImageList, 
		LIST_MENU, 0, guiBuffer);

	SetRightSoftkeyFunction(GoBackHistory, KEY_EVENT_UP);
}

static void lsk() {
}

static void key_1() {
}

static void key_2() {
	S32 i;

	LOG("usable %d", MTPNP_AD_Get_UsableSide_Number());

	for(i = 0; i < MMI_SIM_NUMBER; i++)
	{
		if (MTPNP_PFAL_Is_Card_Usable(i))
		{
			LOG("sim %d is usable", i);
		}
		else
		{
			LOG("sim %d is not usable", i);
		}
	}
}

static void key_3() {
	U8 count = MTPNP_AD_Get_UsableSide_Number();

	if (count > 1)
	{
		select_sim_entry();
	}
	else if (count == 1)
	{
		g_selected = MTPNP_AD_GET_UsableSide_Index();
		LOG("one sim: %d", g_selected);
	}
	else
	{
		LOG("no sim");
	}
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
