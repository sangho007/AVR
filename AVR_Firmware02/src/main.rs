#![feature(asm_experimental_arch)]
#![feature(abi_avr_interrupt)]

#![no_std]
#![no_main]

mod scheduler;
mod port;
// scheduler.rs 모듈 포함

use panic_halt as _; // 패닉 발생 시 멈춤 (혹은 다른 panic 처리 가능)
use scheduler::{timer_init, task_add, scheduler_run, delay};
use port::{PORTB};
use avr_device::entry;

/// 예시용 태스크 함수 1
fn user_task_1() {
    // 예) LED 토글 or UART 출력
    PORTB.toggle_pin(7);
}

/// 예시용 태스크 함수 2
fn user_task_2() {
    // 예) 센서 데이터 읽기
}

/// 메인 함수 (실제 엔트리 포인트)
#[entry]
fn main() -> ! {
    // 1) 타이머 초기화
    PORTB.set_pin_output(7);
    timer_init();

    // 2) 태스크 등록 (예: 100ms, 500ms 주기)
    task_add(user_task_1, 1000);

    // 3) 메인 루프
    loop {
        scheduler_run();
    }
}
