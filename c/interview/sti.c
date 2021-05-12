#include <stdio.h>

#define MAX 10000

int sti(char *s) {
    if (s == NULL) {
        return 0;
    }

    int neg = 0;
    if (*s == '-') {
        neg = 1;
        s++;
    }
    
    int i = 0;
    while (*s >= '0' && *s <= '9') {
        i *= 10;
        i += *s - '0';
        s++;
    }
    
    return neg ? -i : i;
}

int main(){
    printf("%d\n", sti("12345"));
    printf("%d\n", sti(""));
    printf("%d\n", sti(NULL));
    printf("%d\n", sti("aaa12345"));
    printf("%d\n", sti("123bdfdsaf45"));
    //printf("%d\n", sti("100001"));
    printf("%d\n", sti("000012345"));
    printf("%d\n", sti("-100"));
}
