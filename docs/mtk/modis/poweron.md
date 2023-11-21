mmi_fmrdo_earphone_notify_hdlr(_mmi_event_struct * event=0x03d29bf0)
execute_evt_cb(unsigned int tbl_idx=281, _mmi_event_struct * evt=0x03d29bf0)
emit_evt_cb(unsigned short evt_id=26254, unsigned int tbl_idx=281, _mmi_event_struct * evt=0x03d29bf0, int (_mmi_event_struct *)* call_back=0x00000000, void * user_data=0x00000000)
process_cb_table(unsigned short evt_id=26254, int (unsigned short, unsigned int, _mmi_event_struct *, int (_mmi_event_struct *)*, void *)* cb_proc=0x00df7d40, _mmi_event_struct * evt=0x03d29bf0, int (_mmi_event_struct *)* call_back=0x00000000, void * user_data=0x00000000)
mmi_frm_cb_emit_event(_mmi_event_struct * evt=0x03d29bf0)
mmi_frm_invoke_post_event()
MMI_task(task_entry_struct * entry_param=0x0a251dc4)
_osc_platform_thread_create()
_callthreadstartex()
_threadstartex(void * ptd=0x0ecb3d30)
kernel32.dll!7512fcc9()
[下面的框架可能不正确和/或缺失，没有为 kernel32.dll 加载符号]
ntdll.dll!77277c6e()
ntdll.dll!77277c3e()

mmi_gpio_handle_earphone_plug_out
srv_prof_notify_hdlr
mdi_audio_cb_evt_hdlr

mmi_bootup_set_scenario_for_interrupt

mmi_bootup_init_apps
srv_sms_init
srv_phb_init_protocol
vm_mre_init
mmi_phnset_init
mmi_bt_init
mmi_wap_prof_app_init
srv_wap_push_inbox_init
srv_reminder_init
srv_gpio_setting_init

mmi_bootup_entry_dummy_base_screen