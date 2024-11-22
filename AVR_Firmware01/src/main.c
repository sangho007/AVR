#include "timer.h"
#include "led.h"
#include "util.h"

void task_10ms(void) {
    // 10ms 마다 실행될 태스크
}

void task_50ms(void) {
    // 50ms 마다 실행될 태스크
}

void task_100ms(void) {
    // 100ms 마다 실행될 태스크
}

void task_500ms(void) {
    // 500ms 마다 실행될 태스크
}

void task_1000ms(void) {
    // 1000ms 마다 실행될 태스크
}



int main(void) {
    timer_init();
	led_init();

    sei();
    
    task_add(task_10ms, 10);  // 10ms 주기 태스크 추가
    task_add(task_50ms, 50);  // 50ms 주기 태스크 추가
    task_add(task_100ms, 100);  // 100ms 주기 태스크 추가
    task_add(task_500ms, 500);  // 500ms 주기 태스크 추가
    task_add(task_1000ms, 1000);  // 1000ms 주기 태스크 추가

    while(1) {
        task_run();
    }
    
    return 0;
}

