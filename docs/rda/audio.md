#include "med_main.h"

static void record_cb(mdi_result res) {
    LOG("rcb:%d", res);
}

static void start_record() {
    INT d;
    U8 path[64] = {0};
    U16 path_w[64] = {0};
    mdi_result res;

    d = MMI_FS_GetDrive(FS_DRIVE_V_REMOVABLE, 1, FS_NO_ALT_DRIVE);
    sprintf(path, "%c:/r.amr", d);
    AnsiiToUnicodeString(path_w, path);
    res = MMI_FS_Delete(path_w);
    LOG("d:%d", res);
    mdi_audio_stop_all();
    res = mdi_audio_start_record(path_w, MED_TYPE_AMR, NULL, record_cb);
    //res = mdi_audio_start_record_with_quality(path_w, MED_TYPE_AMR, 0x04, NULL, record_cb);
    LOG("r:%s=%d", path, res);
}

static void play_cb(mdi_result res) {
    LOG("pcb:%d", res);
}

static void play() {
    INT d;
    U8 path[64] = {0};
    U16 path_w[64] = {0};
    mdi_result res;

    d = MMI_FS_GetDrive(FS_DRIVE_V_REMOVABLE, 1, FS_NO_ALT_DRIVE);
    sprintf(path, "%c:/r.amr", d);
    //sprintf(path, "%c:/Audio/052917240.amr", d);
    AnsiiToUnicodeString(path_w, path);

    /*
    res = mdi_audio_play_file_with_vol_path(
        path_w,
        DEVICE_AUDIO_PLAY_ONCE,
        NULL,
        play_cb,
        GetKeypadVolumeLevel(),
        MDI_DEVICE_SPEAKER2,
        0);
    */
    res = mdi_audio_play_file_portion_with_vol_path(
        path_w,
        0, 0,
        DEVICE_AUDIO_PLAY_ONCE,
        NULL,
        play_cb,
        GetKeypadVolumeLevel(),
        MDI_DEVICE_SPEAKER2);
    LOG("p:%s=%d", path, res);
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
    start_record();
}

static void key_3() {
    mdi_result res;
    res = mdi_audio_stop_record();
    LOG("sr:%d", res);
}

static void key_4() {
}

static void key_5() {
    play();
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
