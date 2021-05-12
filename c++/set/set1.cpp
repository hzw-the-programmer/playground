#include <iostream>
#include <set>

using namespace std;

//g++ set1.cpp
//./a.out

int main() {
    int inta[] = {75, 23, 65, 42, 13, 23};
    
    set<int> ints(inta, inta + 6);

    for (set<int>::iterator iter = ints.begin(); iter != ints.end(); iter++) {
        if (iter != ints.begin()) {
            cout << " ";
        }
        cout << *iter;
    }

    cout << endl;

    for (set<int>::reverse_iterator iter = ints.rbegin(); iter != ints.rend(); iter++) {
        if (iter != ints.rbegin()) {
            cout << " ";
        }
        cout << *iter;
    }

    cout << endl;

    pair<set<int>::iterator, bool> ret;
    cout << &ret << " " << ret.second /*<< " " << *ret.first*/ << endl;

    ret = ints.insert(11);
    cout << &ret << " " << ret.second << " " << *ret.first << endl;

    ret = ints.insert(23);
    cout << &ret << " " << ret.second << " " << *ret.first << endl;

    ret = ints.insert(100);
    cout << &ret << " " << ret.second << " " << *ret.first << endl;

    for (set<int>::iterator iter = ints.begin(); iter != ints.end(); iter++) {
        if (iter != ints.begin()) {
            cout << " ";
        }
        cout << *iter;
    }

    cout << endl;
}