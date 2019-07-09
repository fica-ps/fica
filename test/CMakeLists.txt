enable_testing()

include_directories (../src)
include_directories(../include)

add_executable(uTests tests.cpp)

target_link_libraries (uTests fica Eigen3::Eigen)

add_test(uTests uTests)