#![feature(asm_experimental_arch)]
#![feature(abi_avr_interrupt)]

#![no_std]
#![no_main]

mod scheduler;
mod port;
mod serial;
mod arduino;

use panic_halt as _;
use avr_device::entry;
use avr_device::atmega2560;
use port::*;
use arduino::*;
use arduino::PinMode::*;

/// 예시용 태스크 함수 1
fn user_task_1() {
    // 예) LED 토글
    // PORTB.toggle_pin(7);
    digital_toggle(LED_BUILTIN);
}

/// 예시용 태스크 함수 2
fn user_task_2() {
    // 예) UART 출력
    serial::write_str("10ms_Task!\r\n");
}

fn user_task_3() {
    // 예) UART 출력
    serial::write_str("2ms_Task!\r\n");
}
fn user_task_4() {
    serial::serial_echo();
}




/// 메인 함수 (실제 엔트리 포인트)
#[entry]
fn main() -> ! {
    let dp = atmega2560::Peripherals::take().unwrap();

    // 1) 타이머 초기화(Timer0)
    scheduler::timer_init(dp.TC0);
    // 2) serial 초기화
    serial::serial_init(dp.USART0, 115200);

    // 2) 태스크 등록 (예: 100ms, 500ms 주기)
    scheduler::task_add(user_task_1, 1000);
    scheduler::task_add(user_task_2, 10);
    scheduler::task_add(user_task_3, 2);
    scheduler::task_add(user_task_4, 0);

    // PORTB.set_pin_output(7);
    pin_mode(LED_BUILTIN, Output);

    // 3) 메인 루프
    loop {
        scheduler::scheduler_run();
    }
}
