D:\work\mtk6261d\plutommi\MtkApp\Video\VideoSrc\VdoRecApp.c
mmi_vdorec_lauch
mmi_vdorec_start_preview
D:\work\mtk6261d\plutommi\Service\MDI\MDISrc\mdi_video_rec.c
mdi_video_rec_preview_start

D:\work\mtk6261d\media\video\src\vid_api.c
media_vid_preview
vid_send_incoming_ilm_with_data
msg_send_ext_queue

execute_softkey_function

```
MoDIS.exe!mmi_vdorec_draw_softkey_string(unsigned short sk_str_id=153, unsigned int sk_draw_layer=0, unsigned char press=0, WGUI_SOFTKEY_ENUM type=MMI_LEFT_SOFTKEY, o_style_text_struct * pos=0x011055f0, mmi_frm_screen_rotate_enum rotate=MMI_FRM_SCREEN_ROTATE_0)  行16012	C
MoDIS.exe!mmi_vdorec_draw_softkey()  行16138 + 0x26 字节	C
MoDIS.exe!mmi_vdorec_draw_osd(unsigned int component=99)  行17815	C
MoDIS.exe!mmi_vdorec_entry_app_internal()  行11985 + 0x7 字节	C
MoDIS.exe!mmi_vdorec_lauch()  行5618	C
MoDIS.exe!execute_softkey_function(int k=1, WGUI_SOFTKEY_ENUM key=MMI_CENTER_SOFTKEY)  行1985 + 0x14 字节	C
MoDIS.exe!softkey_up(WGUI_SOFTKEY_ENUM key=MMI_CENTER_SOFTKEY)  行2557 + 0xb 字节	C
MoDIS.exe!center_softkey_up()  行2677 + 0x7 字节	C
MoDIS.exe!mmi_key_hdlr_proc(mmi_frm_key_evt_struct * evt=0x0d9ffd84)  行4283 + 0x5 字节	C
MoDIS.exe!mmi_frm_send_event(_mmi_event_struct * evt=0x0d9ffd84, int (_mmi_event_struct *)* proc=0x00540fd0, void * user_data=0x00000000)  行1270 + 0x9 字节	C
MoDIS.exe!process_key_event_routing(mmi_frm_key_evt_struct * key_evt_p=0x0d9ffd84)  行4889 + 0x1f 字节	C
MoDIS.exe!mmi_key_handle(mmi_key_evt_struct * mmi_evt_p=0x0d9ffe20)  行4657 + 0x9 字节	C
MoDIS.exe!dev_key_handle(dev_key_evt_struct * dev_evt_p=0x0d9ffe40)  行4549 + 0x9 字节	C
MoDIS.exe!mmi_frm_key_handle(void * paraBuff=0x08747984)  行3433 + 0x9 字节	C
MoDIS.exe!mmi_frm_execute_current_protocol_handler(unsigned short eventID=14829, void * MsgStruct=0x08747984, int mod_src=25, void * Message=0x0d9ffefc)  行682 + 0x11 字节	C
MoDIS.exe!MMI_task(task_entry_struct * entry_param=0x08763904)  行2745 + 0x16 字节	C
MoDIS.exe!_osc_platform_thread_create()  + 0x2a0 字节	
msvcr90d.dll!_callthreadstartex()  行348 + 0xf 字节	C
msvcr90d.dll!_threadstartex(void * ptd=0x0cb436b8)  行331	C
kernel32.dll!76246359() 	
[下面的框架可能不正确和/或缺失，没有为 kernel32.dll 加载符号]	
ntdll.dll!77e587a4() 	
ntdll.dll!77e58774() 	
```

D:\work\mtk6261d\media\common\src\med_main.c
med_task_main
med_main
D:\work\mtk6261d\media\video\src\vid_main.c
D:\work\mtk6261d\media\video\src\vid_recorder_msg_handler.c
vid_preview_req_hdlr
vid_recorder_init
vid_construct_recorder
vid_alloc_ext_mem_impl

kal_retrieve_eg_events // block here
kal_set_eg_events // unblock pre line
