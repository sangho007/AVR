#!/bin/bash
# flash.sh
avrdude -v \
    -p atmega2560 \
    -c wiring \
    -P /dev/tty.usbmodem211301 \
    -b 115200 \
    -D \
    -U flash:w:$1:i
# avrdude -v \
#     -p atmega2560 \
#     -c wiring \
#     -P /dev/tty.usbserial-21130 \
#     -b 115200 \
#     -D \
#     -U flash:w:$1:i