// arduino.rs

// 사용되지 않는 코드에 대한 경고를 비활성화 (모든 핀을 항상 사용하는 것은 아니므로)
#![allow(dead_code)]

// port.rs 모듈을 가져옵니다.
use crate::port;

/// 핀 모드를 나타내는 열거형 (입력, 출력, 풀업 입력)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PinMode {
    Input,
    Output,
    InputPullup,
}

/// 핀의 상태를 나타내는 열거형 (High 또는 Low)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PinState {
    Low,
    High,
}

/// bool 값을 PinState로 변환합니다.
impl From<bool> for PinState {
    fn from(value: bool) -> Self {
        if value { PinState::High } else { PinState::Low }
    }
}

/// PinState 값을 bool로 변환합니다.
impl From<PinState> for bool {
    fn from(value: PinState) -> Self {
        match value {
            PinState::High => true,
            PinState::Low => false,
        }
    }
}

/// 아두이노 핀 번호를 실제 MCU 포트 및 핀 번호로 매핑하기 위한 내부 구조체
struct ArduinoPinMapping {
    /// 해당 핀이 속한 MCU 포트 (port.rs의 Port 구조체 참조)
    port: &'static port::Port,
    /// 해당 포트 내에서의 비트 번호 (0-7)
    pin_on_port: u8,
}

// --- Arduino Mega 2560 핀 정의 ---
// 이 핀 번호는 아두이노 IDE에서 사용하는 번호와 일치합니다.

// 디지털 핀 (PWM 가능 핀은 주석으로 표시)
pub const D0: u8 = 0;   // RX0 (PORTE, Bit 0)
pub const D1: u8 = 1;   // TX0 (PORTE, Bit 1)
pub const D2: u8 = 2;   // (PORTE, Bit 4) PWM ~
pub const D3: u8 = 3;   // (PORTE, Bit 5) PWM ~
pub const D4: u8 = 4;   // (PORTG, Bit 5) PWM ~
pub const D5: u8 = 5;   // (PORTE, Bit 3) PWM ~
pub const D6: u8 = 6;   // (PORTH, Bit 3) PWM ~
pub const D7: u8 = 7;   // (PORTH, Bit 4) PWM ~
pub const D8: u8 = 8;   // (PORTH, Bit 5) PWM ~
pub const D9: u8 = 9;   // (PORTH, Bit 6) PWM ~
pub const D10: u8 = 10; // (PORTB, Bit 4) PWM ~
pub const D11: u8 = 11; // (PORTB, Bit 5) PWM ~
pub const D12: u8 = 12; // (PORTB, Bit 6) PWM ~
pub const D13: u8 = 13; // (PORTB, Bit 7) PWM ~, LED_BUILTIN

pub const D14: u8 = 14; // TX3 (PORTJ, Bit 1)
pub const D15: u8 = 15; // RX3 (PORTJ, Bit 0)
pub const D16: u8 = 16; // TX2 (PORTH, Bit 1)
pub const D17: u8 = 17; // RX2 (PORTH, Bit 0)
pub const D18: u8 = 18; // TX1 (PORTD, Bit 3)
pub const D19: u8 = 19; // RX1 (PORTD, Bit 2)
pub const D20: u8 = 20; // SDA (PORTD, Bit 1)
pub const D21: u8 = 21; // SCL (PORTD, Bit 0)

pub const D22: u8 = 22; // (PORTA, Bit 0)
pub const D23: u8 = 23; // (PORTA, Bit 1)
pub const D24: u8 = 24; // (PORTA, Bit 2)
pub const D25: u8 = 25; // (PORTA, Bit 3)
pub const D26: u8 = 26; // (PORTA, Bit 4)
pub const D27: u8 = 27; // (PORTA, Bit 5)
pub const D28: u8 = 28; // (PORTA, Bit 6)
pub const D29: u8 = 29; // (PORTA, Bit 7)

pub const D30: u8 = 30; // (PORTC, Bit 7)
pub const D31: u8 = 31; // (PORTC, Bit 6)
pub const D32: u8 = 32; // (PORTC, Bit 5)
pub const D33: u8 = 33; // (PORTC, Bit 4)
pub const D34: u8 = 34; // (PORTC, Bit 3)
pub const D35: u8 = 35; // (PORTC, Bit 2)
pub const D36: u8 = 36; // (PORTC, Bit 1)
pub const D37: u8 = 37; // (PORTC, Bit 0)

pub const D38: u8 = 38; // (PORTD, Bit 7)
pub const D39: u8 = 39; // (PORTG, Bit 2) PWM ~ (주의: 아두이노 핀맵에서 PWM으로 표시 안될 수 있음)
pub const D40: u8 = 40; // (PORTG, Bit 1)
pub const D41: u8 = 41; // (PORTG, Bit 0)

pub const D42: u8 = 42; // (PORTL, Bit 7)
pub const D43: u8 = 43; // (PORTL, Bit 6)
pub const D44: u8 = 44; // (PORTL, Bit 5) PWM ~
pub const D45: u8 = 45; // (PORTL, Bit 4) PWM ~
pub const D46: u8 = 46; // (PORTL, Bit 3) PWM ~

pub const D47: u8 = 47; // (PORTL, Bit 2)
pub const D48: u8 = 48; // (PORTL, Bit 1)
pub const D49: u8 = 49; // (PORTL, Bit 0)

pub const D50: u8 = 50; // MISO (PORTB, Bit 3)
pub const D51: u8 = 51; // MOSI (PORTB, Bit 2)
pub const D52: u8 = 52; // SCK  (PORTB, Bit 1)
pub const D53: u8 = 53; // SS   (PORTB, Bit 0)

// 아날로그 핀 (디지털 핀으로도 사용 가능)
// 아두이노 핀 번호 A0-A15는 디지털 기능 사용 시 54-69로 취급합니다.
pub const A0: u8 = 54;  // (PORTF, Bit 0)
pub const A1: u8 = 55;  // (PORTF, Bit 1)
pub const A2: u8 = 56;  // (PORTF, Bit 2)
pub const A3: u8 = 57;  // (PORTF, Bit 3)
pub const A4: u8 = 58;  // (PORTF, Bit 4)
pub const A5: u8 = 59;  // (PORTF, Bit 5)
pub const A6: u8 = 60;  // (PORTF, Bit 6)
pub const A7: u8 = 61;  // (PORTF, Bit 7)
pub const A8: u8 = 62;  // (PORTK, Bit 0)
pub const A9: u8 = 63;  // (PORTK, Bit 1)
pub const A10: u8 = 64; // (PORTK, Bit 2)
pub const A11: u8 = 65; // (PORTK, Bit 3)
pub const A12: u8 = 66; // (PORTK, Bit 4)
pub const A13: u8 = 67; // (PORTK, Bit 5)
pub const A14: u8 = 68; // (PORTK, Bit 6)
pub const A15: u8 = 69; // (PORTK, Bit 7)

// LED_BUILTIN 별칭
pub const LED_BUILTIN: u8 = D13;

// 매핑할 총 핀 수 (D0 ~ D69 = 70개 핀)
const TOTAL_MAPPED_PINS: usize = 70;

// 아두이노 핀 번호 -> MCU 포트 및 핀 매핑 테이블
// Option을 사용하여 혹시 모를 누락된 매핑을 처리할 수 있지만, 여기서는 모든 핀을 매핑합니다.
static ARDUINO_PIN_MAP: [Option<ArduinoPinMapping>; TOTAL_MAPPED_PINS] = [
    // D0-D13
    Some(ArduinoPinMapping { port: &port::PORTE, pin_on_port: 0 }), // D0
    Some(ArduinoPinMapping { port: &port::PORTE, pin_on_port: 1 }), // D1
    Some(ArduinoPinMapping { port: &port::PORTE, pin_on_port: 4 }), // D2
    Some(ArduinoPinMapping { port: &port::PORTE, pin_on_port: 5 }), // D3
    Some(ArduinoPinMapping { port: &port::PORTG, pin_on_port: 5 }), // D4
    Some(ArduinoPinMapping { port: &port::PORTE, pin_on_port: 3 }), // D5
    Some(ArduinoPinMapping { port: &port::PORTH, pin_on_port: 3 }), // D6
    Some(ArduinoPinMapping { port: &port::PORTH, pin_on_port: 4 }), // D7
    Some(ArduinoPinMapping { port: &port::PORTH, pin_on_port: 5 }), // D8
    Some(ArduinoPinMapping { port: &port::PORTH, pin_on_port: 6 }), // D9
    Some(ArduinoPinMapping { port: &port::PORTB, pin_on_port: 4 }), // D10
    Some(ArduinoPinMapping { port: &port::PORTB, pin_on_port: 5 }), // D11
    Some(ArduinoPinMapping { port: &port::PORTB, pin_on_port: 6 }), // D12
    Some(ArduinoPinMapping { port: &port::PORTB, pin_on_port: 7 }), // D13 (LED_BUILTIN)
    // D14-D21
    Some(ArduinoPinMapping { port: &port::PORTJ, pin_on_port: 1 }), // D14
    Some(ArduinoPinMapping { port: &port::PORTJ, pin_on_port: 0 }), // D15
    Some(ArduinoPinMapping { port: &port::PORTH, pin_on_port: 1 }), // D16
    Some(ArduinoPinMapping { port: &port::PORTH, pin_on_port: 0 }), // D17
    Some(ArduinoPinMapping { port: &port::PORTD, pin_on_port: 3 }), // D18
    Some(ArduinoPinMapping { port: &port::PORTD, pin_on_port: 2 }), // D19
    Some(ArduinoPinMapping { port: &port::PORTD, pin_on_port: 1 }), // D20
    Some(ArduinoPinMapping { port: &port::PORTD, pin_on_port: 0 }), // D21
    // D22-D29
    Some(ArduinoPinMapping { port: &port::PORTA, pin_on_port: 0 }), // D22
    Some(ArduinoPinMapping { port: &port::PORTA, pin_on_port: 1 }), // D23
    Some(ArduinoPinMapping { port: &port::PORTA, pin_on_port: 2 }), // D24
    Some(ArduinoPinMapping { port: &port::PORTA, pin_on_port: 3 }), // D25
    Some(ArduinoPinMapping { port: &port::PORTA, pin_on_port: 4 }), // D26
    Some(ArduinoPinMapping { port: &port::PORTA, pin_on_port: 5 }), // D27
    Some(ArduinoPinMapping { port: &port::PORTA, pin_on_port: 6 }), // D28
    Some(ArduinoPinMapping { port: &port::PORTA, pin_on_port: 7 }), // D29
    // D30-D37
    Some(ArduinoPinMapping { port: &port::PORTC, pin_on_port: 7 }), // D30
    Some(ArduinoPinMapping { port: &port::PORTC, pin_on_port: 6 }), // D31
    Some(ArduinoPinMapping { port: &port::PORTC, pin_on_port: 5 }), // D32
    Some(ArduinoPinMapping { port: &port::PORTC, pin_on_port: 4 }), // D33
    Some(ArduinoPinMapping { port: &port::PORTC, pin_on_port: 3 }), // D34
    Some(ArduinoPinMapping { port: &port::PORTC, pin_on_port: 2 }), // D35
    Some(ArduinoPinMapping { port: &port::PORTC, pin_on_port: 1 }), // D36
    Some(ArduinoPinMapping { port: &port::PORTC, pin_on_port: 0 }), // D37
    // D38-D41
    Some(ArduinoPinMapping { port: &port::PORTD, pin_on_port: 7 }), // D38
    Some(ArduinoPinMapping { port: &port::PORTG, pin_on_port: 2 }), // D39
    Some(ArduinoPinMapping { port: &port::PORTG, pin_on_port: 1 }), // D40
    Some(ArduinoPinMapping { port: &port::PORTG, pin_on_port: 0 }), // D41
    // D42-D46
    Some(ArduinoPinMapping { port: &port::PORTL, pin_on_port: 7 }), // D42
    Some(ArduinoPinMapping { port: &port::PORTL, pin_on_port: 6 }), // D43
    Some(ArduinoPinMapping { port: &port::PORTL, pin_on_port: 5 }), // D44
    Some(ArduinoPinMapping { port: &port::PORTL, pin_on_port: 4 }), // D45
    Some(ArduinoPinMapping { port: &port::PORTL, pin_on_port: 3 }), // D46
    // D47-D49
    Some(ArduinoPinMapping { port: &port::PORTL, pin_on_port: 2 }), // D47
    Some(ArduinoPinMapping { port: &port::PORTL, pin_on_port: 1 }), // D48
    Some(ArduinoPinMapping { port: &port::PORTL, pin_on_port: 0 }), // D49
    // D50-D53
    Some(ArduinoPinMapping { port: &port::PORTB, pin_on_port: 3 }), // D50
    Some(ArduinoPinMapping { port: &port::PORTB, pin_on_port: 2 }), // D51
    Some(ArduinoPinMapping { port: &port::PORTB, pin_on_port: 1 }), // D52
    Some(ArduinoPinMapping { port: &port::PORTB, pin_on_port: 0 }), // D53

    // 아날로그 핀 A0-A15 (디지털 핀 54-69로 매핑)
    Some(ArduinoPinMapping { port: &port::PORTF, pin_on_port: 0 }), // A0 (D54)
    Some(ArduinoPinMapping { port: &port::PORTF, pin_on_port: 1 }), // A1 (D55)
    Some(ArduinoPinMapping { port: &port::PORTF, pin_on_port: 2 }), // A2 (D56)
    Some(ArduinoPinMapping { port: &port::PORTF, pin_on_port: 3 }), // A3 (D57)
    Some(ArduinoPinMapping { port: &port::PORTF, pin_on_port: 4 }), // A4 (D58)
    Some(ArduinoPinMapping { port: &port::PORTF, pin_on_port: 5 }), // A5 (D59)
    Some(ArduinoPinMapping { port: &port::PORTF, pin_on_port: 6 }), // A6 (D60)
    Some(ArduinoPinMapping { port: &port::PORTF, pin_on_port: 7 }), // A7 (D61)
    Some(ArduinoPinMapping { port: &port::PORTK, pin_on_port: 0 }), // A8 (D62)
    Some(ArduinoPinMapping { port: &port::PORTK, pin_on_port: 1 }), // A9 (D63)
    Some(ArduinoPinMapping { port: &port::PORTK, pin_on_port: 2 }), // A10 (D64)
    Some(ArduinoPinMapping { port: &port::PORTK, pin_on_port: 3 }), // A11 (D65)
    Some(ArduinoPinMapping { port: &port::PORTK, pin_on_port: 4 }), // A12 (D66)
    Some(ArduinoPinMapping { port: &port::PORTK, pin_on_port: 5 }), // A13 (D67)
    Some(ArduinoPinMapping { port: &port::PORTK, pin_on_port: 6 }), // A14 (D68)
    Some(ArduinoPinMapping { port: &port::PORTK, pin_on_port: 7 }), // A15 (D69)
];

/// 주어진 아두이노 핀 번호에 대한 MCU 포트 및 핀 번호 매핑 정보를 가져옵니다.
/// 유효하지 않은 핀 번호인 경우 패닉합니다.
#[inline(always)]
fn get_mapping(arduino_pin_number: u8) -> &'static ArduinoPinMapping {
    let index = arduino_pin_number as usize;
    if index < TOTAL_MAPPED_PINS {
        // ARDUINO_PIN_MAP[index]가 Some이어야 하며, None이면 프로그래밍 오류
        match &ARDUINO_PIN_MAP[index] {
            Some(mapping) => mapping,
            None => panic!(
                "Arduino pin D{} is defined as a constant but not mapped in ARDUINO_PIN_MAP.",
                arduino_pin_number
            ),
        }
    } else {
        panic!("Invalid Arduino pin number: D{}", arduino_pin_number);
    }
}

/// 아두이노 핀의 모드를 설정합니다 (Input, Output, InputPullup).
///
/// # Arguments
/// * `pin_number`: 아두이노 핀 번호 (예: `D13`, `A0`). `A0`~`A15`는 `54`~`69`로 사용합니다.
/// * `mode`: 원하는 `PinMode`.
///
/// # Panics
/// `pin_number`가 유효하지 않으면 패닉합니다.
pub fn pin_mode(pin_number: u8, mode: PinMode) {
    let mapping = get_mapping(pin_number);
    match mode {
        PinMode::Output => {
            mapping.port.set_pin_output(mapping.pin_on_port);
        }
        PinMode::Input => {
            mapping.port.set_pin_input(mapping.pin_on_port);
            // 일반 입력 모드에서는 풀업 저항을 비활성화합니다 (PORTx 비트를 0으로 설정).
            mapping.port.set_pin_low(mapping.pin_on_port);
        }
        PinMode::InputPullup => {
            mapping.port.set_pin_input(mapping.pin_on_port);
            // 입력 모드로 설정 후, 내부 풀업 저항을 활성화합니다 (PORTx 비트를 1로 설정).
            mapping.port.set_pin_high(mapping.pin_on_port);
        }
    }
}

/// 아두이노 핀에 디지털 값(High 또는 Low)을 씁니다.
/// 해당 핀은 `pin_mode`를 사용하여 `Output`으로 설정되어 있어야 합니다.
///
/// # Arguments
/// * `pin_number`: 아두이노 핀 번호.
/// * `value`: 쓸 `PinState` (High 또는 Low).
///
/// # Panics
/// `pin_number`가 유효하지 않으면 패닉합니다.
pub fn digital_write(pin_number: u8, value: PinState) {
    let mapping = get_mapping(pin_number);
    match value {
        PinState::High => mapping.port.set_pin_high(mapping.pin_on_port),
        PinState::Low => mapping.port.set_pin_low(mapping.pin_on_port),
    }
}

/// 아두이노 핀에서 디지털 값(High 또는 Low)을 읽습니다.
/// 해당 핀은 `Input` 또는 `InputPullup`으로 설정되어 있는 것이 이상적입니다.
///
/// # Arguments
/// * `pin_number`: 아두이노 핀 번호.
///
/// # Returns
/// 핀의 `PinState` (High 또는 Low).
///
/// # Panics
/// `pin_number`가 유효하지 않으면 패닉합니다.
pub fn digital_read(pin_number: u8) -> PinState {
    let mapping = get_mapping(pin_number);
    if mapping.port.read_pin(mapping.pin_on_port) {
        PinState::High
    } else {
        PinState::Low
    }
}

/// 아두이노 핀의 상태를 토글합니다. (High -> Low, Low -> High)
/// 해당 핀은 `pin_mode`를 사용하여 `Output`으로 설정되어 있어야 합니다.
///
/// # Arguments
/// * `pin_number`: 아두이노 핀 번호.
///
/// # Panics
/// `pin_number`가 유효하지 않으면 패닉합니다.
pub fn digital_toggle(pin_number: u8) {
    let mapping = get_mapping(pin_number);
    mapping.port.toggle_pin(mapping.pin_on_port);
}

