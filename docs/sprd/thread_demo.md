#if 1
static void log_init() {
}

static void log_deinit() {
}
#endif

#if 1
void thread_entry(uint32 argc, void *argv)
{
    LOG("thread_entry: %d", SCI_IdentifyThread());
}

void thread_test()
{
    BLOCK_ID tid;

    tid = SCI_CreateThread("tn", "qn", thread_entry, 0, NULL, 2*1024, 10, 100, SCI_PREEMPT, SCI_AUTO_START);
    LOG("SCI_CreateThread: %d", tid);
}
#endif

MMI_RESULT_E HandleAppWinMsg(
   MMI_WIN_ID_T        win_id, 
   MMI_MESSAGE_ID_E    msg_id, 
   DPARAM              param
   )
{
    switch(msg_id)
    {
        case MSG_FULL_PAINT:
            {
                GUI_LCD_DEV_INFO dev = {GUI_MAIN_LCD_ID, GUI_BLOCK_MAIN};
                UILAYER_FillColor(&dev, MMI_YELLOW_COLOR);
                break;
            }

        case MSG_APP_CANCEL:
            log_deinit();
            MMK_CloseWin(win_id);
            break;

        case MSG_KEYUP_1:
            thread_test();
            break;
    }
}

WINDOW_TABLE(MMIAPP_WIN_TAB) =
{
    WIN_FUNC((uint32)HandleAppWinMsg),
    END_WIN,
};

static int run_tests() {
    log_init();
    LOG("run_tests: %d", SCI_IdentifyThread());
    MMK_CreateWin((uint32*)MMIAPP_WIN_TAB, PNULL);
    return 1;
}
