cmake_minimum_required (VERSION 2.6)
project (Tutorial)
add_executable (Tutorial tutorial.cxx)

set (Tutorial_VERSION_MAJOR 1)
set (Tutorial_VERSION_MINOR 0)
configure_file (
  "${PROJECT_SOURCE_DIR}/TutorialConfig.h.in"
  "${PROJECT_BINARY_DIR}/TutorialConfig.h"
)

include_directories("${PROJECT_BINARY_DIR}")

if (USE_MYMATH)
  add_subdirectory (MathFunctions)
  include_directories ("${PROJECT_SOURCE_DIR}/MathFunctions")
  target_link_libraries (Tutorial MathFunctions)
endif (USE_MYMATH)

option (USE_MYMATH "Use tutorial provided math implementation" ON)
