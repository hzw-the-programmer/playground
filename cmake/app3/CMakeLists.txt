
project(app3)

message(STATUS "Source Dir: ${CMAKE_CURRENT_SOURCE_DIR}")
message(STATUS "Binary Dir: ${CMAKE_BINARY_DIR}")
message(STATUS "Host System name: ${CMAKE_HOST_SYSTEM_NAME}")
message(STATUS "CMAKE Version: ${CMAKE_VERSION}")

message(STATUS "System name: ${CMAKE_SYSTEM_NAME}")
message(STATUS "System version: ${CMAKE_SYSTEM_VERSION}")
message(STATUS "Platform version: ${CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION}")
message(STATUS "Build type: ${CMAKE_BUILD_TYPE}")
message(STATUS "CMAKE Configuration types: ${CMAKE_CONFIGURATION_TYPES}")
message(STATUS "WIN32: ${WIN32}")

message(STATUS "Project Source dir: ${PROJECT_SOURCE_DIR}")

message(STATUS "C Compiler version: ${CMAKE_C_COMPILER_VERSION}")
message(STATUS "C Compiler id: ${CMAKE_C_COMPILER_ID}")
message(STATUS "CXX Compiler version: ${CMAKE_CXX_COMPILER_VERSION}")
message(STATUS "CXX Compiler id: ${CMAKE_CXX_COMPILER_ID}")
message(STATUS "Archive Output directory: ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}")

message(STATUS "C Flags release: ${CMAKE_C_FLAGS_RELEASE}")
message(STATUS "C Flags debug: ${CMAKE_C_FLAGS_DEBUG}")
message(STATUS "CXX Flags release: ${CMAKE_CXX_FLAGS_RELEASE}")
message(STATUS "CXX Flags debug: ${CMAKE_CXX_FLAGS_DEBUG}")

message(STATUS "Shared Linker flags release: ${CMAKE_SHARED_LINKER_FLAGS_RELEASE}")
message(STATUS "Exe Linker flags release: ${CMAKE_EXE_LINKER_FLAGS_RELEASE}")

message(STATUS "")
message(STATUS "MSVC Runtime library: ${CMAKE_MSVC_RUNTIME_LIBRARY}")
set(tmp "MultiThreaded$<$<CONFIG:Debug>:Debug>")
message(STATUS "MSVC Runtime library: ${tmp}")
