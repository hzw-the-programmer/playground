# PATH=$PATH:/d/tools/cmake-3.18.2-win64-x64/bin
# cmake.exe -S . -B build
# or
# mkdir build
# cd build
# cmake.exe ..
# cmake --build .

cmake_minimum_required(VERSION 3.10)

project(Tutorial VERSION 1.0)

configure_file(TutorialConfig.h.in TutorialConfig.h)

add_executable(Tutorial tutorial.cxx)

target_include_directories(Tutorial PUBLIC "${PROJECT_BINARY_DIR}")
