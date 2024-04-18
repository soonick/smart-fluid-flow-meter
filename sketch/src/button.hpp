#pragma once

class Button {
 public:
  /**
   * Reads button state in the given pin. Buttons are treated as normally high
   * (i.e. When they are not pressed, their value is 1, when they are pressed,
   * their value is 0)
   *
   * @param pin - Pin where events are going to be read from
   */
  Button(const int pin);

  /**
   * Returns true if the button is currently being pressed
   */
  bool isPressed();

  /**
   * Returns true if the button has been pressed for at least `millis` amount of
   * miliseconds
   */
  bool isLongPressed(const int ms);

 private:
  const int buttonPin;
  unsigned int pressedSince = 0;
};
