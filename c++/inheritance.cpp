#include <iostream>

using namespace std;

class X {
    int i;
public:
    int getI() {return i;}
};

//class Y : X {
class Y : public X {
    int i;
public:
    int get() {return getI();}
};

int main(int argc, char* argv[]) {
    Y y;
    cout << y.getI() << endl;
    cout << y.get() << endl;
    return 0;
}
