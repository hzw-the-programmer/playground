#include "mmk_app.h"
#include "IN_Message.h"
#include "socket_types.h"
#include "socket_api.h"
#include "window_parse.h"
#include "demoapp.h"
#include "log.h"
#include "compat.h"

MMI_APPLICATION_T g_demoapp;

static MMI_RESULT_E processMsg(PWND app_ptr, uint16 msg_id, DPARAM param);

#if 1 // pdp
static uint32 g_netid;
static void pdp_active();
#endif // pdp

#if 1 // dns
#define DOMAIN "localhost"
#define PORT 443
static TCPIP_HOST_HANDLE g_dns_request_id;
static uint32 g_ip;
static void dns(const char *domain);
static MMI_RESULT_E dns_cb(PWND app_ptr, uint16 msg_id, DPARAM param);
#endif // dns

#if 1 // socket
static TCPIP_SOCKET_T g_socket;
static void connect(uint32 addr, uint16 port);
static MMI_RESULT_E socket_cb(PWND app_ptr, uint16 msg_id, DPARAM param);
#endif // socket

#if defined(PICOTLS_SUPPORT)
#include "picotls.h"
#include "picotls/minicrypto.h"

typedef struct {
    ptls_t *tls;
    ptls_buffer_t buf;
} ptls_ctx_t;

static ptls_ctx_t g_ptls_ctx;

static void picotls_init();
static void ptls_buffer_shift(ptls_buffer_t *buf, size_t len);
#endif // PICOTLS_SUPPORT

void demoapp_init() {
    g_demoapp.ProcessMsg = processMsg;
}

static MMI_RESULT_E processMsg(PWND app_ptr, uint16 msg_id, DPARAM param) {
    LOG("processMsg: msg_id=%d", msg_id);

    switch(msg_id) {
	    case SOCKET_ASYNC_GETHOSTBYNAME_CNF:
            return dns_cb(app_ptr, msg_id, param);

		case SOCKET_CONNECT_EVENT_IND:
		case SOCKET_READ_EVENT_IND:
		case SOCKET_WRITE_EVENT_IND:
        case SOCKET_CONNECTION_CLOSE_EVENT_IND:
            return socket_cb(app_ptr, msg_id, param);

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
            pdp_active();
            break;

        case MSG_KEYUP_2:
            dns(DOMAIN);
            break;

        case MSG_KEYUP_3:
            connect(g_ip, PORT);
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
#if defined(PICOTLS_SUPPORT)
    picotls_test();
    picotls_init();
#endif
    MMK_CreateWin((uint32*)MMIDEMOAPP_WIN_TAB, PNULL);
}

#if 1 // pdp
// APP_Task
// MMK_DispatchExtSig
// DispatchSysSig
// HandlePdpPsMsg
// FsmDispatch
// FsmActiving
// Callback
static void pdp_active_cb(MMIPDP_CNF_INFO_T *msg_ptr) {
    LOG("pdp_active_cb: app_handler=%d, nsapi=%d, msg_id=%d, result=%d",
        msg_ptr->app_handler, msg_ptr->nsapi, msg_ptr->msg_id, msg_ptr->result);

    if (msg_ptr->app_handler != MMI_MODULE_DEMOAPP) {
        LOG("app_handler: want=%x, got=%x", MMI_MODULE_DEMOAPP, msg_ptr->app_handler);
        return;
    }

    switch(msg_ptr->msg_id) {
        case MMIPDP_ACTIVE_CNF:
            if(msg_ptr->result == MMIPDP_RESULT_SUCC) {
                LOG("MMIPDP_ACTIVE_CNF: netid=%d", msg_ptr->nsapi);
                g_netid = msg_ptr->nsapi;
            }
            break;
        default:
            break;
    }
}

static void pdp_active() {
    MN_DUAL_SYS_E sim;
	uint8 setting_index;
	MMICONNECTION_LINKSETTING_DETAIL_T *setting_item_ptr;
    MMIPDP_ACTIVE_INFO_T app_info = {0};

    for (sim = MN_DUAL_SYS_1; sim < MN_DUAL_SYS_MAX; sim++) {
        if (MMIPHONE_IsSimOk(sim)) {
            break;
        }
    }
    if (sim == MN_DUAL_SYS_MAX) {
        LOG("no sim usable");
        return;
    }

    if (!MMIAPIPHONE_IsNetworkAttached(sim)) {
        LOG("net not attached");
        return;
    }

    setting_index = MMIAPIBROWSER_GetNetSettingIndex(sim);
    setting_item_ptr = MMIAPICONNECTION_GetLinkSettingItemByIndex(sim, setting_index);
    if (setting_item_ptr == PNULL)
    {
        LOG("no setting");
        return;
    }

    LOG("apn=\"%s\", user_name=\"%s\", password=\"%s\", ip_type=%d, auth_type=%d",
        setting_item_ptr->apn,
        setting_item_ptr->username, setting_item_ptr->password,
        setting_item_ptr->ip_type,
        setting_item_ptr->auth_type);

    app_info.dual_sys = sim;
    app_info.app_handler = MMI_MODULE_DEMOAPP;
    app_info.auth_type = setting_item_ptr->auth_type;
    app_info.apn_ptr = (char*)setting_item_ptr->apn;
    app_info.user_name_ptr = (char*)setting_item_ptr->username;
    app_info.psw_ptr = (char*)setting_item_ptr->password;
    //app_info.ps_service_type = STREAMING_E;
    //app_info.ps_service_rat = MN_UNSPECIFIED;
    //app_info.ps_interface = MMIPDP_INTERFACE_GPRS;
    //app_info.storage = MN_GPRS_STORAGE_ALL;
    app_info.priority = 3;
    app_info.handle_msg_callback = pdp_active_cb;
#ifdef IPVERSION_SUPPORT_V4_V6 
    //app_info.ip_type = setting_item_ptr->ip_type;  
#endif

    LOG("MMIAPIPDP_Active: %d", MMIAPIPDP_Active(&app_info));
}
#endif // pdp

#if 1 // dns
static void dns(const char *domain) {
    g_dns_request_id = sci_async_gethostbyname(domain, SCI_IdentifyThread(), 20000, g_netid);
    LOG("dns: %s, request_id=%d", domain, g_dns_request_id);
}

static MMI_RESULT_E dns_cb(PWND app_ptr, uint16 msg_id, DPARAM param) {
    ASYNC_GETHOSTBYNAME_CNF_SIG_T *dns_ind = (ASYNC_GETHOSTBYNAME_CNF_SIG_T*)param;

    LOG("dns_cb: error_code=%d, request_id=%d, netid=%d",
        dns_ind->error_code, dns_ind->request_id, dns_ind->netid);

    if (dns_ind->request_id != g_dns_request_id) {
        LOG("not our request: want=%d", g_dns_request_id);
        return MMI_RESULT_FALSE;
    }

    LOG("addrtype=%d, length=%d, cntv4=%d, cntv6=%d", dns_ind->hostent.h_addrtype,
        dns_ind->hostent.h_length,
        dns_ind->hostent.h_cntv4, dns_ind->hostent.h_cntv6);

    if (dns_ind->hostent.h_cntv4) {
        uint32 a, b, c, d;

        a = dns_ind->hostent.h_addr_list[0][0];
        b = dns_ind->hostent.h_addr_list[0][1];
        c = dns_ind->hostent.h_addr_list[0][2];
        d = dns_ind->hostent.h_addr_list[0][3];

        LOG("addr: %d.%d.%d.%d", a, b, c, d);
        LOG("addr: %x.%x.%x.%x", a, b, c, d);
        g_ip = a | (b<<8) | (c<<16) | (d<<24);
        LOG("addr: %x", g_ip);
    }
}
#endif // dns

#if 1 // socket
static TCPIP_SOCKET_T socket_open() {
    TCPIP_SOCKET_T sock;
    int32 result;

    sock = sci_sock_socket(AF_INET, SOCK_STREAM, 0, g_netid);
    LOG("sci_sock_socket: sock=%d, netid=%d, err=%d", sock, g_netid, sci_sock_errno(sock));
    if (sock == -1) {
        return sock;
    }

    result = sci_sock_setsockopt(sock, SO_NBIO, NULL);
    LOG("sci_sock_setsockopt: %d", result);
    result = sci_sock_asyncselect(sock, SCI_IdentifyThread(), AS_CONNECT|AS_READ|AS_WRITE|AS_CLOSE); 
    LOG("sci_sock_asyncselect: %d", result);

    return sock;
}

static void connect(uint32 ip, uint16 port) {
    TCPIP_SOCKET_T sock;
    struct sci_sockaddr addr = {0};
    int32 result;

    sock = socket_open();
    if (sock == -1) {
        return;
    }

    addr.ip_addr = ip;
    addr.port = htons(port);
    addr.family = AF_INET;

    result = sci_sock_connect(sock, &addr, sizeof(addr));
    LOG("sci_sock_connect: %d", result);
    if (result < 0) {
        result = sci_sock_errno(sock);
        LOG("sci_sock_errno: %d", result);
    }

    g_socket = sock;
}

static int send(TCPIP_SOCKET_T soc, ptls_buffer_t *buf) {
    int ret;

    while (buf->off != 0) {
        ret = sci_sock_send(soc, buf->base, buf->off, 0);
        LOG("sci_sock_send: want=%d, got=%d", buf->off, ret);
        if (ret <= 0) {
            break;
        }
        ptls_buffer_shift(buf, ret);
    }

    return ret;
}

static MMI_RESULT_E socket_cb(PWND app_ptr, uint16 msg_id, DPARAM param) {
    int ret;

    switch(msg_id) {
		case SOCKET_CONNECT_EVENT_IND:
            LOG("SOCKET_CONNECT_EVENT_IND");
            send(g_socket, &g_ptls_ctx.buf);
            break;
		case SOCKET_READ_EVENT_IND:
            {
                uint8_t data[1024];

                LOG("SOCKET_READ_EVENT_IND");
                while (1) {
                    uint32_t len, off, consumed;

                    ret = sci_sock_recv(g_socket, data, sizeof(data), 0);
                    LOG("sci_sock_recv: %d", ret);
                    if (ret <= 0) {
                        break;
                    }

                    len = ret;
                    off = 0;
                    while ((consumed = len - off) > 0) {
                        if (!ptls_handshake_is_complete(g_ptls_ctx.tls)) {
                            ret = ptls_handshake(g_ptls_ctx.tls, &g_ptls_ctx.buf, data + off, &consumed, NULL);
                            LOG("ptls_handshake: ret=0x%x, consumed=%d", ret, consumed);
                            if (ret == 0) {
                                uint8_t *req = "GET / HTTP/1.1\r\n\r\n";
                                ptls_send(g_ptls_ctx.tls, &g_ptls_ctx.buf, req, strlen(req));
                                send(g_socket, &g_ptls_ctx.buf);
                            }
                        } else {
                            ret = ptls_receive(g_ptls_ctx.tls, &g_ptls_ctx.buf, data + off, &consumed);
                            if (ret == 0) {
                                if (g_ptls_ctx.buf.off) {
                                    slice_t slice = slice_new(g_ptls_ctx.buf.base, g_ptls_ctx.buf.off);
                                    LOG("%S", &slice);
                                }
                            }
                        }

                        off += consumed;
                    }
                }
                break;
            }
		case SOCKET_WRITE_EVENT_IND:
            LOG("SOCKET_WRITE_EVENT_IND");
            send(g_socket, &g_ptls_ctx.buf);
            break;
        case SOCKET_CONNECTION_CLOSE_EVENT_IND:
            LOG("SOCKET_CONNECTION_CLOSE_EVENT_IND");
            break;
    }
}
#endif // socket

#if defined(PICOTLS_SUPPORT)
static void picotls_init() {
    ptls_context_t *ctx;
    ptls_t* tls;
    ptls_buffer_t buf;
    int ret;

    ctx = malloc(sizeof(*ctx));
    if (!ctx) {
        LOG("create ptls_context_t failed");
        return;
    }

	ctx->get_time = &ptls_get_time;
	ctx->random_bytes = ptls_minicrypto_random_bytes;
	ctx->key_exchanges = ptls_minicrypto_key_exchanges;
	ctx->cipher_suites = ptls_minicrypto_cipher_suites;

    tls = ptls_client_new(ctx);
    if (!tls) {
        LOG("create ptls_t failed");
        free(ctx);
        return;
    }

    ptls_buffer_init(&buf, "", 0);
    ret = ptls_handshake(tls, &buf, NULL, 0, NULL);
    if (ret != PTLS_ERROR_IN_PROGRESS) {
        ptls_free(tls);
        free(ctx);
        return;
    }

    g_ptls_ctx.tls = tls;
    g_ptls_ctx.buf = buf;
}
#endif // PICOTLS_SUPPORT
