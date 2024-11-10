// Standard library
#include <optional>

// Esp-idf
#include <driver/gpio.h>
#include <esp_log.h>

// Project components
#include "button.hpp"
#include "esp_idf_wifi_manager.hpp"

#define POWER_LED GPIO_NUM_32
#define RESET_BUTTON GPIO_NUM_18

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
 * Button to factory reset wifi settings
 */
Button reset_button = Button(RESET_BUTTON);

/**
 * Monitors reset button and performs hard reset if conditions are met
 */
void factory_reset(void* pvParameters) {
  (void)pvParameters;

  while (true) {
    if (reset_button.is_long_pressed(5000)) {
      ESP_LOGE(TAG, "Resetting meter");
      // TODO: reset wifi manager
    }
  }
}

/**
 * Turns on the power indicator LED
 */
void power_led() {
  gpio_reset_pin(POWER_LED);
  gpio_set_direction(POWER_LED, GPIO_MODE_OUTPUT);
  gpio_set_level(POWER_LED, 1);
}

extern "C" void app_main() {
  EspIdfWifiManager wm = EspIdfWifiManager("my-esp32-ssid", "APassword");

  std::optional<wm_config> config_opt =
      wm.get_config([](wm_config in) { config = in; });

  if (config_opt.has_value()) {
    config = config_opt.value();
  }

  xTaskCreate(factory_reset, "factory_reset", 4096, nullptr, tskIDLE_PRIORITY,
              &factory_reset_handle);

  power_led();
}
