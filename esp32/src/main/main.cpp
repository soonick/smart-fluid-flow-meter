#include "esp_idf_wifi_manager.hpp"

// #include <freertos/idf_additions.h>
#include <esp_log.h>

#include <optional>

/**
 * Used for logging
 */
const char* TAG = "smart-fluid-flow-meter";

/**
 * WiFi configuration
 */
wm_config config;

/**
 * factory_reset task handle
 */
TaskHandle_t factory_reset_handle;

/**
 * Monitors reset button and performs hard reset if conditions are met
 */
void factory_reset(void* pvParameters) {
  (void)pvParameters;

  while (true) {
    vTaskDelay(3000 / portTICK_PERIOD_MS);  // Blink every 3 seconds
    ESP_LOGE(TAG, "config.ssid = %s", config.ssid.c_str());
  }
}

/**
 * Connects to WiFi networks as soon as we have wifi configuration available
 */
void connect_to_wifi(void* pvParameters) {}

extern "C" void app_main() {
  EspIdfWifiManager wm = EspIdfWifiManager("my-esp32-ssid", "APassword");

  std::optional<wm_config> config_opt =
      wm.get_config([](wm_config in) { config = in; });

  if (config_opt.has_value()) {
    config = config_opt.value();
  }

  xTaskCreate(factory_reset, "factory_reset", 4096, nullptr, tskIDLE_PRIORITY,
              &factory_reset_handle);
}
