cmake_minimum_required(VERSION 3.27.4)
project(SmarFluidFlowMeterSketch CXX)
add_executable(main src/main.cpp)

# Arduino.h and WiFi.h are included in main.cpp, so link them
target_link_arduino_libraries(main PRIVATE WiFi core)

# This is needed for the generation of HEX binary and uploading it
target_enable_arduino_upload(main)
