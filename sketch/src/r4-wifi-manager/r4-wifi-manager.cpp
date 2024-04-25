#include "r4-wifi-manager.h"

// Parses the first line of a request to /save
// (e.g. GET /save?name=Blue+bird&value=jose&name=Uno&value=One HTTP/1.1)
// to be a comma separated list of key values
// (e.g. Blue bird=jose,Uno=One)
// It takes care of some URL decodings, but probably not all of them. For now it
// naively assumes there are no commans in keys or values
String R4WifiManager::getKeyValues(const String& firstLine) {
  Serial.println("DELETE ME ##############################");
  int start = firstLine.indexOf("?") + 1;
  int end = firstLine.indexOf(" ", start);
  String queryString = firstLine.substring(start, end);

  String result = "";
  long unsigned int currentStart = 0;
  while (currentStart < queryString.length()) {
    int currentEnd = queryString.indexOf("&", currentStart);
    if (currentEnd == -1) {
      currentEnd = queryString.length();
    }

    String currentPair = queryString.substring(currentStart, currentEnd);
    result += currentPair + ",";

    currentStart = currentEnd;
  }

  return result;
}
