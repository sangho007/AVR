#include "scheduler.h"
#include "led.h"
#include "util.h"
#include "motorshield.h"
#include <util/delay.h>

void task_0ms(void) {
    // 0ms 마다 실행될 태스크
}

void task_10ms(void) {
    // 10ms 마다 실행될 태스크
}

void task_50ms(void) {
    // 50ms 마다 실행될 태스크
}

void task_100ms(void) {
    // 100ms 마다 실행될 태스크
}

void task_1000ms(void) {
    // 1000ms 마다 실행될 태스크

}

int main(void) {
    timer_init();
	led_init();

    sei();
    
    task_add(task_0ms, 0);  // 최대한 빠르게 실행
    task_add(task_10ms, 10);  // 10ms 주기 태스크 추가
    task_add(task_50ms, 50);  // 50ms 주기 태스크 추가
    task_add(task_100ms, 100);  // 100ms 주기 태스크 추가
    task_add(task_1000ms, 1000);  // 100ms 주기 태스크 추가

    while(1) {
        scheduler_run();
    }
    
    return 0;
}

