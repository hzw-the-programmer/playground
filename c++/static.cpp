#include <iostream>

using namespace std;

class Obj {
    char c;
public:
    Obj(char cc) : c(cc) {
        cout << "Obj::Obj for " << c << endl;
    }
    ~Obj() {
        cout << "Obj::~Obj for " << c << endl;
    }
};

Obj a('a');

void f() {
    cout << "enter f" << endl;
    static Obj b('b');
    cout << "leave f" << endl;
}

void g() {
    static Obj c('c');
}

int main(int argc, char* argv[]) {
    cout << "enter main" << endl;
    Obj d('d');
    f();
    f();
    cout << "leave main" << endl;
}
