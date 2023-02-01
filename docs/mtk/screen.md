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

static void enter_app()
{
	//EntryNewScreen(666, exit_app, enter_app, NULL);
	// no history
	// ExecuteCurrExitHandler_Ext -> GenericExitScreen
	EntryNewScreen(666, exit_app, NULL, NULL);
	gdi_layer_clear(GDI_COLOR_WHITE);
	SetLeftSoftkeyFunction(lsk, KEY_EVENT_UP);
	SetRightSoftkeyFunction(GoBackHistory, KEY_EVENT_UP);
	gdi_layer_blt_previous(0, 0, UI_DEVICE_WIDTH-1, UI_DEVICE_HEIGHT-1);
}
