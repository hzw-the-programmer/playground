#include <stdio.h>

int strcompare(char *a, char *b) {
    if (a == NULL && b == NULL) {
        return 0;
    } else if (a == NULL && b != NULL) {
        return 1;
    } else if (a != NULL && b == NULL) {
        return -1;
    }

    while (*a != 0 && *a == *b) {
        a++;
        b++;
    }
    
    if (*a == *b) {
        return 0;
    } else if (*a > *b) {
        return 1;
    } else if (*a < *b) {
        return -1;
    }
}

int main(){
    char *a = "";
    char *b = "";
    printf("%d\n", strcompare(a, b));

    a = "abc";
    b = "abc";
    printf("%d\n", strcompare(a, b));
    
    a = "Abc";
    b = "abc";
    printf("%d\n", strcompare(a, b));
    
    a = "abcd";
    b = "abcde";
    printf("%d\n", strcompare(a, b));
    
    a = NULL;
    b = NULL;
    printf("%d\n", strcompare(a, b));
    
    a = "abc";
    b = NULL;
    printf("%d\n", strcompare(a, b));
    
    a = NULL;
    b = "abc";
    printf("%d\n", strcompare(a, b));

}
