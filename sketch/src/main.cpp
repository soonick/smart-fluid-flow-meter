#include <r4-wifi-manager/r4-wifi-manager.hpp>
#include "Arduino.h"

R4WifiManager wifiManager;

void setup() {
  Serial.begin(9600);
  String error =
      wifiManager.startAp("my-arduino", "12345678", IPAddress(192, 48, 56, 2));
  if (!error.isEmpty()) {
    // There was an error. Print the error message forever
    while (true) {
      Serial.println(error);
      delay(1000);
    }
  }
}

void loop() {
  // wifiManager.listen();
}
