#pragma once

#include "Arduino.h"
#include <ArduinoHttpClient.h>
#include <WiFiS3.h>

class BackendService {
 public:
  /**
   * Returns true if the measurement was posted successfully, false otherwise
   */
  bool postMeasurement(const String &deviceId, const float litters);

 private:
  const String CONTENT_TYPE = "application/json";
  const String MEASURE_PATH = "/measure";
  const String HOST_NAME = "smart-fluid-flow-meter-backend.ncona.com";
  const int HTTP_PORT = 443;
  WiFiSSLClient wifi;
  HttpClient client = HttpClient(wifi, HOST_NAME, HTTP_PORT);
};
