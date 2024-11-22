#include "scheduler.h"

static Task_t tasks[10];
static uint8_t task_count = 0;
static volatile uint16_t system_time = 0;

void timer_init(void) {
    // CTC 모드 사용
    TCCR0A = (1 << WGM01);              // CTC 모드
    TCCR0B = (1 << CS01) | (1 << CS00); // 64 분주비
    OCR0A = 249;                        // 16MHz / 64 / (249+1) = 1000Hz (1ms)
    TIMSK0 = (1 << OCIE0A);             // Output Compare A Match 인터럽트 활성화
}

void task_add(void (*task)(void), uint16_t period) {
    if (task_count < 10) {
        tasks[task_count].task = task;
        tasks[task_count].period = period;
        tasks[task_count].next_run = period;
        tasks[task_count].ready = 0;
        
        // 연속 실행 태스크 표시 (period가 0인 경우)
        if (period == 0) {
            tasks[task_count].ready = 1;  // 항상 실행 가능 상태
        }
        
        task_count++;
    }
}

ISR(TIMER0_COMPA_vect) {
    system_time++;
    
    for (uint8_t i = 0; i < task_count; i++) {
        if (tasks[i].period > 0 && system_time >= tasks[i].next_run) {
            tasks[i].ready = 1;
            tasks[i].next_run = system_time + tasks[i].period;
        }
    }
}

void scheduler_run(void) {
    for (uint8_t i = 0; i < task_count; i++) {
        if (tasks[i].ready) {
            tasks[i].task();
            // period가 0인 태스크는 ready 플래그를 리셋하지 않음
            if (tasks[i].period > 0) {
                tasks[i].ready = 0;
            }
        }
    }
}




