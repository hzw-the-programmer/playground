#include <iostream>
#include <cstdlib>
#include <cstring>

using namespace std;

class String {
public:
    String() {
        ca = (char*)malloc(1);
        *ca = '\0';
        cout << "String(): " << ca << endl;
    }

    String(const char* ca) {
        this->ca = (char*)malloc(strlen(ca) + 1);
        strcpy(this->ca, ca);
        cout << "String(const char* ca): " << this->ca << endl;
    }

    String(const String& s) {
        ca = (char*)malloc(strlen(s.ca) + 1);
        strcpy(ca, s.ca);
        cout << "String(const String& s): " << ca << endl;
    }

    ~String() {
        cout << "~String(): " << ca << endl;
        free(ca);
    }

    String& operator=(const String& s) {
        ca = (char*)malloc(strlen(s.ca) + 1);
        strcpy(ca, s.ca);
        cout << "operator=(const String& s): " << ca << endl;
    }

    String& operator=(const char* ca) {
        this->ca = (char*)malloc(strlen(ca) + 1);
        strcpy(this->ca, ca);
        cout << "operator=(const String& s): " << this->ca << endl;
    }

private:
    char *ca;
};

int main(int argc, char** argv) {
    String s1;
    String s2("s2");
    String s3 = "s3";
    String s4 = s3;
    s4 = "s4";

    return 0;
}
