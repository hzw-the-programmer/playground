```
#define MTK  0
#define SPRD 1

#define 128_160 0
#define 240_320 1

#define PLATFORM MTK
#define LCD 128_160

#if PLATFORM == MTK

#if LCD == 128_160
#define MTK_128_160
#elif LCD == 240_320
#define MTK_240_320
#endif

#elif PLATFORM == SPRD

#if LCD == 128_160
#define SPRD_128_160
#elif LCD == 240_320
#define SPRD_240_320
#endif

#else
#error "macro PLATFORM is invalid"
#endif
```
