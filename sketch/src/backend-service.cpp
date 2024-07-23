#include "backend-service.hpp"

bool BackendService::postMeasurement(const String& deviceId,
                                     const float litters) {
  char bodyBuffer[120];
  sprintf(bodyBuffer, "{\"device_id\":\"%s\",\"measure\":\"%f\"}",
          deviceId.c_str(), litters);
  client.post(MEASURE_PATH, CONTENT_TYPE, bodyBuffer);
  int statusCode = client.responseStatusCode();
  client.stop();

  return statusCode == 200;
}
