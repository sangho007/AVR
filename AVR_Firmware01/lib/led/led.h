#ifndef LED_H
#define LED_H

#include <avr/io.h>
#include "util.h"

typedef struct {
    bool state;
} led_defualt_t;


void led_init(void);
void led_off_default(void);
void led_on_default(void);
void led_toggle_default(void);

#endif

