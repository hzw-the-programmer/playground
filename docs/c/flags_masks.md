# mask 的使用
用2位表示4个类型，定义如下
#define TYPE_0 0x00 // 0000 0000
#define TYPE_1 0x01 // 0000 0001
#define TYPE_2 0x02 // 0000 0010
#define TYPE_3 0x03 // 0000 0011

有一个 flags 变量

uint8 flags = 0xab // 1010 1011

如何取到编码在 flags 里面的类型

类型编码在最后两位，只需取最后两位

定义最后两位为1，其他位为0的 mask

#define TYPE_MASK 0x03 // 0000 0011

再用flags与上mask

flags & TYPE_MASK == 0x03 // 0000 0011

# flag 的使用
