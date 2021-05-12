#include <iostream>
#include <string>

using namespace std;

class X {
    static int count;
    string name;
public:
    X(const string& name) : name(name) {
        count++;
        cout << "X(const string&): " << this->name << endl;
    }
    X(const X& x) : name(x.name) {
        name += " copy";
        count++;
        cout << "X(const X&): " << name << endl;
    }
    ~X() {
        count--;
        cout << "~X(): " << name << endl;
    }
    static void print() {
        cout << "count: " << count << endl;
    }
};

int X::count = 0;

X f(X t) {
    cout << "in f" << endl;
    return t;
}

int main(int argc, char* argv[]) {
    X a("a");
    //X b = f(a);
    //f(a);
    //f(f(a));
    X b = f(f(a));
    X::print();
    cout << "***END***" << endl;
}
