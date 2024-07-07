#include "fluid-meter.hpp"
#include "Arduino.h"

volatile int FluidMeter::totalPulses = 0;
FluidMeter* FluidMeter::instance = nullptr;

FluidMeter::FluidMeter(const int pin) : meterPin{pin}, conversionFactor{450} {
  attachInterrupt(digitalPinToInterrupt(2), this->countPulses, RISING);
}

FluidMeter* FluidMeter::getInstance(const int pin) {
  if (FluidMeter::instance == nullptr) {
    FluidMeter::instance = new FluidMeter(pin);
  }

  return FluidMeter::instance;
}

void FluidMeter::countPulses() {
  FluidMeter::totalPulses++;
}

float FluidMeter::getVolume() {
  int pulses;

  // Protect totalPulses by disabling interrupts
  noInterrupts();
  pulses = this->totalPulses;
  this->totalPulses = 0;
  interrupts();

  return pulses / conversionFactor;
}
