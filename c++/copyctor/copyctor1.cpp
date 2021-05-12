#include <iostream>
#include <cstring>
#include <stdio.h>

using namespace std;

class String {
    private:
        char *s;
        int size;
    public:
        String(const char*);
        ~String();
        //String(const String&);
        void print() { cout << s << endl;}
        void change(const char*);
};

String::String(const char *str) {
    cout << "ctor called." << endl;
    size = strlen(str);
    s = new char[size + 1];
    strcpy(s, str);
}

String::~String() {
    cout << "dtor called." << endl;
    delete [] s;
}

/*
String::String(const String &str) {
    cout << "cctor called." << endl;
    size = str.size;
    s = new char[size + 1];
    strcpy(s, str.s);
}
*/

void String::change(const char *str) {
    printf("%08lx\n", (long)s);
    //delete [] s;
    printf("%08lx\n", (long)s);
    size = strlen(str);
    s = new char[size + 1];
    printf("%08lx\n", (long)s);
    strcpy(s, str);
}

int main(int argc, char **argv) {
    String s1("hello");
    String s2 = s1;

    s1.print();
    s2.print();

    s2.change("zhiwenhe");

    s1.print();
    s2.print();
}
