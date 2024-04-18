#include <r4-wifi-manager/constants.hpp>
#include <r4-wifi-manager/r4-wifi-manager.hpp>
#include "Arduino.h"
#include "Hashtable.h"
#include "api/Common.h"
#include "button.hpp"
#include "fluid-meter.hpp"

#define RESET_PIN 7
#define SENSOR_PIN 2

const int MILLIS_BETWEEN_POSTS = 600'000;  // 10 minutes

Hashtable<String, String> userConfig;
R4WifiManager wifiManager;
bool apStarted = false;
bool connectedToWifi = false;
Button resetButton = Button(RESET_PIN);
FluidMeter* fluidMeter = nullptr;
int lastPost = millis();

void setup() {
  Serial.begin(9600);
  pinMode(RESET_PIN, INPUT_PULLUP);
  pinMode(SENSOR_PIN, INPUT);
  fluidMeter = FluidMeter::getInstance(SENSOR_PIN);
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

void postMeasurements() {
  if ((millis() - lastPost) > MILLIS_BETWEEN_POSTS) {
    lastPost = millis();
    float litters = fluidMeter->getVolume();
    Serial.print("Read ");
    Serial.print(litters);
    Serial.println(" litters");
  }
}

void loop() {
  reset();
  ap();
  connectToWifi();
  postMeasurements();
}
