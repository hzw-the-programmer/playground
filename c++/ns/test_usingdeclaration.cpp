#include "usingdeclaration.h"

using namespace U;
using V::f;
/*
void f() {
    cout << "f" << endl;
}
*/

int main(int argc, char* argv[]) {
    f();
    //U::f();

    /*
    using namespace U;
    using V::f;
    f();
    U::f();
    */
}
