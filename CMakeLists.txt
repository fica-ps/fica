cmake_minimum_required(VERSION 3.14)
project(fica)

set(CMAKE_CXX_STANDARD 17)

find_package(Eigen3 3.3 REQUIRED NO_MODULE)

add_library(fica SHARED include/fica.h src/fastica.cpp include/fastica.h src/contrast.cpp include/contrast.h src/capi.cpp include/capi.h src/whitening.cpp include/whitening.h)

target_link_libraries (fica Eigen3::Eigen)

add_subdirectory(test)