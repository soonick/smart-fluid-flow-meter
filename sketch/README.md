# Sketch

The Arduino Sketch for `smart-fluid-flow-meter`.

Reads measurements from [YF-S201](http://www.mantech.co.za/datasheets/products/yf-s201_sea.pdf) and publishes them to a back-end.

Currently only tested with [Arduino UNO R4 WiFi](https://store.arduino.cc/products/uno-r4-wifi).

## Compile

To generate a docker image and build the code:

```
make build
```

## Upload to Arduino

Assumes the device is connected at `/dev/ttyACM0`.

```
make upload
```

## Monitor serial port

Assumes the device is connected at `/dev/ttyACM0`.

```
make serial
```
