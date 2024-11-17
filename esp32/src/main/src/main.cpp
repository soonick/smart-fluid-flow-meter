// Standard library
#include <optional>

// Esp-idf
#include <driver/gpio.h>
#include <esp_log.h>

// Project components
#include "backend-service.hpp"
#include "button.hpp"
#include "esp_idf_wifi_manager.hpp"
#include "fluid-meter.hpp"

#define POWER_LED GPIO_NUM_32
#define RESET_BUTTON GPIO_NUM_18
#define FLOW_SENSOR GPIO_NUM_26

/**
 * Used for logging
 */
const char* TAG = "smart-fluid-flow-meter";

/**
 * Measurements will be posted after this amount of time has passed since last
 * post
 * TODO: Change value
 */
const int MS_BETWEEN_POSTS = 10'000;

/**
 * WiFi configuration
 */
wm_config wifi_config;

/**
 * factory_reset task handle
 */
TaskHandle_t factory_reset_handle;

/**
 * post_measurements task handle
 */
TaskHandle_t post_measurements_handle;

/**
 * Button to factory reset wifi settings
 */
Button reset_button = Button(RESET_BUTTON);

/**
 * Wifi manager
 */
EspIdfWifiManager* wm = nullptr;

/**
 * Backend service
 */
BackendService* bs = nullptr;

/**
 * Fluid meter
 */
FluidMeter* fluid_meter = nullptr;

/**
 * Last time measurements were posted to backend
 */
uint64_t last_post = 0;

/**
 * Saves the wifi config and shuts down access point
 */
void save_config(wm_config in) {
  wifi_config = in;
  bs = BackendService::get_instance(in.ssid, in.password);
  wm->shutdown_ap();
  ESP_LOGI(TAG, "Wifi credentials set in eeprom");
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
      delete bs;
      bs = nullptr;
      std::optional<wm_config> config_opt = wm->get_config(save_config);
    }
  }
}

/**
 * Reads measurements from fluid meter and posts them to backend on the
 * specified cadence
 */
void post_measurements(void* pvParameters) {
  (void)pvParameters;

  while (true) {
    uint64_t current_millis = esp_timer_get_time() / 1000;
    if ((current_millis - last_post) > MS_BETWEEN_POSTS) {
      last_post = current_millis;
      // TODO: save litters information somewhere so it can be sent again when
      // network comes back up
      const float litters = fluid_meter->get_volume();
      if (bs == nullptr) {
        ESP_LOGE(TAG, "Backend service is nullptr");
      } else {
        bs->post_measurement(wifi_config.id, litters);
      }
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

  fluid_meter = FluidMeter::get_instance(FLOW_SENSOR);
  xTaskCreate(post_measurements, "post_measurements", 4096, nullptr,
              tskIDLE_PRIORITY, &post_measurements_handle);

  power_led();
}
