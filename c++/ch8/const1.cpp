#include <iostream>

using namespace std;

extern const int bufsize;
extern int buf[];

int main(int argc, char* argv[]) {
    cout << bufsize << endl;
    for (int i = 0; i < bufsize; i++) {
        cout << buf[i] << endl;
    }
    return 0;
}
