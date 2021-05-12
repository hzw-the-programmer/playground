#include <fstream>

using namespace std;

//ofstream out("StaticDestructors.txt");
extern ofstream out;

class Obj {
    char c;
public:
    Obj(char c) : c(c) {
        out << "call Obj(char c) for " << c << endl;
    }
    ~Obj() {
        out << "call ~Obj() fro " << c << endl;
    }
};

ofstream out("StaticDestructors.txt");
Obj a('a');
//ofstream out("StaticDestructors.txt");

void f() {
    static Obj b('b');
}

void g() {
    static Obj c('c');
}

int main(int argc, char* argv[]) {
    out << "enter main" << endl;
    f();
    f();
    out << "leave main" << endl;
}