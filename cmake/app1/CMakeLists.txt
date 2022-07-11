cmake_minimum_required(VERSION 3.18)

project(hproj)

file(GLOB_RECURSE TMP_FILES
    RELATIVE ${PROJECT_SOURCE_DIR}
    *.c
    *.h)
list(FILTER TMP_FILES EXCLUDE REGEX "^build.*")

message(STATUS "${TMP_FILES}")

add_executable(Main ${TMP_FILES})

foreach(source IN LISTS TMP_FILES)
    get_filename_component(source_path "${source}" PATH)
    string(REPLACE "/" "\\" source_path_msvc "${source_path}")
    source_group("${source_path_msvc}" FILES "${source}")
endforeach()
