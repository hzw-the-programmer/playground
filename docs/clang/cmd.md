find . -iname "*.c" -o -iname "*.h" | xargs clang-format.exe -i
clang-format -style=llvm -dump-config > .clang-format

// clang-format off
// clang-format on

