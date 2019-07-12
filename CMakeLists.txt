cmake_minimum_required(VERSION 3.0)
project(fica)

add_compile_definitions(COMPILING_DLL=1)

add_subdirectory(examples/functions)

set(CMAKE_CXX_STANDARD 17)
include_directories(eigen)
add_library(fica SHARED src/capi.cpp src/contrast.cpp src/whitening.cpp src/fastica.cpp
        include/capi.h include/fastica.h include/fica.h include/whitening.h include/contrast.h)