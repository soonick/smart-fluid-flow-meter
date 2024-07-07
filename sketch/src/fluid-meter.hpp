#pragma once

/**
 * This is a singleton so the Service Interrupt Routine can be a method
 */
class FluidMeter {
 public:
  /**
   * Delete this constructos to make sure there can only be one instance
   */
  FluidMeter(FluidMeter const&) = delete;
  void operator=(FluidMeter const&) = delete;

  /**
   * Reads measurements from given pin
   *
   * @param pin - Pin where measurements are going to be read from
   */
  static FluidMeter* getInstance(const int pin);

  /**
   * Returns the number of litters that have been measured since last run
   */
  float getVolume();

 private:
  const int meterPin;
  const float conversionFactor;

  static FluidMeter* instance;
  static volatile int totalPulses;

  FluidMeter(const int pin);

  /**
   * Fluid meter interrupt callback. Counts the pulses emitted by the meter
   */
  static void countPulses();
};
