#ifndef TREE_H
#define TREE_H

#include <iostream>

class Tree {
    int height;
public:
    Tree(int height) : height(height) {
        std::cout << "Tree(int)\n";
    }
    ~Tree() {
        std::cout << "~Tree()\n";
    }
    friend std::ostream& operator<<(std::ostream& os, const Tree *t) {
        return os << "Tree height is: "
                  << t->height << std::endl;
    }
};

#endif
