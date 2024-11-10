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
wm_config wifi_config;

/**
 * factory_reset task handle
 */
TaskHandle_t factory_reset_handle;

/**
 * Button to factory reset wifi settings
 */
Button reset_button = Button(RESET_BUTTON);

/**
 * Wifi manager
 */
EspIdfWifiManager* wm = nullptr;

/**
 * Saves the wifi config and shuts down access point
 */
void save_config(wm_config in) {
  wifi_config = in;
  wm->shutdown_ap();
  ESP_LOGI(TAG, "Wifi is configured");
}

/**
 * Monitors reset button and performs hard reset if conditions are met
 */
void factory_reset(void* pvParameters) {
  (void)pvParameters;

  while (true) {
    if (!wifi_config.ssid.empty() && reset_button.is_long_pressed(5000)) {
      ESP_LOGI(TAG, "Resetting meter");
      wm->clear_config();
      wifi_config.ssid = "";
      std::optional<wm_config> config_opt = wm->get_config(save_config);
    }
  }
}

/**
 * Turns on the power indicator LED
 */
void power_led() {
  gpio_reset_pin(POWER_LED);
  gpio_set_direction(POWER_LED, GPIO_MODE_OUTPUT);
  ESP_LOGI(TAG, "Turning on power LED");
  gpio_set_level(POWER_LED, 1);
}

extern "C" void app_main() {
  wm = EspIdfWifiManager::get_instance("my-esp32-ssid", "APassword");

  wifi_config.ssid = "";
  std::optional<wm_config> config_opt = wm->get_config(save_config);

  if (config_opt.has_value()) {
    save_config(config_opt.value());
  }

  xTaskCreate(factory_reset, "factory_reset", 4096, nullptr, tskIDLE_PRIORITY,
              &factory_reset_handle);

  power_led();
}
