```
MMISMS_NV_ReadSmsItem
NVSMS_Task
SCI_CreateThread(4)
_tx_thread_shell_entry
_tx_win32_thread_entry
```

```
MMISMS_InitForPS
MMIAPISMS_InitForPS
APP_Task
```

MS_Code\MS_MMI\source\mmi_app\app\sms\c\mmismsapp_main.c
HandlePsMsg开机收到的消息
```
APP_MN_READ_SMS_CNF(19)
APP_MN_SMS_READY_IND
APP_MN_READ_SMS_CNF
APP_MN_SMS_READY_IND
APP_MN_READ_SMS_CNF
APP_MN_GET_VMW_FLAG_CNF
APP_MN_GET_VMW_FLAG_CNF
```

消息APP_MN_SMS_READY_IND调用栈
```
MMISMS_InitSmsList
StartAppInitSmsList
MMISMS_HandleSmsReadyInd
// APP_MN_SMS_READY_IND
HandlePsMsg
DispatchSysSig
MMK_DispatchExtSig
APP_Task
```

收件箱查看短信调用栈
```
MMISMS_AppReadSms
MMISMS_ReadPointedSMS
HandleOkMsgInSmsList
HandleMtBoxChildWinMsg
MMK_RunWinProc
MMK_DispatchToHandle
MMK_DispatchWinMSG
MMK_DispatchMSGQueue
APP_Task
```

MNSMS_ReadSmsEx请求的响应调用栈
```
MMISMS_HandleUserReadSMSCnf
MMISMS_HandleReadSmsCnf
// APP_MN_READ_SMS_CNF
HandlePsMsg
DispatchSysSig
MMK_DispatchExtSig
APP_Task
```

新消息调用栈
```
MMISMS_HandleSMSMTInd
// APP_MN_SMS_IND
HandlePsMsg
DispatchSysSig
MMK_DispatchExtSig
APP_Task
```
