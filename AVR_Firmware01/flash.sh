#!/bin/bash
# flash.sh
avrdude -v \
    -p atmega2560 \
    -c wiring \
    -P /dev/tty.usbmodem11301 \
    -b 115200 \
    -D \
    -U flash:w:$1:i
