#include <iostream>

using namespace std;

class Base {
    public:

    Base() {
        cout << "Base constructor" << endl;
    }

    /*virtual*/ ~Base() {
        cout << "Base destructor" << endl;
    }

    /*virtual*/ void show() {
        cout << "Base show" << endl;
    }
};

class Derived : public Base {
    public:

    Derived() {
        cout << "Derived constructor" << endl;
    }

    ~Derived() {
        cout << "Derived destructor" << endl;
    }

    void show() {
        cout << "Derived show" << endl;
    }
};

int main(int argc, char **argv) {
    //Base *b = new Derived;
    Derived d;
    Base *b = &d;
    //Derived *b = new Derived;
    b->show();
    //delete b;
    return 0;
}
