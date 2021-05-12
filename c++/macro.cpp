#include <iostream>

using namespace std;

//#define F(a, b) a >= b ? 0 : 1
#define F(a, b) (a >= b ? 0xff : 1)
//#define F(a, b) ((a) >= b ? 0xff : 1)

int main(int argc, char* argv[]) {
    cout << "F(1, 2)=" << F(1, 2) << endl;
    cout << "F(0x11 & 0x0f, 0x07)=" << F(0x11 & 0x0f, 0x07) << endl;
    cout << (0x11 & 0x0f >= 0x07 ? 0xff : 1) << endl;
    cout << (0x11 & (0x0f >= 0x07 ? 0xff : 1)) << endl;
}
