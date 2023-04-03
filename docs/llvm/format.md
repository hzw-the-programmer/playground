unzip LLVM-16.0.0-win64.exe
copy LLVM-16.0.0-win64\bin\clang-format.exe
copy LLVM-16.0.0-win64\bin\VCRUNTIME140_1.dll
add to path

clang-format.exe --help
clang-format -style=llvm -i xxx.c
// .clang-format
clang-format.exe --style=file -i xxx.c
clang-format.exe -i xxx.c
