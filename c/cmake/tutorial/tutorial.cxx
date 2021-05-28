// mkdir build
// cd build
// /d/tools/cmake-3.18.2-win64-x64/bin/cmake.exe ../
// /d/tools/cmake-3.18.2-win64-x64/bin/cmake.exe --build .
#include <iostream>
#include "config.h"

int main(int argc, char *argv[]) {
    std::cout << argv[0] << " Version " << TUTORIAL_VERSION_MAJOR << "." << TUTORIAL_VERSION_MINOR << std::endl;
}
