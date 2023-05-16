#define FN L"d:/corrupt.jpg"
static void decode_result_cb(S32 result)
{
    if (result < 0)
    {
        MMI_FS_Delete(FN);
    }
}

static void lsk() {
}

static void key_1() {
}

static void key_2() {
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

static void exit_app() {
}

static void enter_app() {
	EntryNewScreen(666, exit_app, enter_app, NULL);
    // no history
    // ExecuteCurrExitHandler_Ext -> GenericExitScreen
    //EntryNewScreen(666, exit_app, NULL, NULL);

    ShowCategory222Screen(
        STR_GLOBAL_VIEW, 0,
        0, 0,
        STR_GLOBAL_BACK, 0,
        GDI_COLOR_BLACK, NULL,
        FN,
        FALSE,
        decode_result_cb);

    SetRightSoftkeyFunction(GoBackHistory, KEY_EVENT_UP);
}