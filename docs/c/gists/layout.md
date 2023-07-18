```
w = 0;
for (; start < end; start++) {
    child.x = x + w;
    if (reverse) {
        child.x = x + width - w - child.w;
    }
    w += child.w;
}
```