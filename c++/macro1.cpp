#include <iostream>

using namespace std;

#define BAND(x) ((x) > 5 && (x) < 10 ? (x) : 0)

int main(int argc, char* argv[]) {
    for (int i = 4; i < 11; i++) {
        int a = i;
        cout << "a = " << a << endl;
        cout << "\tBAND(++a) = " << BAND(++a) << endl;
        cout << "\ta = " << a << endl;
    }
}
