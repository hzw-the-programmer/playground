#include <iostream>

using namespace std;

class X {
    int i;
    int copyed;
public:
    X(int i = 0) : i(i), copyed(0) {
        cout << "X(int): " << this->i << endl;
    }
    X(const X& x) : i(x.i), copyed(1) {
        cout << "X(const X&): " << i << endl;
    }
    ~X() {
        cout << "~X(): " << this->i;
        if (copyed) {
            cout << " copyed";
        }
        cout << endl;
    }
    /*const*/ X& operator=(const X& r) {
        cout << "operator=: " << this->i << "=" << r.i << endl;
        this->i = r.i;
        return *this;
    }
    const X operator+(const X& r) const {
        cout << "operator+: " << this->i << "=" << r.i << endl;
        return X(i + r.i);
    }
    void func() /*const*/ {
        cout << "func" << endl;
    }
};

X func(X x) {
    cout << "func" << endl;
    //return x;
    return X();
}

int main(int argc, char* argv[]) {
    X a(1), b(2), c(3);
    //X d = a + b + c;

    //X d;
    //d = a + b + c;

    //a = b = c;

    //(a = b = c).func();

    func(a + b);

    cout << "***END***" << endl;
}
