D:\github\lvgl\lv_port_pc_visual_studio\LvglPlatform\lvgl\src\drivers\windows\lv_windows_display.c
```
lv_windows_create_display
data.mutex = CreateEventExW(NULL, NULL, 0, EVENT_ALL_ACCESS);
_beginthreadex
WaitForSingleObjectEx(data.mutex, INFINITE, FALSE);

lv_windows_display_thread_entrypoint
CreateWindowExW
SetEvent(data->mutex)
```