#pragma once

#include "Arduino.h"

enum {
  WL_IDLE_STATUS = 0,
  WL_AP_LISTENING = 1,
  WL_AP_CONNECTED = 2,
};

struct IPAddress {};

class WiFiClient {
 public:
  explicit operator bool() const;
  bool available();
  bool connected();
  void print(String);
  void println();
  void println(String);
  char read();
  void stop();
};

class WiFiServer {
 public:
  WiFiServer(int);

  WiFiClient available();
  void begin();
};

class HardwareWifi {
 public:
  IPAddress localIP();
  String SSID();
  int beginAP(String, String);
  int status();
  void config(IPAddress);
};

extern HardwareWifi WiFi;
