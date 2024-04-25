#include "WiFiS3.h"
#include <iostream>

WiFiServer::WiFiServer(int port) {
  std::cout << port;
}
WiFiClient WiFiServer::available() {
  WiFiClient client;
  return client;
}
void WiFiServer::begin() {}

WiFiClient::operator bool() const {
  return true;
}
bool WiFiClient::available() {
  return true;
}
bool WiFiClient::connected() {
  return true;
}
void WiFiClient::println() {}
void WiFiClient::println(String a) {
  std::cout << a;
}
void WiFiClient::print(String a) {
  std::cout << a;
}
char WiFiClient::read() {
  return 'a';
}
void WiFiClient::stop() {}

String HardwareWifi::SSID() {
  return "";
}
IPAddress HardwareWifi::localIP() {
  IPAddress address;
  return address;
}
void HardwareWifi::config(IPAddress ip) {
  std::cout << &ip;
}
int HardwareWifi::beginAP(String ssid, String) {
  if (ssid == "badssid") {
    return WL_IDLE_STATUS;
  }
  return WL_AP_LISTENING;
}
int HardwareWifi::status() {
  return WL_AP_LISTENING;
}

HardwareWifi WiFi;
