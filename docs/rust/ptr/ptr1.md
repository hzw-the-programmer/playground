*const T 的大小取决于机器和T的类型
假设机器是64位
T 是 u8，*const T，即 *const u8 大小为8字节
T 是 [u8]，*const T，即 *const [u8] 大小为16字节
