cmake_minimum_required(VERSION 3.27.4)
project(SmarFluidFlowMeterSketchTest CXX)

include_directories(include)
include_directories(../src/r4-wifi-manager)
include_directories(external/catch/v2.13.10/include)

set(TEST_TARGET_SRCS
  src/test-main.cpp
  src/r4-wifi-manager/r4-wifi-manager-test.cpp
  ../src/r4-wifi-manager/r4-wifi-manager.cpp
  include/Arduino.cpp
)

add_compile_definitions(HOST)
add_compile_options(-Wall -Wextra -Wpedantic -Werror)

add_executable(
  test
  ${TEST_TARGET_SRCS}
)
