#include "timer.h"

static Task_t tasks[10];
static uint8_t task_count = 0;
static volatile uint16_t system_time = 0;

void timer_init(void) {
    TCCR0B = (1<<CS02) | (1<<CS00);  // 1024 분주비
    TCNT0 = 0xB2;                    // 10ms 주기
    TIMSK0 = (1<<TOIE0);              // 타이머0 오버플로우 인터럽트 활성화
}

void task_add(void (*task)(void), uint16_t period) {
    if (task_count < 10) {
        tasks[task_count].task = task;
        tasks[task_count].period = period;
        tasks[task_count].next_run = period;
        tasks[task_count].ready = 0;
        task_count++;
    }
}

ISR(TIMER0_OVF_vect) {
    TCNT0 = 0xB2;
    system_time += 10;
    
    for (uint8_t i = 0; i < task_count; i++) {
        if (system_time >= tasks[i].next_run) {
            tasks[i].ready = 1;
            tasks[i].next_run = system_time + tasks[i].period;
        }
    }
}

void task_run(void) {
    for (uint8_t i = 0; i < task_count; i++) {
        if (tasks[i].ready) {
            tasks[i].task();
            tasks[i].ready = 0;
        }
    }
}

