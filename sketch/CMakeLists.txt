cmake_minimum_required(VERSION 3.27.4)
project(SmarFluidFlowMeterSketch CXX)

add_subdirectory("deps/r4-wifi-manager")

add_library(ArduinoHttpClient
  "deps/ArduinoHttpClient/src/HttpClient.cpp")
target_link_arduino_libraries(ArduinoHttpClient PUBLIC core)
target_include_directories(ArduinoHttpClient PUBLIC "deps/ArduinoHttpClient/src/")

add_executable(main
  "src/main.cpp"
  "src/backend-service.cpp"
  "src/button.cpp"
  "src/fluid-meter.cpp")
target_link_arduino_libraries(main PRIVATE core WiFiS3)
target_link_libraries(main PRIVATE r4-wifi-manager ArduinoHttpClient)

# This is needed for the generation of HEX binary and uploading it
target_enable_arduino_upload(main)
