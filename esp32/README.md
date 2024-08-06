# ESP32

The firmware for `smart-fluid-flow-meter`.

Reads measurements from [YF-S201](http://www.mantech.co.za/datasheets/products/yf-s201_sea.pdf) and publishes them to the back-end.

## Compile

To generate a docker image and build the code:

```
make build
```

## Upload to ESP32 dev board

Assumes the device is connected at `/dev/ttyACM0`.

```
make upload
```

## Monitor serial port

Assumes the device is connected at `/dev/ttyACM0`.

```
make serial
```
