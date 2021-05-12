#include "Tree.h"
using namespace std;

int main(int argc, char* argv[]) {
    Tree *t = new Tree(23);
    cout << t;
    delete t;
}
