WinMainCRTStartup
__tmainCRTStartup
WinMain
AfxWinMain
CCoolSimulatorApp::InitInstance
CDialog::DoModal
CWnd::RunModalLoop
AfxPumpMessage
CWinThread::PumpMessage
AfxInternalPumpMessage
AfxWndProcBase
AfxWndProc
AfxCallWndProc
CWnd::WindowProc
CWnd::OnWndMsg
CCoolSimulatorDlg::OnTimer
TimerCallBack

static void timer_cb() {
	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,id=%d", __func__, sxr_GetCurrentTaskId());
}

static void timer_test() {
	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,id=%d", __func__, sxr_GetCurrentTaskId());
	StartTimer(EBOOK_RECURSIVE_TIMER, 1000, timer_cb);
}