MTK画图片数组时，图片数组要四字节对齐。

```
__align(4) const uint8 pic[] = {
    0x01, 0x02...
};
```
