cmake_minimum_required(VERSION 3.27.4)
project(SmarFluidFlowMeterSketch CXX)

add_subdirectory("deps/r4-wifi-manager")

add_executable(main
  "src/main.cpp"
  "src/button.cpp")
target_link_arduino_libraries(main PRIVATE core WiFiS3)
target_link_libraries(main PRIVATE r4-wifi-manager)

# This is needed for the generation of HEX binary and uploading it
target_enable_arduino_upload(main)
