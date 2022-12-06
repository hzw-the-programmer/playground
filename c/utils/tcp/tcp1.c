#include "soc_api.h"
#include "cbm_api.h"
#include "mmi_frm_events_gprot.h"
#include "kal_debug.h"

#define HOST_MAX_LEN 64

#if 1 // account
static kal_uint8 g_app_id;
static kal_uint32 g_account_id;

static void account_init() {
    cbm_app_info_struct app_info = {0};

	app_info.app_str_id = 9277;
	app_info.app_icon_id = 9266;
	app_info.app_type = DTCNT_APPTYPE_MRE_NET;

    cbm_register_app_id_with_app_info(
        &app_info, &g_app_id);

    g_account_id = cbm_encode_data_account_id(
        CBM_DEFAULT_ACCT_ID, CBM_SIM_ID_SIM1,
        g_app_id, KAL_FALSE);
}
#endif

#if 1 // dns
#define DNS_REQ_MAX_LEN 11
#define DNS_ENTRY_MAX_LEN 7

typedef struct dns_req_s dns_req_t;
typedef struct dns_entry_s dns_entry_t;
typedef struct dns_ctx_s dns_ctx_t;

struct dns_req_s {
    char host[HOST_MAX_LEN];
    kal_int32 id;
};
struct dns_entry_s {
    char host[HOST_MAX_LEN];
    sockaddr_struct addr;
};
struct dns_ctx_s {
    dns_req_t reqs[DNS_REQ_MAX_LEN];
    kal_uint8 req_len;
    dns_entry_t entries[DNS_ENTRY_MAX_LEN];
    kal_uint8 entry_len;
};

static dns_ctx_t g_dns_ctx;

static dns_req_t* dns_req_find(
    dns_ctx_t *ctx,
    kal_int32 id) {
    int i;

    for (i = 0; i < ctx->req_len; i++) {
        if (ctx->reqs[i].id == id) {
            return &ctx->reqs[i];
        }
    }

    return NULL;
}

static dns_req_t* dns_req_new(
    dns_ctx_t *ctx) {
    if (ctx->req_len >= DNS_REQ_MAX_LEN) {
        return NULL;
    }

    return &ctx->reqs[ctx->req_len++];
}

static void dns_req_rm(
    dns_ctx_t *ctx,
    dns_req_t *req) {
    *req = ctx->reqs[--ctx->req_len];
}

static dns_entry_t* dns_entry_find(
    dns_ctx_t *ctx,
    const char *host) {
    int i;

    for (i = 0; i < ctx->entry_len; i++) {
        if (!strcmp(ctx->entries[i].host, host)) {
            return &ctx->entries[i];
        }
    }

    return NULL;
}

static dns_entry_t* dns_entry_new(
    dns_ctx_t *ctx) {
    if (ctx->entry_len >= DNS_ENTRY_MAX_LEN) {
        return NULL;
    }

    return &ctx->entries[ctx->entry_len++];
}

static kal_uint8 dns_query_cb(
    void *param)
{
    dns_ctx_t *ctx = &g_dns_ctx;
    dns_req_t *req;
    dns_entry_t *entry = {0};
    app_soc_get_host_by_name_ind_struct* msg =
        (app_soc_get_host_by_name_ind_struct*)param;

    if (!msg || !msg->result
        || msg->access_id != g_app_id)
    {
        return KAL_FALSE;
    }

    req = dns_req_find(ctx, msg->request_id);
    if (!req) {
        return KAL_FALSE;
    }

    entry = dns_entry_find(ctx, req->host);
    if (!entry) {
        entry = dns_entry_new(ctx);
        if (!entry) {
            return KAL_FALSE;
        }
    }

    strcpy(entry->host, req->host);
    memcpy(entry->addr.addr, msg->addr, msg->addr_len);
    entry->addr.addr_len = msg->addr_len;
    entry->addr.port = 80;

    dns_req_rm(ctx, req);

    return KAL_TRUE;
}

void dns_query(
    const char *host)
{
    dns_ctx_t *ctx = &g_dns_ctx;
    static kal_uint8 request_id = 0;
    kal_uint8 access_id = g_app_id;
    dns_req_t *req;
    kal_int8 ret;
    kal_uint8 buf[4];
    kal_uint8 len = sizeof(buf);
    app_soc_get_host_by_name_ind_struct msg = {0};

    req = dns_req_new(ctx);
    if (!req) {
        return;
    }

    req->id = request_id++;
    strcpy(req->host, host);

    ret = soc_gethostbyname(KAL_FALSE,
             MOD_MMI, req->id, req->host, 
             buf, &len,
             access_id, g_account_id);

    if (ret == SOC_WOULDBLOCK)
    {
        mmi_frm_set_protocol_event_handler(
            MSG_ID_APP_SOC_GET_HOST_BY_NAME_IND,
            dns_query_cb, MMI_TRUE);
        return;
    }

    msg.request_id = req->id;
    msg.access_id = access_id;
    msg.result = KAL_TRUE;
    memcpy(msg.addr, buf, len);
    msg.addr_len = len;
    if (ret != SOC_SUCCESS)
    {        
        msg.result = KAL_FALSE;
    }

    dns_query_cb(&msg);
}
#endif

#if 1 // conn
#define CONN_MAX_LEN 3

typedef struct conn_if_s conn_if_t;
typedef struct conn_s conn_t;
typedef void (*conn_cb)(conn_t*);
typedef struct conn_ctx_s conn_ctx_t;

struct conn_if_s {
    conn_cb read_cb;
    conn_cb write_cb;
};
struct conn_s {
    kal_int8 soc;
    conn_if_t iface;
};
struct conn_ctx_s {
    conn_t conns[CONN_MAX_LEN];
    kal_uint8 len;
};

static conn_ctx_t g_conn_ctx;

static conn_t* find_conn(kal_int8 soc) {
    conn_ctx_t *ctx = &g_conn_ctx;
    int i;

    for (i = 0; i < ctx->len; i++) {
        if (ctx->conns[i].soc == soc) {
            return &ctx->conns[i];
        }
    }

    return NULL;
}

void conn_notify(void *param) {
    app_soc_notify_ind_struct *msg =
        (app_soc_notify_ind_struct*)param;
    conn_t *conn;

    conn = find_conn(msg->socket_id);
	if (!conn) {
		return;
	}
	
    switch (msg->event_type) 
    {
        case SOC_READ:
            break;
        case SOC_WRITE:
            break;
        case SOC_CONNECT:
			break;

        case SOC_CLOSE:
            break;            
    }
}

static conn_t* connect(
    sockaddr_struct *addr) {
    kal_int8 soc, ret;
    kal_uint8 option;

    soc = soc_create(SOC_PF_INET,
        SOC_SOCK_STREAM, 0,
        MOD_MMI, g_account_id);
    if (soc < 0) {
        return NULL;
    }

	option = SOC_READ | SOC_WRITE | SOC_CONNECT | SOC_CLOSE;
	ret = soc_setsockopt(soc, SOC_ASYNC,
        &option, sizeof(option));
    if (ret != SOC_SUCCESS) {
        soc_close(soc);
        return NULL;
    }
	option = 1;
	ret = soc_setsockopt(soc, SOC_NBIO,
        &option, sizeof(option));
    if (ret != SOC_SUCCESS) {
        soc_close(soc);
        return NULL;
    }

    ret = soc_connect(soc, addr);
    if (ret == SOC_SUCCESS
        || ret == SOC_WOULDBLOCK) {
        mmi_frm_set_protocol_event_handler(
            MSG_ID_APP_SOC_NOTIFY_IND,
            conn_notify, MMI_TRUE);
    }
}
#endif

#if 1 // http
#define PATH_MAX_LEN 64
#define METHOD_MAX_LEN 8
#define HTTP_MAX_REQS 5

typedef struct http_req_s http_req_t;
typedef struct http_ctx_s http_ctx_t;

struct http_req_s {
    char host[HOST_MAX_LEN];
    char path[PATH_MAX_LEN];
    char method[METHOD_MAX_LEN];
};
struct http_ctx_s {
    http_req_t reqs[HTTP_MAX_REQS];
    int len;
};

static http_ctx_t g_http_ctx;

void http_get(
    const char * host,
    const char * path) {
    http_ctx_t *ctx = &g_http_ctx;
    http_req_t req = {0};

    //assert(host);
    //assert(ctx->len < HTTP_MAX_REQS);

    strcpy(req.host, host);
    req.path[0] = '/';
    if (path) {
        strcpy(req.path, path);
    }
    strcpy(req.method, "GET");

    ctx->reqs[ctx->len++] = req;
}
#endif

void dns_query_test() {
    dns_query("www.163.com");
    dns_query("www.baidu.com");
}

void tcp1_main() {
    account_init();
    dns_query_test();
    //http_get("www.baidu.com", NULL);
}

typedef struct udp_conn_s udp_conn_t;
static udp_conn_t *g_udp_conn;
static kal_bool udp1_cb(void * data);
#define UDP_PORT 6789
#define UDP_MSG_SIZE (60*1024)

struct udp_conn_s {
    kal_int32 sock;
    kal_uint32 cap, r, w;
    kal_uint8 buf[1];
};

static void udp_conn_free(
    udp_conn_t *conn) {
    if (!conn) {
        return;
    }

    if (conn->sock >= 0) {
        soc_close(conn->sock);
    }

    free(conn);
}

static udp_conn_t* udp_conn_new(
    kal_uint32 cap,
    kal_uint16 port,
    kal_uint32 acct_id) {
    size_t offset, size;
    udp_conn_t *conn;
    kal_uint8 option;
    kal_int8 ret;
    sockaddr_struct addr = {0};

    offset = &((udp_conn_t*)0)->buf;
    //offset = ((udp_conn_t*)0)->buf;
    //offset = &((udp_conn_t*)0)->cap;
    //offset = ((udp_conn_t*)0)->cap;
    size = sizeof(*conn);
    size += cap - (size - offset);
    conn = malloc(size);
    if (!conn) {
        return NULL;
    }
    conn->cap = cap;
    conn->r = conn->w = 0;

    conn->sock = soc_create(
        SOC_PF_INET, SOC_SOCK_DGRAM, 0,
        MOD_MMI, acct_id);
    if (conn->sock < 0) {
        udp_conn_free(conn);
        return NULL;
    }

    option = KAL_TRUE;
    ret = soc_setsockopt(
        conn->sock, SOC_NBIO,
        &option, sizeof(option));
    if (ret != SOC_SUCCESS) {
        udp_conn_free(conn);
        return NULL;
    }

    option = SOC_READ | SOC_WRITE | SOC_CLOSE;
    ret = soc_setsockopt(
        conn->sock, SOC_ASYNC,
        &option, sizeof(option));
    if (ret != SOC_SUCCESS) {
        udp_conn_free(conn);
        return NULL;
    }

    addr.port = port;
    ret = soc_bind(conn->sock, &addr);
    if (ret != SOC_SUCCESS) {
        udp_conn_free(conn);
        return NULL;
    }

    ret = soc_getsockaddr(
        conn->sock, KAL_TRUE, &addr);
    if (ret != SOC_SUCCESS) {
        udp_conn_free(conn);
        return NULL;
    }
    kal_printf(
        "local address: %d.%d.%d.%d:%d\n",
        addr.addr[0], addr.addr[1],
        addr.addr[2], addr.addr[3],
        addr.port);

    mmi_frm_set_protocol_event_handler(
        MSG_ID_APP_SOC_NOTIFY_IND,
        (PsIntFuncPtr)udp1_cb, MMI_TRUE);

    return conn;
}

static void udp_conn_send_to(
    udp_conn_t *conn,
    kal_uint8 *buf, kal_uint32 len,
    sockaddr_struct *addr) {
}

static void udp_conn_send(
    udp_conn_t *conn) {
    static kal_uint8 count;
    sockaddr_struct addr = {0};
    kal_int32 ret;
    kal_uint8 buf[128] = {0};

    addr.addr[0] = 127;
    addr.addr[1] = 0;
    addr.addr[2] = 0;
    addr.addr[3] = 1;
    addr.port = UDP_PORT;

    kal_sprintf(buf, "count=%d", count++);

    ret = soc_sendto(conn->sock,
        buf, strlen(buf), 0, &addr);
    kal_printf("soc_sendto: %d\n", ret);
}

static void udp_conn_send_test(
    udp_conn_t *conn) {
    static kal_uint8 count;
    sockaddr_struct addr = {0};
    kal_int32 ret;
    kal_uint8 buf[128] = {0};

    addr.addr[0] = 127;
    addr.addr[1] = 0;
    addr.addr[2] = 0;
    addr.addr[3] = 1;
    addr.port = UDP_PORT;

    while (1) {
        kal_sprintf(buf, "count=%d", count++);

        ret = soc_sendto(conn->sock,
            buf, strlen(buf), 0, &addr);
        if (ret < 0) {
            break;
        }
    }
    kal_printf("soc_sendto: ret=%d, "
        "count=%d\n",
        ret, count);
}

static void udp_conn_send_test_1(
    udp_conn_t *conn) {
    kal_int8 sock = conn->sock;
    kal_uint32 size = 0;
    soc_getsockopt(
        sock, SOC_RCVBUF,
        &size, sizeof(size));
    kal_printf("RCVBUF: %u\n", size);
    soc_getsockopt(
        sock, SOC_SENDBUF,
        &size, sizeof(size));
    kal_printf("SENDBUF: %u\n", size);
}

static void udp_conn_send_test_2(
    udp_conn_t *conn) {
    sockaddr_struct addr = {0};
    static kal_uint8 *buf;
    //kal_uint32 size = 10*1024;
    //kal_uint32 size = 10*1024*1024;
    //kal_uint32 size = 1024*1024;
    kal_uint32 size = UDP_MSG_SIZE;
    kal_int32 ret;

    addr.addr[0] = 127;
    addr.addr[1] = 0;
    addr.addr[2] = 0;
    addr.addr[3] = 1;
    addr.port = UDP_PORT;

    if (!buf) {
        buf = malloc(size);
    }
    memset(buf, 'l', size);

    ret = soc_sendto(conn->sock,
        buf, size, 0, &addr);
    kal_printf("soc_sendto: %d\n", ret);
}

static void udp_conn_rcv_test(
    udp_conn_t *conn) {
    static kal_uint8 *buf;
    sockaddr_struct addr = {0};
    kal_int32 n;
    kal_bool printed = KAL_FALSE;

    if (!buf) {
        buf = malloc(UDP_MSG_SIZE+1);
    }
    memset(buf, 0, UDP_MSG_SIZE+1);

    while (1) {
        n = soc_recvfrom(
            g_udp_conn->sock,
            buf, UDP_MSG_SIZE, 0, &addr);
        if (!printed) {
            printed = KAL_TRUE;
            kal_printf("rcv from: %d.%d.%d.%d:%d\n",
                addr.addr[0], addr.addr[1],
                addr.addr[2], addr.addr[3],
                addr.port);
        }
        if (n < 0) {
            kal_printf("    finish: %d\n", n);
            break;
        }
        kal_printf("    total: %d\n", n);
    }
}

static kal_bool udp1_cb(void * data) {
    app_soc_notify_ind_struct *ind = (app_soc_notify_ind_struct*)data;

    kal_printf("udp1_cb: %d\n", ind->event_type);

    if (ind == NULL
        || ind->socket_id != g_udp_conn->sock) {
        return KAL_FALSE;
    }
        
    switch (ind->event_type) {
    case SOC_WRITE: {
            udp_conn_send(g_udp_conn);
        }
        break;
    case SOC_READ: {
            udp_conn_rcv_test(g_udp_conn);
        }
        break;
    case SOC_CLOSE: {
        }
        break;
    default: {
        }
        break;
    }

    return KAL_TRUE;
}

void udp1_main() {
    kal_printf("udp1_main\n");

    if (!g_udp_conn) {
        account_init();
        g_udp_conn = udp_conn_new(
            128, 0, g_account_id);
        if (!g_udp_conn) {
            return;
        }
        udp_conn_send(g_udp_conn);
    } else {
        //udp_conn_send(g_udp_conn);
        //udp_conn_send_test(g_udp_conn);
        //udp_conn_send_test_1(g_udp_conn);
        udp_conn_send_test_2(g_udp_conn);
    }
}

void examples_main() {
    //tcp1_main();
    udp1_main();
}

