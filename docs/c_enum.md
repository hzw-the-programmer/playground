```
typedef enum {
    EINT8_1 = -1,
    EINT8_2 = 0x7f,
} EINT8;

typedef enum {
    EUINT8_1 = 0xff,
} EUINT8;

typedef enum {
    EINT16_1 = -1,
    EINT16_2 = 0x7fff,
} EINT16;

typedef enum {
    EUINT16_1 = 0xffff,
} EUINT16;

typedef enum {
    EINT24_1 = -1,
    EINT24_2 = 0x7fffff,
} EINT24;

typedef enum {
    EUINT24_1 = 0xffffff,
} EUINT24;

typedef enum {
    EINT32_1 = -1,
    EINT32_2 = 0x7fffffff,
} EINT32;

typedef enum {
    EUINT32_1 = 0xffffffff,
} EUINT32;

int main() {
    printf("EINT8=%d\n", sizeof(EINT8));
    printf("EUINT8=%d\n", sizeof(EUINT8));
    printf("EINT16=%d\n", sizeof(EINT16));
    printf("EUINT16=%d\n", sizeof(EUINT16));
    printf("EINT24=%d\n", sizeof(EINT24));
    printf("EUINT24=%d\n", sizeof(EUINT24));
    printf("EINT32=%d\n", sizeof(EINT32));
    printf("EUINT32=%d\n", sizeof(EUINT32));
}
```

# vs2008输出：
```
EINT8=4
EUINT8=4
EINT16=4
EUINT16=4
EINT24=4
EUINT24=4
EINT32=4
EUINT32=4
```

# mtk输出：
```
EINT8=1
EUINT8=1
EINT16=2
EUINT16=2
EINT24=4
EUINT24=4
EINT32=4
EUINT32=4
```

# sprd输出：
```
EINT8=4
EUINT8=4
EINT16=4
EUINT16=4
EINT24=4
EUINT24=4
EINT32=4
EUINT32=4
```

proto的enum是force到4字节的：
```
#ifndef PROTOBUF_C__FORCE_ENUM_TO_BE_INT_SIZE
    #define PROTOBUF_C__FORCE_ENUM_TO_BE_INT_SIZE(enum_name) \
        , _##enum_name##_IS_INT_SIZE = INT_MAX
#endif

typedef enum _Ops {
    OPS_1 = 0,
    OPS_2 = 1,
    OPS_3 = 2
    PROTOBUF_C__FORCE_ENUM_TO_BE_INT_SIZE(OPS)
} Ops;
```

如果我们自己定义了enum:
```
typedef enum _COps {
    COPS_1 = 0,
    COPS_2 = 1,
    COPS_3 = 2,
    COPS_4 = 3,
} COps;
```

则，如下赋值是错误的：
```
COps cops[2] = {1, 2};
Ops ops[2] = cops; 
```
因为cops在mtk是1字节,ops是4字节
