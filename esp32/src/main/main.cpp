#include "wifi_manager.hpp"

extern "C" void app_main() {
  WifiManager("my-esp32-ssid", "APassword");
}
