#pragma once

#include "Arduino.h"

class R4WifiManager {
 public:
  String getKeyValues(const String& firstLine);
};
