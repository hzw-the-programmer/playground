#include <iostream>

using namespace std;

#define CLASS(ID) class ID { \
public: \
    ID() {cout << #ID " constructor\n";} \
    ~ID() {cout << #ID " destructor\n";} \
}

CLASS(Base1);
CLASS(Member1);
CLASS(Member2);
CLASS(Member3);
CLASS(Member4);

class Derived1 : public Base1 {
    Member1 m1;
    Member2 m2;
public:
    //Derived1() {cout << "Derived1 constructor\n";}
    Derived1() : m2(), m1(), Base1() {cout << "Derived1 constructor\n";}
    ~Derived1() {cout << "Derived1 destructor\n";}
};

class Derived2: public Derived1 {
    Member4 m4;
    Member3 m3;
public:
    //Derived2() {cout << "Derived2 constructor\n";}
    Derived2() : m3(), m4(), Derived1() {cout << "Derived2 constructor\n";}
    ~Derived2() {cout << "Derived2 destructor\n";}
};

int main(int argc, char* argv[]) {
    Derived2 d2;
}