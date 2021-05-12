#include <stdio.h>

int main()
{
    float a = 123.123;
    //char *b = (char*)&a;
    unsigned char *b = (char*)&a;

    int size = sizeof(float);
    
    printf("float size = %d\n", size);

    for (int i = 0; i < size; i++) {
        printf("%x ", *(b + i));
    }
    printf("\n");

    /*
    for (int i = 0; i < size; i++) {
        printf("%x ", *(b + i) & 0xFF);
    }
    printf("\n");
    */
   //while(1) {}
}
