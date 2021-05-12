#include <iostream>

using namespace std;

class X {
    int i;
    int copyed;
public:
    X(int i = 0);
    X(const X& x);
    ~X();
    X& operator=(const X& x);
    void modify();
};

X::X(int i) : i(i), copyed(0) {
    cout << "X(int): " << this->i << endl;
}

X::X(const X& x) : i(x.i), copyed(1) {
    cout << "X(const X&): " << i << endl;
}

X::~X() {
    cout << "~X(): " << i;
    if (copyed) {
        cout << " copyed";
    }
    cout << endl;
}

X& X::operator=(const X& x) {
    cout << "operator=: " << i << "=" << x.i << endl;
    //i = x.i;
    return *this;
}

void X::modify() {
    i++;
}

X f1() {
    X(1);
    return X();
}

const X f2() {
    return X();
}

void f3(X x) {
    X(3);
}

void f4(const X x) {
    X(4);
}

void f5(X& x) {
}

void f6(const X& x) {
    X(6);
}

int main(int argc, char* argv[]) {
    f1() = X(2);
    //f1().modify();
    //f3(f1());
    //f4(f1());
    //f5(f1());
    //f6(f1());

    //f2() = X(1);
    //f2().modify();
    //f3(f2());
    //f4(f2());
    //f5(f2());
    //f6(f2());

    cout << "***END***" << endl;
    return 0;
}
