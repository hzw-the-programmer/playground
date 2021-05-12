```
MMISMS_HandleSendCnf(void * 0x06a57e54) line 2097
// APP_MN_SEND_SMS_CNF
HandlePsMsg(void * 0x09957180 _g_mmisms_app, unsigned short 41376, void * 0x06a57e54) line 1035 + 9 bytes
DispatchSysSig(MmiSignalSTag * 0x06a57e44) line 793 + 23 bytes
MMK_DispatchExtSig(MmiSignalSTag * * 0x1c46fe64) line 660 + 11 bytes
APP_Task(unsigned long 0, void * 0x00000000) line 2206 + 9 bytes
MSDEVKERNEL! SCI_CreateThread line 856 + 11 bytes
MSDEVKERNEL! _tx_thread_shell_entry line 100 + 18 bytes
MSDEVKERNEL! _tx_win32_thread_entry@4 line 164
KERNEL32! 76d2343d()
NTDLL! 77969812()
NTDLL! 779697e5()
```

```
MMISMS_HandleSaveSMSCnf(void * 0x06d88014) line 821
// APP_MN_WRITE_SMS_CNF
HandlePsMsg(void * 0x09c87180 _g_mmisms_app, unsigned short 41378, void * 0x06d88014) line 1067 + 9 bytes
DispatchSysSig(MmiSignalSTag * 0x06d88004) line 793 + 23 bytes
MMK_DispatchExtSig(MmiSignalSTag * * 0x1c4cfe64) line 660 + 11 bytes
APP_Task(unsigned long 0, void * 0x00000000) line 2206 + 9 bytes
MSDEVKERNEL! SCI_CreateThread line 856 + 11 bytes
MSDEVKERNEL! _tx_thread_shell_entry line 100 + 18 bytes
MSDEVKERNEL! _tx_win32_thread_entry@4 line 164
KERNEL32! 7747343d()
NTDLL! 77d69812()
NTDLL! 77d697e5()
```

```
HandleSMSSendCnf(int 0, int 0) line 832
MMISMS_HandleSendCnf(void * 0x06a57e54) line 2164 + 18 bytes
HandlePsMsg(void * 0x09957180 _g_mmisms_app, unsigned short 41376, void * 0x06a57e54) line 1035 + 9 bytes
DispatchSysSig(MmiSignalSTag * 0x06a57e44) line 793 + 23 bytes
MMK_DispatchExtSig(MmiSignalSTag * * 0x1c46fe64) line 660 + 11 bytes
APP_Task(unsigned long 0, void * 0x00000000) line 2206 + 9 bytes
MSDEVKERNEL! SCI_CreateThread line 856 + 11 bytes
MSDEVKERNEL! _tx_thread_shell_entry line 100 + 18 bytes
MSDEVKERNEL! _tx_win32_thread_entry@4 line 164
KERNEL32! 76d2343d()
NTDLL! 77969812()
NTDLL! 779697e5()
```

```
HandleSMSSendSuccessCnf(int 0) line 241
***HandleSMSSendCnf(int 0, int 0) line 848 + 9 bytes
MMISMS_HandleSendCnf(void * 0x06a57e54) line 2164 + 18 bytes
HandlePsMsg(void * 0x09957180 _g_mmisms_app, unsigned short 41376, void * 0x06a57e54) line 1035 + 9 bytes
DispatchSysSig(MmiSignalSTag * 0x06a57e44) line 793 + 23 bytes
MMK_DispatchExtSig(MmiSignalSTag * * 0x1c46fe64) line 660 + 11 bytes
APP_Task(unsigned long 0, void * 0x00000000) line 2206 + 9 bytes
MSDEVKERNEL! SCI_CreateThread line 856 + 11 bytes
MSDEVKERNEL! _tx_thread_shell_entry line 100 + 18 bytes
MSDEVKERNEL! _tx_win32_thread_entry@4 line 164
KERNEL32! 76d2343d()
NTDLL! 77969812()
NTDLL! 779697e5()
```

```
MMISMS_AppSendSms(int 0, unsigned char 1, MMISMS_SEND_T_tag * 0x07475ee4) line 2187
MMISMS_WintabSendSMS(unsigned char 0, unsigned char 0) line 3032 + 20 bytes
***MMISMS_HandleSendOperation(int 0, unsigned long 458823, unsigned char 0) line 19757 + 11 bytes
MultiSendSelectSimCallback(unsigned long 0, unsigned char 1, void * 0x00000000) line 23392 + 15 bytes
HandleSIMSelectCommonFuncWinMsg(unsigned long 458823, unsigned long 57348, void * 0x0747843c) line 5691 + 19 bytes
MMK_RunWinProc(unsigned long 28049431, unsigned long 57348, void * 0x0747843c) line 4796 + 23 bytes
MMK_DispatchToHandle(unsigned long 28049431, unsigned long 57348, void * 0x0747843c) line 911 + 17 bytes
MMK_DispatchWinMSG(Message_tag * 0x074632f4) line 686 + 25 bytes
MMK_DispatchMSGQueue(Message_tag * 0x074632f4) line 394 + 9 bytes
APP_Task(unsigned long 0, void * 0x00000000) line 2141 + 9 bytes
MSDEVKERNEL! SCI_CreateThread line 856 + 11 bytes
MSDEVKERNEL! _tx_thread_shell_entry line 100 + 18 bytes
MSDEVKERNEL! _tx_win32_thread_entry@4 line 164
KERNEL32! 76d2343d()
NTDLL! 77969812()
NTDLL! 779697e5()
```

```
MMISMS_AddSendList(MMISMS_SEND_T_tag * 0x0747fd0c) line 3825
MMISMS_SendMsgBySeqNum(int 0, unsigned char 1, unsigned char 1, unsigned char 0, MMISMS_SEND_T_tag * 0x0747fd0c) line 1347 + 9 bytes
MMISMS_AppSendSms(int 0, unsigned char 1, MMISMS_SEND_T_tag * 0x0747fd0c) line 2219 + 21 bytes
MMISMS_WintabSendSMS(unsigned char 0, unsigned char 0) line 3032 + 20 bytes
MMISMS_HandleSendOperation(int 0, unsigned long 458823, unsigned char 0) line 19757 + 11 bytes
MultiSendSelectSimCallback(unsigned long 0, unsigned char 1, void * 0x00000000) line 23392 + 15 bytes
HandleSIMSelectCommonFuncWinMsg(unsigned long 458823, unsigned long 57348, void * 0x0747ee6c) line 5691 + 19 bytes
MMK_RunWinProc(unsigned long 48889880, unsigned long 57348, void * 0x0747ee6c) line 4796 + 23 bytes
MMK_DispatchToHandle(unsigned long 48889880, unsigned long 57348, void * 0x0747ee6c) line 911 + 17 bytes
MMK_DispatchWinMSG(Message_tag * 0x074633c4) line 686 + 25 bytes
MMK_DispatchMSGQueue(Message_tag * 0x074633c4) line 394 + 9 bytes
APP_Task(unsigned long 0, void * 0x00000000) line 2141 + 9 bytes
MSDEVKERNEL! SCI_CreateThread line 856 + 11 bytes
MSDEVKERNEL! _tx_thread_shell_entry line 100 + 18 bytes
MSDEVKERNEL! _tx_win32_thread_entry@4 line 164
KERNEL32! 76d2343d()
NTDLL! 77969812()
NTDLL! 779697e5()
```

```
MMISMS_SetCurSendDualSys(int 0) line 2705
MultiSendSelectSimCallback(unsigned long 0, unsigned char 1, void * 0x00000000) line 23381 + 9 bytes
HandleSIMSelectCommonFuncWinMsg(unsigned long 458823, unsigned long 57345, void * 0x074c4bdc) line 5691 + 19 bytes
MMK_RunWinProc(unsigned long 22282266, unsigned long 57345, void * 0x074c4bdc) line 4796 + 23 bytes
MMK_DispatchToHandle(unsigned long 22282266, unsigned long 57345, void * 0x074c4bdc) line 911 + 17 bytes
MMK_DispatchWinMSG(Message_tag * 0x074b3324) line 686 + 25 bytes
MMK_DispatchMSGQueue(Message_tag * 0x074b3324) line 394 + 9 bytes
APP_Task(unsigned long 0, void * 0x00000000) line 2141 + 9 bytes
MSDEVKERNEL! SCI_CreateThread line 856 + 11 bytes
MSDEVKERNEL! _tx_thread_shell_entry line 100 + 18 bytes
MSDEVKERNEL! _tx_win32_thread_entry@4 line 164
KERNEL32! 76d2343d()
NTDLL! 77969812()
NTDLL! 779697e5()
```

```
MMISMS_SetMessageContent(unsigned short 2, unsigned short * 0x074c8a2c, unsigned char 1, MMISMS_SMSEDIT_CONTENT_T * 0x099a782c) line 605
***HandleEditSmsWinMsg(unsigned long 458845, unsigned long 57348, void * 0x074d12ec) line 28800 + 21 bytes
MMK_RunWinProc(unsigned long 18284560, unsigned long 57348, void * 0x074d12ec) line 4796 + 23 bytes
MMK_DispatchToHandle(unsigned long 18284560, unsigned long 57348, void * 0x074d12ec) line 911 + 17 bytes
MMK_DispatchWinMSG(Message_tag * 0x074b33d4) line 686 + 25 bytes
MMK_DispatchMSGQueue(Message_tag * 0x074b33d4) line 394 + 9 bytes
APP_Task(unsigned long 0, void * 0x00000000) line 2141 + 9 bytes
MSDEVKERNEL! SCI_CreateThread line 856 + 11 bytes
MSDEVKERNEL! _tx_thread_shell_entry line 100 + 18 bytes
MSDEVKERNEL! _tx_win32_thread_entry@4 line 164
KERNEL32! 76d2343d()
NTDLL! 77969812()
NTDLL! 779697e5()
```

```
MMISMS_SetAddressToMessage(const unsigned char * 0x1c92fb40, unsigned char 11, MMISMS_NUMBER_LIST_T * 0x099a7836) line 2471
GetNumberFromEditbox(unsigned long 458960, unsigned char 0) line 25324 + 27 bytes
***HandleEditSmsWinMsg(unsigned long 458845, unsigned long 57348, void * 0x074d12ec) line 28804 + 12 bytes
MMK_RunWinProc(unsigned long 18284560, unsigned long 57348, void * 0x074d12ec) line 4796 + 23 bytes
MMK_DispatchToHandle(unsigned long 18284560, unsigned long 57348, void * 0x074d12ec) line 911 + 17 bytes
MMK_DispatchWinMSG(Message_tag * 0x074b33d4) line 686 + 25 bytes
MMK_DispatchMSGQueue(Message_tag * 0x074b33d4) line 394 + 9 bytes
APP_Task(unsigned long 0, void * 0x00000000) line 2141 + 9 bytes
MSDEVKERNEL! SCI_CreateThread line 856 + 11 bytes
MSDEVKERNEL! _tx_thread_shell_entry line 100 + 18 bytes
MSDEVKERNEL! _tx_win32_thread_entry@4 line 164
KERNEL32! 76d2343d()
NTDLL! 77969812()
NTDLL! 779697e5()
```

```
MMISMS_GetAutoSave
```

```
//参考HandleEditSmsWinMsg
MMISMS_SetMessageContent(edit_string.wstr_len, edit_string.wstr_ptr, TRUE, &g_mmisms_global.edit_content);
MMISMS_ClearDestNum();
MMISMS_SetAddressToMessage((uint8 *)dest_addr,(uint8)strlen((char *)dest_addr), &g_mmisms_global.edit_content.dest_info.dest_list);
MMISMS_SetCurSendDualSys(MN_DUAL_SYS_1+((MN_DUAL_SYS_E)major_select_sim-MMISET_MAJOR_SIM_SEL_SIM1));
MMISMS_ClearDestId();
MMISMS_SetDeliverResendFlag(FALSE);
MMISMS_SetIsSendFail(FALSE);
if (MMISMS_GetAutoSave() == MMINV_SMS_SETTING_SAVE
    && MMISMS_HaveEnoughSpace(dualSys, TRUE, 0)) {
    MMISMS_SetSaveFlag(TRUE);
}
else
{
    MMISMS_SetSaveFlag(FALSE);
}
MMISMS_WintabSendSMS
```
