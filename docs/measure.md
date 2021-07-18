```
#define MEASURE(func) \
{ \
    kal_uint32 start; \
    start = kal_get_systicks(); \
    func; \
    logf("%s cost=%d", #func, kal_get_systicks() - start); \
}
```
