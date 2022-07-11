cmake_minimum_required(VERSION 3.18)

project(hproj)

set(build_dir build)
set(os xp)
set(os_prefix os_)

file(GLOB_RECURSE src_list
    RELATIVE ${PROJECT_SOURCE_DIR}
    *.c
    *.h)
list(FILTER src_list EXCLUDE REGEX "^${build_dir}.*")

foreach(src IN LISTS src_list)
    message(${src})
endforeach()
message("")

set(platform_list ${src_list})
list(FILTER platform_list INCLUDE REGEX ".*${os_prefix}${os}.*")

foreach(src IN LISTS platform_list)
    message(${src})
endforeach()
message("")

list(FILTER src_list EXCLUDE REGEX ".*${os_prefix}.*")

foreach(src IN LISTS src_list)
    message(${src})
endforeach()
message("")

list(APPEND src_list ${platform_list})

foreach(src IN LISTS src_list)
    message(${src})
endforeach()
message("")

add_executable(hexe ${src_list})

foreach(src IN LISTS src_list)
    if(src MATCHES ".*/.*")
        get_filename_component(path "${src}" PATH)
        list(APPEND inc_list ${path})

        string(REPLACE "/" "\\" msvc_path "${path}")
        source_group("${msvc_path}" FILES "${src}")

        if(path MATCHES ".*/.*")
            get_filename_component(path "${path}" PATH)
            list(APPEND inc_list ${path})
        endif()
    else()
        list(APPEND inc_list ".")
        source_group("" FILES "${src}")
    endif()
endforeach()

message("")

list(REMOVE_DUPLICATES inc_list)

foreach(src IN LISTS inc_list)
    message("/I ${src}")
endforeach()
message("")

target_include_directories(hexe PUBLIC ${inc_list})
