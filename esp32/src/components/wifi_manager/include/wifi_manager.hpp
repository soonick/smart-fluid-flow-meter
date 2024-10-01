#pragma once

#include "esp_err.h"
#include "esp_http_server.h"
#include "string"

class WifiManager {
public:
  static const char *TAG;

  WifiManager(const std::string ap_ssid, const std::string ap_password);

private:
  const std::string ap_ssid;
  const std::string ap_password;
  const httpd_uri_t config_page = {.uri = "/",
                                   .method = HTTP_GET,
                                   .handler = config_page_handler,
                                   .user_ctx = NULL};

  static esp_err_t http_404_error_handler(httpd_req_t *req,
                                          httpd_err_code_t err);
  static esp_err_t config_page_handler(httpd_req_t *req);
  static void dns_server_task(void *pvParameters);

  void init_ap();
  void start_web_server();
  void start_dns_server();
  void string_to_uint8_array(const std::string& str, uint8_t *arr);
};
