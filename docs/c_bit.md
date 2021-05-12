```
#define FLAG1 0x01
#define FLAG2 0x02
uint8 flag = 0;
if (flag & FLAG1) {
    flag &= ~FLAG1;
    flag |= FLAG2;
}
```
