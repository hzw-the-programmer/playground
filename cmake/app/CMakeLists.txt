cmake_minimum_required(VERSION 3.10)

project(hproj)

add_subdirectory(hzw)

add_executable(hexe happ.c happ1.c app.c)

target_link_libraries(hexe PUBLIC hzwl)

target_include_directories(hexe PUBLIC
                          "${PROJECT_BINARY_DIR}"
                          "${PROJECT_SOURCE_DIR}/hzw"
                          )