#include <iostream>

using namespace std;

#define DEBUG(x) cout << #x " = " << x << endl;
#define TRACE(x) cerr << #x << endl, x

void f(int i) {}

int main(int argc, char* argv[]) {
    int x = 10;
    DEBUG(x);
    for (int i = 0; i < 10; i++)
        TRACE(f(i));
}

// g++ -E stringizing.cpp
// g++ -E -P stringizing.cpp
// g++ stringizing.cpp
