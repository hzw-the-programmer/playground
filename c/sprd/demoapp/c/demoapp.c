#include "mmk_app.h"
#include "IN_Message.h"
#include "socket_types.h"
#include "window_parse.h"
#include "demoapp.h"
#include "log.h"

MMI_APPLICATION_T g_demoapp;

static MMI_RESULT_E processMsg(PWND app_ptr, uint16 msg_id, DPARAM param);

void demoapp_init() {
    g_demoapp.ProcessMsg = processMsg;
}

static MMI_RESULT_E processMsg(PWND app_ptr, uint16 msg_id, DPARAM param) {
    switch(msg_id)
	{
	    case SOCKET_ASYNC_GETHOSTBYNAME_CNF:
        {
            ASYNC_GETHOSTBYNAME_CNF_SIG_T *dns_ind = (ASYNC_GETHOSTBYNAME_CNF_SIG_T*)param;
            return MMI_RESULT_TRUE;
        }

	    default:
            return MMI_RESULT_FALSE;
    }
}

static MMI_RESULT_E handle_main_win_msg(
    MMI_WIN_ID_T win_id,
    MMI_MESSAGE_ID_E msg_id,
    DPARAM param) {
    MMI_RESULT_E result = MMI_RESULT_TRUE;

    LOG("handle_main_win_msg: %x", msg_id);

    switch(msg_id) {
        case MSG_APP_CANCEL:
            LOG_DEINIT();
            MMK_CloseWin(win_id);
            break;

        case MSG_KEYUP_1:
            break;

        case MSG_KEYUP_2:
            break;

        case MSG_KEYUP_3:
            break;

        case MSG_KEYUP_4:
            break;

        case MSG_KEYUP_5:
            break;

        case MSG_KEYUP_6:
            break;

        case MSG_TIMER:
            break;

        default:
            result = MMI_RESULT_FALSE;
            break;
    }

    return result;
}

WINDOW_TABLE(MMIDEMOAPP_WIN_TAB) = {
   WIN_ID(MMIDEMOAPP_WIN_ID_MAIN),
   WIN_FUNC((uint32)handle_main_win_msg),
   CREATE_RICHTEXT_CTRL(MMIDEMOAPP_CTRL_ID_RICHTEXT),
   END_WIN,
};

void demoapp_entry() {
    LOG_INIT();
    MMK_CreateWin((uint32*)MMIDEMOAPP_WIN_TAB, PNULL);
}
