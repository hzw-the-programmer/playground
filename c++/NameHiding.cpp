#include <iostream>
#include <string>

using namespace std;

class Base {
public:
    int f() const {
        cout << "Base::f()\n";
        return 1;
    }
    int f(string) const {
        cout << "Base::f(string)\n";
        return 1;
    }
    /*
    void f() const {

    }
    */
    int f(int) const {

    }
    void g() {
        cout << "Base::g()\n";
    }
};

class Derived2 : public Base {
public:
    // Redefinition
    int f() const {
        cout << "Derived2::f()\n";
        return 2;
    }
};

class Derived3 : public Base {
public:
    void f() const {
        cout << "Derived3::f()\n";
    }
};

class Derived4 : public Base {
public:
    // Overload
    int f(int) const {
        cout << "Derived4::f(int)\n";
        return 3;
    }
};

class Derived5 : public Base {
public:
    // Redefine
    int f() {
        cout << "Derived5::f()\n";
        return 10;
    }
};

int main(int argc, char* argv[]) {
    Derived2 d2;
    cout << d2.f() << endl;
    //cout << d2.f("hello world") << endl;

    Derived3 d3;
    d3.f();
    //d3.f("hello world");

    Derived4 d4;
    //d4.f();
    //d4.f("hello world");
    d4.f(1);

    Derived5 d5;
    d5.f();
    //d5.f("hello");

    return 0;
}