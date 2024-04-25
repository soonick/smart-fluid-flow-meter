cmake_minimum_required(VERSION 3.27.4)
project(SmarFluidFlowMeterSketchTest CXX)

include_directories(include)
include_directories(external/catch/v2.13.10/include)

set(TESTING_SRCS
)

set(INCLUDE_SRCS
  include/WiFiS3.cpp
  include/Arduino.cpp
)

set(TEST_TARGET_SRCS
  src/test-main.cpp
)

add_compile_definitions(HOST)
add_compile_options(-Wall -Wextra -Wpedantic -Werror)

add_executable(
  test
  ${TEST_TARGET_SRCS}
  ${INCLUDE_SRCS}
  ${TESTING_SRCS}
)
