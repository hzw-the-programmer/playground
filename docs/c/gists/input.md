```
for (i = begin; i <= end; i++) {
    int index;
    char c = 0;

    if (inverse) {
        index = end - (i - begin);
        if (index > begin) {
            c = buf[index - 1];
        }
    } else {
        index = i;
        if (index < end) {
            c = buf[index];
        }
    }

    if (index == cursor) {
        // draw cursor
    }

    x += cursor_width * sign;

    if (c) {
        // draw char
        x += width * sign;
    }
}
```