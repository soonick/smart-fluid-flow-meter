#include "eeprom_storage.h"

#include <EEPROM.h>

std::map<std::string, std::string> EepromStorage::readSettings() {
  std::map<std::string, std::string> data;
  EEPROM.get(EepromStorage::ADDRESS, data);

  return data;
}
