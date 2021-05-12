在收件箱读取短信

调用栈
```
MNSMS_ReadSmsEx
MMISMS_AppReadSms
MMISMS_ReadPointedSMS
HandleOkMsgInSmsList
HandleMtBoxChildWinMsg
MMK_RunWinProc
MMK_DispatchToHandle
MMK_DispatchWinMSG
MMK_DispatchMSGQueue
APP_Task
MSDEVKERNEL! SCI_CreateThread
MSDEVKERNEL! _tx_thread_shell_entry
MSDEVKERNEL! _tx_win32_thread_entry
KERNEL32! 7664343d()
NTDLL! 77679812()
NTDLL! 776797e5()
```

MNSMS_ReadSmsEx会发送读短信请求：MSG_ID_MN_SMS_READ_SMS_REQ
当请求完成，MN会发送读短信请求确认：APP_MN_READ_SMS_CNF
调用栈
```
MMISMS_HandleUserReadSMSCnf
MMISMS_HandleReadSmsCnf
// APP_MN_READ_SMS_CNF
HandlePsMsg
DispatchSysSig
MMK_DispatchExtSig
APP_Task
MSDEVKERNEL! SCI_CreateThread
MSDEVKERNEL! _tx_thread_shell_entry
MSDEVKERNEL! _tx_win32_thread_entry
KERNEL32! 7664343d()
NTDLL! 77679812()
NTDLL! 776797e5()
```

MMISMS_HandleUserReadSMSCnf执行到如下语句会去尝试拿长短信的下一部分
```
PUBLIC void MMISMS_HandleUserReadSMSCnf(DPARAM param)
{
    ...
    if (g_mmisms_global.operation.cur_order_index < MMISMS_SPLIT_MAX_NUM-1)
    {
        g_mmisms_global.operation.cur_order_index ++;
        // 尝试去拿长短信的下一部分
        next_order_id = MMISMS_GetCurOperationOrderId();
    }
    ...
    if ( next_order_id != PNULL )
    {
        ...
        // 有长短信的下一部分，再次发起读取请求
        MMISMS_AppReadSms( next_order_id );
        ...
    }
    else
    {
        ...
        // 该短信读取完毕，短信内容存储在g_mmisms_global.read_msg.read_content
        ...
    }
    ...
}
```

数据丢给richtext展示
```
GUIRICHTEXT_AddItem
AddSMSContentItemtoRichText
LoadSms2Window
MMISMS_SetReadedSms2Window
HandleOkMsgInSmsList
HandleMtBoxChildWinMsg
MMK_RunWinProc
MMK_DispatchToHandle
MMK_SendMsg(child_win_id, MSG_SMS_FINISH_READ, PNULL);
HandleMsgBoxMainWindow
MMK_RunWinProc
MMK_DispatchToHandle
MMK_SendMsg(MMISMS_MSGBOX_MAIN_WIN_ID, MSG_SMS_FINISH_READ, PNULL);
MMISMS_HandleUserReadSMSCnf
MMISMS_HandleReadSmsCnf
HandlePsMsg
DispatchSysSig
MMK_DispatchExtSig
APP_Task
```

总结
从该分析可以看出，展讯平台已将长短信拼接完成。
