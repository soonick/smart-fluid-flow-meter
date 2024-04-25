#include <r4-wifi-manager.h>
#include <catch.hpp>

SCENARIO("Arduino Cloud Properties are encoded",
         "[ArduinoCloudThing::encode-1]") {
  WHEN("A 'bool' property is added") {
    R4WifiManager wm;
    String s = wm.getKeyValues("hello?abc=1");
    REQUIRE(s == "abc=1,");
  }
}
