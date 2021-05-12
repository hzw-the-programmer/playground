kal_uint8 soc_notify(void *p) {
    app_soc_notify_ind_struct *sni = (app_soc_notify_ind_struct*)p;
    kal_int32 r = 0;

    char *req = "GET /user/hezhiwen HTTP/1.1\nHost: 10.86.109.185:8080\n\n";
    char res[512] = {0};

    switch (sni->event_type) {
    case SOC_READ:
        r = soc_recv(sni->socket_id, res, sizeof(res), 0);
        break;

    case SOC_CONNECT:
        r = soc_send(sni->socket_id, req, strlen(req), 0);
        break;

    case SOC_CLOSE:
        break;

    case SOC_WRITE:
        break;

    default:
        break;
    }

    return 1;
}

void socket_test() {
    cbm_app_info_struct info = {1, 1, DTCNT_APPTYPE_EMAIL};
    kal_uint8 app_id = 0;

    srv_dtcnt_sim_type_enum sim_type = 0;
    cbm_sim_id_enum sim_id = CBM_SIM_ID_SIM1;

    kal_uint32 acct_id = 0;
    
    kal_int8 soc = 0;

    kal_uint8 option = 0;

    sockaddr_struct addr;
    kal_int8 result = 0;

    if (cbm_register_app_id_with_app_info(&info, &app_id) != CBM_OK) {
        return;
    }

    srv_dtcnt_get_sim_preference(&sim_type);
    if (sim_type <= SRV_DTCNT_SIM_TYPE_NONE || sim_type >= SRV_DTCNT_SIM_TYPE_TOTAL) {
        return;
    }
    sim_id = (sim_type - SRV_DTCNT_SIM_TYPE_1) + CBM_SIM_ID_SIM1;

    acct_id = cbm_encode_data_account_id(CBM_DEFAULT_ACCT_ID, sim_id, app_id, 0);

    soc = soc_create(SOC_PF_INET, SOC_SOCK_STREAM, 0, MOD_MMI, acct_id);

    option = 1;
    if (soc_setsockopt(soc, SOC_NBIO, &option, sizeof(option)) != SOC_SUCCESS) {
        return;
    }

    option = SOC_CONNECT | SOC_WRITE | SOC_READ | SOC_CLOSE;
    if (soc_setsockopt(soc, SOC_ASYNC, &option, sizeof(option)) != SOC_SUCCESS) {
        return;
    }

    mmi_frm_set_protocol_event_handler(MSG_ID_APP_SOC_NOTIFY_IND, soc_notify, 0);

    addr.sock_type = SOC_SOCK_STREAM;
    addr.addr_len = 4;
    addr.port = 8080;
    addr.addr[0] = 10;
    addr.addr[1] = 86;
    addr.addr[2] = 109;
    addr.addr[3] = 185;

    result = soc_connect(soc, &addr);
}
