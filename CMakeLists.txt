cmake_minimum_required(VERSION 3.10)

project(fica)

set(CMAKE_CXX_STANDARD 17)

if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif()

set(CMAKE_CXX_FLAGS_DEBUG "-g")
set(CMAKE_CXX_FLAGS_RELEASE "-O3")
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

find_package (Eigen3 3.3 REQUIRED NO_MODULE)

add_library(fica SHARED include/fica.h src/cpp/fastica.cpp include/fastica.h src/cpp/contrast.cpp include/contrast.h src/cpp/capi.cpp include/capi.h src/cpp/whitening.cpp include/whitening.h)

target_link_libraries (fica Eigen3::Eigen)
