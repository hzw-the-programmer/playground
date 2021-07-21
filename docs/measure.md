```
#define MEASURE(func) \
{ \
    kal_uint32 start; \
    start = kal_get_systicks(); \
    func; \
    kal_prompt_trace(MOD_ABM, "%s cost=%d", #func, kal_get_systicks() - start); \
}
```

```
#define MEASURE(func) \
{ \
    uint32 start; \
    start = SCI_GetTickCount(); \
    func; \
    SCI_TRACE_LOW("hzw %s cost=%d", #func, SCI_GetTickCount() - start); \
}
```
