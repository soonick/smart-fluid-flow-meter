#include "Arduino.h"
#include <iostream>

void delay(int) {}

void HardwareSerial::print(String a) {
  std::cout << a;
}
void HardwareSerial::println(String a) {
  std::cout << a;
}
void HardwareSerial::println(IPAddress a) {
  std::cout << &a;
}

HardwareSerial Serial;
