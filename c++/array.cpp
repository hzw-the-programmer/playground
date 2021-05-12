#include <iostream>

using namespace std;

class X {
    static int count;
    int i;
public:
    X(int i = count);
    X(const X& x);
    ~X();
};

int X::count = 0;

X::X(int i) : i(i) {
    cout << "X(int): " << this->i << endl;
    count++;
}

X::X(const X& x) : i(x.i) {
    cout << "X(const X&): " << i << endl;
}

X::~X() {
    cout << "~X: " << i << endl;
}

int main(int argc, char* argv[]) {
    X x[10];
}
