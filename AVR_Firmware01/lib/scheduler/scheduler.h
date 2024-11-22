#ifndef TIMER_H
#define TIMER_H

#include <avr/io.h>
#include <avr/interrupt.h>

typedef struct {
    void (*task)(void);
    uint16_t period;
    uint16_t next_run;
    uint8_t ready;
} Task_t;

void timer_init(void);
void task_add(void (*task)(void), uint16_t period);
void task_run(void);

#endif

