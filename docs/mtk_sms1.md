```
MoDIS.exe!srv_sms_send_msg_callback(void * sms_handle=0x0a23d56c, kal_bool result=KAL_FALSE, srv_sms_cause_enum cause=2172)  行1746 + 0x9 字节	C
MoDIS.exe!srv_sms_send_sms_rsp(srv_sms_disp_cb_struct * cb_data=0x0fd4fe24)  行1661 + 0x11 字节	C
MoDIS.exe!srv_sms_handle_sms_send_rsp(srv_sms_sim_enum sim_id=SRV_SMS_SIM_1, unsigned int msg_id=14591, void * inMsg=0x0a23bd58)  行1102 + 0x1d 字节	C
MoDIS.exe!srv_sms_disp_send_sms_rsp(void * inMsg=0x0a23bd58, int mod_dest=25, void * Message=0x0fd4fefc)  行1051 + 0x11 字节	C
MoDIS.exe!mmi_frm_execute_current_protocol_handler(unsigned short eventID=14591, void * MsgStruct=0x0a23bd58, int mod_src=25, void * Message=0x0fd4fefc)  行670 + 0x11 字节	C
MoDIS.exe!MMI_task(task_entry_struct * entry_param=0x0a26214c)  行2711 + 0x16 字节	C
MoDIS.exe!_osc_platform_thread_create()  + 0x2a0 字节	
msvcr90d.dll!_callthreadstartex()  行348 + 0xf 字节	C
msvcr90d.dll!_threadstartex(void * ptd=0x0ec14130)  行331	C
kernel32.dll!77066359() 	
[下面的框架可能不正确和/或缺失，没有为 kernel32.dll 加载符号]	
ntdll.dll!77567c24() 	
ntdll.dll!77567bf4() 	
```

```
srv_sms_send_msg
```
