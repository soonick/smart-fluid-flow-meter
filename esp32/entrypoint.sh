#!/bin/bash

. /esp-idf/export.sh
idf.py set-target esp32
mkdir -p /esp32/build
cd /esp32/build
cmake ..
cd ..

exec "$@"
