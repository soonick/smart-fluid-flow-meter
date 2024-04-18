// int SENSOR_PIN = 2;
// const float CONVERSION_FACTOR = 450;
//
// float totalLitters = 0;
// long timeLastRun = 0;
//
// volatile int totalPulses;
//
// void countPulse() {
//   totalPulses++;
// }
//
// int getPulsesPerSecond() {
//   delay(1000);
//   int pulses;
//
//   // Protect totalPulses by disabling interrupts
//   noInterrupts();
//   pulses = totalPulses;
//   totalPulses = 0;
//   interrupts();
//
//   return pulses;
// }
//
// void setup() {
//   Serial.begin(9600);
//   pinMode(SENSOR_PIN, INPUT);
//   attachInterrupt(digitalPinToInterrupt(2), countPulse, RISING);
//   timeLastRun = millis();
// }
//
// void loop() {
//   float pulsesPerSecond = getPulsesPerSecond();
//   float littersPerSecond = pulsesPerSecond / CONVERSION_FACTOR;
//   float secondsSinceLastRun = (millis() - timeLastRun) / 1000.0;
//   timeLastRun = millis();
//   totalLitters += littersPerSecond * secondsSinceLastRun;
//
//   Serial.print("Total litters: ");
//   Serial.print(totalLitters);
//   Serial.println(" L");
// }


#include <WiFi.h>

char ssid[] = "ssid";
char pass[] = "pass";
int status = WL_IDLE_STATUS;

void setup() {
  Serial.begin(9600);

  WiFi.config(INADDR_NONE, INADDR_NONE, INADDR_NONE, INADDR_NONE);
  WiFi.setHostname("smart-water-meter");

  // attempt to connect to Wifi network:
  while (status != WL_CONNECTED) {
    Serial.print("Attempting to connect to WPA SSID: ");
    Serial.println(ssid);

    status = WiFi.begin(ssid, pass);

    // Allow some time for the connection
    delay(3000);
  }

  Serial.print("You're connected to the network");
}

void loop() {
  printCurrentNet();
  printWifiData();
  delay(10000);
}

void printWifiData() {
  IPAddress ip = WiFi.localIP();

  Serial.print("IP Address: ");
  Serial.println(ip);

  byte mac[6];
  WiFi.macAddress(mac);
  Serial.print("MAC address: ");
  Serial.print(mac[5], HEX);
  Serial.print(":");
  Serial.print(mac[4], HEX);
  Serial.print(":");
  Serial.print(mac[3], HEX);
  Serial.print(":");
  Serial.print(mac[2], HEX);
  Serial.print(":");
  Serial.print(mac[1], HEX);
  Serial.print(":");
  Serial.println(mac[0], HEX);
  Serial.println();
}

void printCurrentNet() {
  Serial.print("SSID: ");
  Serial.println(WiFi.SSID());

  byte bssid[6];
  WiFi.BSSID(bssid);
  Serial.print("BSSID: ");
  Serial.print(bssid[5], HEX);
  Serial.print(":");
  Serial.print(bssid[4], HEX);
  Serial.print(":");
  Serial.print(bssid[3], HEX);
  Serial.print(":");
  Serial.print(bssid[2], HEX);
  Serial.print(":");
  Serial.print(bssid[1], HEX);
  Serial.print(":");
  Serial.println(bssid[0], HEX);

  long rssi = WiFi.RSSI();
  Serial.print("signal strength (RSSI):");
  Serial.println(rssi);

  // print the encryption type:
  byte encryption = WiFi.encryptionType();
  Serial.print("Encryption Type:");
  Serial.println(encryption, HEX);
  Serial.println();
}
