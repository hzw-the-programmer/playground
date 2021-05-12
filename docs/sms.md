1120 = 153 * 7 + 49
总共1120个字符，每条短信153个字符，收到7个完整短信，加49个字符短信。

612 = 153 * 4
展讯收件箱第一条短信612个字符，即4条完整短信。

508 = 153 * 3 + 49
展讯收件箱第二条短信508个字符，即3条完整短信，加49个字符短信。

MS_Code\MS_MMI\source\mmi_app\app\sms\h\mmisms_app.h
MMISMS_MAX_DEF_SPLIT_LEN 153
MMISMS_MAX_MESSAGE_LEN (MMISMS_SPLIT_MAX_NUM * MMISMS_MAX_DEF_SPLIT_LEN)

MS_Code\MS_MMI\source\mmi_app\custom\h\mmi_custom_define.h
MMISMS_SPLIT_MAX_NUM 4 // 6

MS_Code\MS_MMI\source\mmi_app\app\sms\c\mmisms_receive.c
MMISMS_HandleSMSMTInd
