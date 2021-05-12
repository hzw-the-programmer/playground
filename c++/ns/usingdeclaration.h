#ifndef USING_DECLARATION_H
#define USING_DECLARATION_H

#include <iostream>
using namespace std;

namespace U {
    void f() {
        cout << "U::f" << endl;
    }

    void g() {
        cout << "U::g" << endl;
    }

    char a = 'U';
}

namespace V {
    void f() {
        cout << "V::f" << endl;
    }

    void g() {
        cout << "V::g" << endl;
    }

    char a = 'V';
}

#endif // USING_DECLARATION_H
