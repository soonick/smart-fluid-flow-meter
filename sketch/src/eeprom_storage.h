#pragma once

#include <map>

class EepromStorage {
 public:
  std::map<std::string, std::string> readSettings();

 private:
  static const int ADDRESS = 0;
};
