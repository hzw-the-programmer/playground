# CMakeList.txt: CMakePro1 的 CMake 项目，在此处包括源代码并定义
# 项目特定的逻辑。
#

# 将源代码添加到此项目的可执行文件。
add_executable (CMakePro1 "CMakePro1.c" "CMakePro1.h")

if (CMAKE_VERSION VERSION_GREATER 3.12)
  set_property(TARGET CMakePro1 PROPERTY CXX_STANDARD 20)
endif()

# TODO: 如有需要，请添加测试并安装目标。

include_directories(../include)
target_link_directories(CMakePro1 PUBLIC ../lib)
target_link_libraries(CMakePro1 mbedtls)
