#define TIMER_MSG 11

typedef struct sig {
    _SIGNAL_VARS
} sig_t;

BLOCK_ID g_tid;

static void thread_entry(uint32 argc, void *argv) {
    xSignalHeaderRec *sig_ptr = PNULL;

    g_tid = SCI_IdentifyThread();
    LOG("thread_entry: tid=%d", g_tid);

    while (1) {
        sig_ptr = SCI_GetSignal(g_tid);   
        if (sig_ptr == PNULL) {
            break;
        }
        switch(sig_ptr->SignalCode) {
            case TIMER_MSG:
                LOG("thread_entry: timer_msg: tid=%d", sig_ptr->Sender);
                break;
        }
        free(sig_ptr);
    }
}

static void thread_test() {
    BLOCK_ID tid;

    tid = SCI_CreateThread("tn", "qn", thread_entry, 0, NULL, 2*1024, 10, 100, SCI_PREEMPT, SCI_AUTO_START);
    LOG("SCI_CreateThread: %d", tid);
}

static void timer_cb(uint32 msg) {
    sig_t *sig;

    sig = malloc(sizeof(sig_t));
    sig->Sender = SCI_IdentifyThread();
    sig->SignalCode = msg;
    sig->SignalSize = sizeof(sig_t);
    SCI_SendSignal((xSignalHeader)(sig), g_tid);
}

static void timer_test() {
    LOG("timer_test: tid=%d", SCI_IdentifyThread());
    SCI_CreateTimer("timer", timer_cb, TIMER_MSG, 1000, SCI_AUTO_ACTIVATE);
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

        case MSG_KEYUP_2:
            timer_test();
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