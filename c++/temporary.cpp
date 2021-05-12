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
};

X::X(int i) : i(i), copyed(0) {
    cout << "X(int): " << this->i << endl;
}

X::X(const X& x): i(x.i), copyed(1) {
    cout << "X(const X&): " << i << endl;
}

X::~X() {
    cout << "~X: " << i;
    if (copyed) {
        cout << " copyed";
    }
    cout << endl;
}

X& X::operator=(const X& x) {
    cout << "operator=: " << i << "=" << x.i << endl;
}

X f1(X x) {
    return X(1);
}

int main(int argc, char* argv[]) {
    X a;
    a = f1(a);

    cout << "***END***" << endl;
}
