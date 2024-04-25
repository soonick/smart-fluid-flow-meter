cmake_minimum_required(VERSION 3.27.4)
project(SmarFluidFlowMeterSketch CXX)

add_library(r4-wifi-manager src/r4-wifi-manager/r4-wifi-manager.cpp)
target_link_arduino_libraries(r4-wifi-manager PRIVATE core)

add_executable(main src/main.cpp)
# Arduino.h and WiFi.h are included in main.cpp, so link them
target_link_arduino_libraries(main PRIVATE core)
target_link_libraries(main PRIVATE r4-wifi-manager)

# This is needed for the generation of HEX binary and uploading it
target_enable_arduino_upload(main)
