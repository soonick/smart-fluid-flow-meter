#include <r4-wifi-manager/constants.hpp>
#include <r4-wifi-manager/r4-wifi-manager.hpp>
#include "Arduino.h"
#include "Hashtable.h"
#include "button.h"

#define RESET_PIN 2

Hashtable<String, String> userConfig;
R4WifiManager wifiManager;
bool apStarted = false;
bool connectedToWifi = false;
Button resetButton = Button(RESET_PIN);

void setup() {
  Serial.begin(9600);
  pinMode(RESET_PIN, INPUT_PULLUP);
}

void reset() {
  if (resetButton.isLongPressed(5000)) {
    Serial.println("Resetting meter");
    wifiManager.reset();
    apStarted = false;
    connectedToWifi = false;
    userConfig = Hashtable<String, String>();
  }
}

void ap() {
  if (userConfig.elements() != 3) {
    userConfig = wifiManager.getUserConfig();

    // If userConfig wasn't retrieved from eeprom and AP hasn't been already
    // started, try to start it
    if (userConfig.elements() != 3 && !apStarted) {
      Serial.println("Starting AP");
      String error = wifiManager.startAp("my-arduino", "12345678",
                                         IPAddress(192, 48, 56, 2));
      if (!error.isEmpty()) {
        // There was an error. Print the error message forever
        while (true) {
          Serial.println(error);
          delay(1000);
        }
      }

      apStarted = true;
    }

    if (userConfig.elements() == 3) {
      wifiManager.disconnect();
      apStarted = false;
    }
  }
}

void connectToWifi() {
  if (userConfig.elements() == 3 && !connectedToWifi) {
    Serial.println("Connecting to Wifi");
    String err = wifiManager.connect(
        *userConfig.get(R4WifiManagerConstants::NETWORK_KEY),
        *userConfig.get(R4WifiManagerConstants::PASSWORD_KEY));
    if (!err.isEmpty()) {
      while (true) {
        Serial.println(err);
        delay(1000);
      }
    }

    connectedToWifi = true;
  }
}

void loop() {
  reset();
  ap();
  connectToWifi();
}
