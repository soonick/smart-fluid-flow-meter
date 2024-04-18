#include "button.hpp"
#include "Arduino.h"

Button::Button(const int pin) : buttonPin{pin} {}

bool Button::isPressed() {
  return !digitalRead(buttonPin);
}

bool Button::isLongPressed(const int ms) {
  if (isPressed()) {
    if (pressedSince == 0) {
      pressedSince = millis();
    }

    return (millis() - pressedSince) > ms;
  } else {
    pressedSince = 0;
    return false;
  }
}
