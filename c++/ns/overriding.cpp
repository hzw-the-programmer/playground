#include <iostream>
#include "math.h"

using namespace std;
using namespace math;

int main(int argc, char* argv[]) {
    Integer a(1); // Hide math::a
    cout << a.getValue() << endl;
    cout << math::a.getValue() << endl;
}
