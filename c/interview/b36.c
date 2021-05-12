#include <stdio.h>
#include <stdlib.h>

int isb36c(char c) {
    return (c >= '0' && c <= '9') || (c >= 'a' && c <= 'z');
}

int b36ctb10(char c) {
    int i;
    if (c >= '0' && c <= '9') {
        i = c - '0';
    } else {
        i = c - 'a' + 10;
    }
    return i;
}

char b10tb36c(int i) {
    char c;
    if (i >= 0 && i <= 9) {
        c = i + '0';
    } else {
        c = i - 10 + 'a';
    }
    return c;
}

int b36tb10(char *s) {
    int i = 0;
    while (isb36c(*s)) {
        i *= 36;
        i += b36ctb10(*s);
        s++;
    }
    return i;
}

void reverses(char *s, int len) {
    for (int i = 0, j = len - 1; i < j; i++, j--) {
        char t = s[i];
        s[i] = s[j];
        s[j] = t;
    }
}

void b10tb36(int i, char *s) {
    char *p = s;
    while (i > 36) {
        *p++ = b10tb36c(i % 36);
        i /= 36;
    }
    *p++ = b10tb36c(i);
    *p = 0;
    
    reverses(s, p - s);
}

void b36add(char *a, char *b, char *r) {
    b10tb36(b36tb10(a) + b36tb10(b), r);
}

int main(){
    printf("%d\n", b36tb10("a"));
    printf("%d\n", b36tb10("ab"));
    printf("%d\n", b36tb10("109ayz"));
    printf("%d\n", b36tb10("aze091"));
    
    char *r = malloc(100);
    
    b10tb36(742, r);
    printf("%s\n", r);
    
    b10tb36(35000, r);
    printf("%s\n", r);
    
    b36add("ab", "ab", r);
    printf("%s\n", r);

    b36add("109ayz", "aze091", r);
    printf("%s\n", r);
}
