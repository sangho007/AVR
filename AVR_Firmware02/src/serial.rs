#![allow(dead_code)]

use avr_device::atmega2560;
use avr_device::interrupt::{self, Mutex};
use core::cell::RefCell;
use crate::serial;

//
// CPU 클록 주파수 (16MHz 기준 예시)
//
const CPU_FREQUENCY: u32 = 16_000_000;

//
// 전역으로 USART0 핸들을 저장할 Mutex+RefCell
//
static USART0: Mutex<RefCell<Option<atmega2560::USART0>>> =
    Mutex::new(RefCell::new(None));

//
// 송신 버퍼 설정(간단 링버퍼)
//
const TX_BUFFER_SIZE: usize = 128; // 필요에 맞게 조정
static TX_BUFFER: Mutex<RefCell<[u8; TX_BUFFER_SIZE]>> =
    Mutex::new(RefCell::new([0; TX_BUFFER_SIZE]));
static TX_HEAD: Mutex<RefCell<usize>> = Mutex::new(RefCell::new(0));
static TX_TAIL: Mutex<RefCell<usize>> = Mutex::new(RefCell::new(0));

/// 시리얼 초기화
///
/// - `usart0`: 메인에서 `dp.USART0`을 받았던 것을 그대로 인자로 전달
/// - `baud_rate`: 예) 9600, 19200, 115200 등
pub fn init(usart0: atmega2560::USART0, baud_rate: u32) {
    // 1) Double Speed 모드 활성화 (U2X0=1)
    usart0.ucsr0a.modify(|_, w| w.u2x0().set_bit());

    // 2) UBRR 계산: (F_CPU / (8 * baud)) - 1
    let ubrr = (CPU_FREQUENCY / (8 * baud_rate) - 1) as u16;

    // 3) UBRR0 설정
    usart0.ubrr0.write(|w| unsafe { w.bits(ubrr) });

    // 4) UCSR0C: 비동기, 패리티 없음, 1 스톱비트, 데이터 8비트
    usart0.ucsr0c.write(|w| {
        w.umsel0().usart_async() // 비동기
            .upm0().disabled()      // 패리티 없음
            .usbs0().bit(false)     // 1 스톱비트
            .ucsz0().chr8()         // 8비트
    });

    // 5) UCSR0B: RX/TX Enable
    usart0.ucsr0b.write(|w| {
        w.rxen0().set_bit()
            .txen0().set_bit()
            .udrie0().clear_bit() // 일단 비활성, 필요 시 인터럽트 enable
    });

    // 전역 USART0에 저장
    interrupt::free(|cs| {
        *USART0.borrow(cs).borrow_mut() = Some(usart0);
    });
}

/// **비동기** 송신: 문자열을 링버퍼에 쌓고, UDRE0 인터럽트를 활성화하여
/// 하드웨어가 준비될 때마다 1바이트씩 전송.
pub fn write_str(s: &str) {
    interrupt::free(|cs| {
        // USART0 핸들 가져오기
        let usart0_opt = USART0.borrow(cs).borrow();
        let usart0 = match *usart0_opt {
            Some(ref u) => u,
            None => return, // 초기화 안 됐다면 무시
        };

        // 링버퍼에 데이터 추가
        let mut tx_buffer = TX_BUFFER.borrow(cs).borrow_mut();
        let mut head = TX_HEAD.borrow(cs).borrow_mut();
        let tail = TX_TAIL.borrow(cs).borrow();

        for &b in s.as_bytes() {
            // 다음 head 위치
            let next_head = (*head + 1) % TX_BUFFER_SIZE;

            // 버퍼가 가득 차 있으면 (next_head == tail) -> 여기서는 '대기' 대신 '무시' 처리
            if next_head == *tail {
                // 버퍼 오버플로 시 추가 문자는 버림
                break;
            }

            // 버퍼에 데이터 넣기
            tx_buffer[*head] = b;
            *head = next_head;
        }

        // UDRE0 인터럽트 활성화 (송신 시작)
        // (이미 인터럽트가 활성화되어 있더라도 문제없지만, 확실히 하기 위해 다시 set)
        usart0.ucsr0b.modify(|_, w| w.udrie0().set_bit());
    });
}

/// 1바이트 **수신** (블로킹)
pub fn read_byte() -> u8 {
    let mut received = 0;
    interrupt::free(|cs| {
        if let Some(ref usart0) = *USART0.borrow(cs).borrow() {
            // RXC0(수신 완료) 플래그 대기
            while usart0.ucsr0a.read().rxc0().bit_is_clear() {}

            // 수신 데이터 읽기
            received = usart0.udr0.read().bits();
        }
    });
    received
}

/// 1바이트 **수신** (논블로킹) - 데이터 있으면 Some, 없으면 None
pub fn read_nonblocking() -> Option<u8> {
    let mut result = None;
    interrupt::free(|cs| {
        if let Some(ref usart0) = *USART0.borrow(cs).borrow() {
            if usart0.ucsr0a.read().rxc0().bit_is_set() {
                // 데이터 있으면 읽기
                result = Some(usart0.udr0.read().bits());
            }
        }
    });
    result
}

pub fn serial_echo() {
    if let Some(byte) = read_nonblocking() {
        let buf = [byte];  // 길이 1짜리 버퍼
        // 만약 들어오는 바이트가 ASCII(또는 UTF-8) 범위라고 가정한다면:
        if let Ok(s) = core::str::from_utf8(&buf) {
            serial::write_str(s);
        }
    }
}

//
// UDRE0 인터럽트 핸들러
//  - 하드웨어가 "UDR0 레지스터 비었다"고 알려주면, 링버퍼에서 다음 바이트를 꺼내 전송.
//
#[avr_device::interrupt(atmega2560)]
fn USART0_UDRE() {
    interrupt::free(|cs| {
        let usart0_opt = USART0.borrow(cs).borrow();
        let usart0 = match *usart0_opt {
            Some(ref u) => u,
            None => return,
        };

        let mut tx_buffer = TX_BUFFER.borrow(cs).borrow_mut();
        let mut head = TX_HEAD.borrow(cs).borrow();
        let mut tail = TX_TAIL.borrow(cs).borrow_mut();

        // 버퍼 안에 보낼 데이터가 있으면 1바이트 전송
        if *tail != *head {
            // 다음 바이트 전송
            let data = tx_buffer[*tail];
            usart0.udr0.write(|w| unsafe { w.bits(data) });

            // tail을 한 칸 이동
            *tail = (*tail + 1) % TX_BUFFER_SIZE;
        } else {
            // 더 이상 보낼 데이터가 없으면 UDRE 인터럽트 비활성화
            usart0.ucsr0b.modify(|_, w| w.udrie0().clear_bit());
        }
    });
}
