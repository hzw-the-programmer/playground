#include <iostream>
#include <cstdlib>
#include <cstring>

using namespace std;

class String {
public:
    String() {
        ca = (char*)malloc(1);
        *ca = 0;
        cout << "default constructor: " << ca << endl;
    }

    String(const char *ca) {
        this->ca = (char*)malloc(strlen(ca) + 1);
        strcpy(this->ca, ca);
        cout << "parameter constructor: " << ca << endl;
    }

    String(const String &s) {
        ca = (char*)malloc(strlen(s.ca) + 1);
        strcpy(ca, s.ca);
        cout << "copy constructor: " << ca << endl;
    }

    ~String() {
        cout << "destructor: " << ca << endl;
        free(ca);
    }

    String& operator=(const String &s) {
        cout << "assign operator begin: " << ca << endl;
        free(ca);
        ca = (char*)malloc(strlen(s.ca) + 1);
        strcpy(ca, s.ca);
        cout << "assign operator end: " << ca << endl;
        return *this;
    }

    /*
    String& operator=(const char *ca) {
        cout << "assign operator begin: " << this->ca << endl;
        free(this->ca);
        this->ca = (char*)malloc(strlen(ca) + 1);
        strcpy(this->ca, ca);
        cout << "assign operator end: " << this->ca << endl;
        return *this;
    }
    */
private:
    char *ca;
};

int main(int argc, char** argv) {
    String s1;
    String s2("s2");
    String s3 = "s3";
    String s4 = s3;
    s1 = s2 = s3;
    s1 = "s1";
    return 0;
}
