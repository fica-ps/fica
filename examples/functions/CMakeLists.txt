cmake_minimum_required(VERSION 3.0)
project(functions)

set(CMAKE_CXX_STANDARD 17)

include_directories(../../include)
include_directories(../../eigen)

add_executable(
        functions
        functions.cpp
)