```
void test() {
	uint8 *buf = RA_MALLOC_MEM(16);
	int32 i = 0;
	uint16 num = 0;

	for (i = 0; i < 16; i++) {
		buf[i] = i;
	}

	for (i = 0; i < 15; i++) {
		num = *(RA_UINT16*)(buf + i);
		printf("test 0x%08x=0x%04x", buf + i, num);
	}
}
```

```
test 0xf02a865c=0x00000100
test 0xf02a865d=0x00000100
test 0xf02a865e=0x00000302
test 0xf02a865f=0x00000302
test 0xf02a8660=0x00000504
test 0xf02a8661=0x00000504
test 0xf02a8662=0x00000706
test 0xf02a8663=0x00000706
test 0xf02a8664=0x00000908
test 0xf02a8665=0x00000908
test 0xf02a8666=0x00000b0a
test 0xf02a8667=0x00000b0a
test 0xf02a8668=0x00000d0c
test 0xf02a8669=0x00000d0c
test 0xf02a866a=0x00000f0e
```
