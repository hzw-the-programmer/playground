#define MASK 0x07 // 0000 0111
#define TYPE_0 0x00 // 0000 0000
#define TYPE_1 0x01 // 0000 0001
#define TYPE_2 0x02 // 0000 0010
#define TYPE_3 0x03 // 0000 0011
#define TYPE_4 0x04 // 0000 0100
#define TYPE_5 0x05 // 0000 0101
#define TYPE_6 0x06 // 0000 0110
#define TYPE_7 0x07 // 0000 0111

#define FLAG_1 0x08 // 0000 1000

unsigned char flags = FLAG_1 | TYPE_1; // 0000 1001

unsigned char type = flags & MASK;
if (flags & FLAG_1)