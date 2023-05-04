#include "filemgr.h"

static void draw(const char *text) {
    gdi_layer_clear(GDI_COLOR_WHITE);

    gui_set_font(&MMI_small_font);
    gui_set_text_color(gui_color(0, 0, 0));
	gui_move_text_cursor(50, 50);
	gui_printf("%s", text);

    gdi_layer_blt_previous(0, 0, UI_DEVICE_WIDTH-1, UI_DEVICE_HEIGHT-1);
}

static void create_dir(const UINT8 *fmt)
{
	INT pdrive, tdrive;
	UINT8 path_asc[FMGR_PATH_CHAR_COUNT+1];
	UINT16 path_ucs2[FMGR_PATH_CHAR_COUNT+1];

	pdrive = MMI_FS_GetDrive( FS_DRIVE_V_NORMAL, 2, FS_DRIVE_I_SYSTEM | FS_DRIVE_V_NORMAL );
	tdrive = MMI_FS_GetDrive( FS_DRIVE_V_REMOVABLE, 1, FS_NO_ALT_DRIVE );

	sprintf(path_asc, fmt, tdrive);
	mmi_asc_to_ucs2(path_ucs2, path_asc);
	if (!MMI_FS_IsExist(path_ucs2))
	{
		LOG("dir does not exist:%s", path_asc);
		MMI_FS_CreateDir(path_ucs2);
	}
}

static void lsk() {
}

static void key_1() {
}

static void key_2() {
	create_dir("%c:/hzw");
	create_dir("%c:/hzw/inbox");
	create_dir("%c:/hzw/outbox");
}

static void key_3() {
	UINT8 asc[FMGR_PATH_CHAR_COUNT+1];
	UINT16 old[FMGR_PATH_CHAR_COUNT+1];
	UINT16 new[FMGR_PATH_CHAR_COUNT+1];

	sprintf(asc, "%c:/hzw/outbox/old.amr", MMI_FS_GetDrive(FS_DRIVE_V_REMOVABLE, 1, FS_NO_ALT_DRIVE));
	mmi_asc_to_ucs2(old, asc);
	sprintf(asc, "%c:/hzw/inbox/new.amr", MMI_FS_GetDrive(FS_DRIVE_V_REMOVABLE, 1, FS_NO_ALT_DRIVE));
	mmi_asc_to_ucs2(new, asc);

	MMI_FS_Rename(old, new);
}

static void key_4() {
}

static void key_5() {
	UINT8 asc[FMGR_PATH_CHAR_COUNT+1];
	UINT16 old[FMGR_PATH_CHAR_COUNT+1];
	UINT16 new[FMGR_PATH_CHAR_COUNT+1];

	sprintf(asc, "%c:/hzw/outbox/old.amr", MMI_FS_GetDrive(FS_DRIVE_V_REMOVABLE, 1, FS_NO_ALT_DRIVE));
	mmi_asc_to_ucs2(old, asc);
	sprintf(asc, "%c:/hzw/inbox/new.amr", MMI_FS_GetDrive(FS_DRIVE_V_REMOVABLE, 1, FS_NO_ALT_DRIVE));
	mmi_asc_to_ucs2(new, asc);

	//MMI_FS_Move(old, new, FS_MOVE_KILL | FS_MOVE_OVERWRITE, NULL, 0);
	MMI_FS_Move(old, new, FS_MOVE_KILL, NULL, 0);
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