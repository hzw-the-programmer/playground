#include <iostream>

using namespace std;

class X {
    int i;
public:
    X(int i = 0) : i(i) {
        cout << "X(int): " << this->i << endl;
    }
    X(const X& x) : i(x.i) {
        cout << "X(const X&): " << this->i << endl;
    }
    ~X() {
        cout << "~X(): " << this->i << endl;
    }
    /*const*/ X& operator++() {
        cout << "pre operator++: " << this->i << endl;
        this->i++;
        return *this;
    }
    const X operator++(int) {
        cout << "post operator++: " << this->i << endl;
        X t(this->i);
        cout << "***1***" << endl;
        this->i++;
        return t;
    }
    X& operator=(const X& x) {
        cout << "operator=: " << this->i << "=" << x.i << endl;
        this->i = x.i;
        return *this;
    }
    void func() /*const*/ {
        cout << "func: " << this->i << endl;
    }
};

int main(int argc, char* argv[]) {
    X a;
    
    //++a;

    //(++a).func();

    //X b = ++a;

    //X b(2);
    //b = ++a;

    //a++;

    //X b = a++;

    X b(1);
    b = a++;

    cout << "***2***" << endl;
}
