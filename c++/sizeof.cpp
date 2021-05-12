#include <iostream>
#include <cstdio>
using namespace std;

struct x {
    char c;
    int i;
    float f;
    double d;
};

struct y {
    char c;
    short s;
    void *p;
    int i;
};

int main(int argc, char* argv[]) {
    cout << "char is " << sizeof(char) << endl;
    cout << "short is " << sizeof(short) << endl;
    cout << "int is " << sizeof(int) << endl;
    cout << "long is " << sizeof(long) << endl;
    cout << "float is " << sizeof(float) << endl;
    cout << "double is " << sizeof(double) << endl;
    cout << "pointer is " << sizeof(int*) << endl;
    cout << "struct x is " << sizeof(struct x) << endl;
    cout << "struct y is " << sizeof(struct y) << endl;

    struct y a = {0x01, 0x0203, (void*)0x0405060708090a0b, 0x0c0d0e0f};
    char *cp = (char*)&a;
    //unsigned char *cp = (unsigned char*)&a;
    for (int i = 0; i < sizeof(struct y); i++) {
        printf("%02hhx", *cp++);
        //printf("%02x", *cp++);
    }
    cout << endl;

    struct x b = {0};
    b.c = 0x01;
    b.i = 0x02030405;
    b.f = 0.1;
    b.d = 0.1;
    cp = (char*)&b;
    //cp = (unsigned char*)&b;
    for (int i = 0; i < sizeof(struct x); i++) {
        printf("%02hhx", *cp++);
        //printf("%02x", *cp++);
    }
    cout << endl;
}
