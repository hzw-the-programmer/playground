# CMakeList.txt: callrust 的 CMake 项目，在此处包括源代码并定义
# 项目特定的逻辑。
#

# 将源代码添加到此项目的可执行文件。

# cargo install --force cbindgen
# cbindgen.exe --lang c -o mylib.h

add_executable (callrust "callrust.c" "callrust.h" "mylib.h")

# cargo rustc -- --crate-type staticlib --print=native-static-libs
target_link_libraries(callrust ${CMAKE_SOURCE_DIR}/mylib.lib kernel32 advapi32 bcrypt ntdll userenv ws2_32 msvcrt)

if (CMAKE_VERSION VERSION_GREATER 3.12)
  set_property(TARGET callrust PROPERTY CXX_STANDARD 20)
endif()

# TODO: 如有需要，请添加测试并安装目标。
