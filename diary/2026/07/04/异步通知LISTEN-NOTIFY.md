# 异步通知 LISTEN/NOTIFY

服务器可通过 **NotificationResponse ('A')** 推送事件：

```text
'A'
Int32   长度
Int32   通知进程 PID
String  频道名 (null 结尾)
String  传递的 payload (null 结尾)
```
