#include "led.h"

static led_defualt_t led;

void led_init(void){
	DDRB |= (1 << 7);
    led.state = false;
}

void led_off_default(void){
	PORTB &= ~(1 << 7);
    led.state = false;
}

void led_on_default(void){
	PORTB |= (1 << 7);
    led.state = true;
}

void led_toggle_default(void){
    if(led.state == false){
        PORTB |= (1 << 7);
        led.state = true;
    }
    else if(led.state == true){
        PORTB &= ~(1 << 7);
        led.state = false;
    }
}


