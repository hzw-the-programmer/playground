#include <iostream>
#include <set>

using namespace std;

class Num {
    int num;
    friend ostream& operator<<(ostream&, const Num&);
public:
    Num(int num = 0) : num(num) {cout << num << " Num::Num(int)\n";}
    Num(const Num& a) : num(a.num + 1) {cout << num << " Num::Num(Num&)\n";}
    ~Num() {cout << num << " Num::~Num()\n";}
    Num& operator=(const Num& num) {cout << this->num << " operator=(const Num&)\n"; return *this;}
    bool operator<(const Num& a) const {
        return num < a.num;
        //return num > a.num;
    }
};

ostream& operator<<(ostream& os, const Num& num) {
    return os << num.num;
}

int main() {
    Num n0;
    Num n1(1);
    Num n2(n1);
    Num n3 = n2;
    n0 = n1;

    set<Num> s;
    s.insert(n3);
    s.insert(n2);
    s.insert(n1);
    s.insert(n0);

    for (set<Num>::iterator iter = s.begin(); iter != s.end(); iter++) {
        if (iter != s.begin()) {
            cout << " ";
        }
        cout << *iter;
    }
    cout << endl;
}