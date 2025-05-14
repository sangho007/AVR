use core::ptr::{read_volatile, write_volatile};

/// Port 구조체:
/// - `pin`  : PINx 레지스터 주소
/// - `ddr`  : DDRx 레지스터 주소
/// - `port` : PORTx 레지스터 주소
///
/// 해당 구조체의 메서드를 통해 핀 방향 설정/출력/입력 읽기 등의 작업을 수행합니다.
pub struct Port {
    pin: *mut u8,
    ddr: *mut u8,
    port: *mut u8,
}

unsafe impl Sync for Port {}
unsafe impl Send for Port {}

impl Port {
    /// 지정한 비트(pin_number)를 출력으로 설정합니다. (DDRx |= (1 << pin_number))
    pub fn set_pin_output(&self, pin_number: u8) {
        unsafe {
            let val = read_volatile(self.ddr);
            write_volatile(self.ddr, val | (1 << pin_number));
        }
    }

    /// 지정한 비트(pin_number)를 입력으로 설정합니다. (DDRx &= ~(1 << pin_number))
    pub fn set_pin_input(&self, pin_number: u8) {
        unsafe {
            let val = read_volatile(self.ddr);
            write_volatile(self.ddr, val & !(1 << pin_number));
        }
    }

    /// 지정한 비트(pin_number)를 High(1)로 설정합니다. (PORTx |= (1 << pin_number))
    pub fn set_pin_high(&self, pin_number: u8) {
        unsafe {
            let val = read_volatile(self.port);
            write_volatile(self.port, val | (1 << pin_number));
        }
    }

    /// 지정한 비트(pin_number)를 Low(0)로 설정합니다. (PORTx &= ~(1 << pin_number))
    pub fn set_pin_low(&self, pin_number: u8) {
        unsafe {
            let val = read_volatile(self.port);
            write_volatile(self.port, val & !(1 << pin_number));
        }
    }

    /// 지정한 비트(pin_number)를 토글합니다. (PORTx ^= (1 << pin_number))
    pub fn toggle_pin(&self, pin_number: u8) {
        unsafe {
            let val = read_volatile(self.port);
            write_volatile(self.port, val ^ (1 << pin_number));
        }
    }

    /// 지정한 비트(pin_number)의 입력값(PINx)을 읽어 반환합니다. (true = High, false = Low)
    pub fn read_pin(&self, pin_number: u8) -> bool {
        unsafe {
            let val = read_volatile(self.pin);
            (val & (1 << pin_number)) != 0
        }
    }
}

//------------------------------------------------------------------------------
// ATmega2560의 각 포트별 레지스터 주소 (데이터시트 참고)
// PINx / DDRx / PORTx 순서로 매핑
//
// Note: 주소는 I/O 메모리 맵(0x20 ~) 기준입니다. 실제로는
//       gcc-avr에서 <avr/io.h> 매크로가 i/o space로 매핑해줍니다.
//       아래는 대표적인 매핑 예시이며, 반드시 공식 데이터시트로 재확인하세요.
//------------------------------------------------------------------------------
pub const PORTA: Port = Port {
    pin: 0x20 as *mut u8,  // PINA
    ddr: 0x21 as *mut u8,  // DDRA
    port: 0x22 as *mut u8, // PORTA
};

pub const PORTB: Port = Port {
    pin: 0x23 as *mut u8,  // PINB
    ddr: 0x24 as *mut u8,  // DDRB
    port: 0x25 as *mut u8, // PORTB
};

pub const PORTC: Port = Port {
    pin: 0x26 as *mut u8,  // PINC
    ddr: 0x27 as *mut u8,  // DDRC
    port: 0x28 as *mut u8, // PORTC
};

pub const PORTD: Port = Port {
    pin: 0x29 as *mut u8,  // PIND
    ddr: 0x2A as *mut u8,  // DDRD
    port: 0x2B as *mut u8, // PORTD
};

pub const PORTE: Port = Port {
    pin: 0x2C as *mut u8,  // PINE
    ddr: 0x2D as *mut u8,  // DDRE
    port: 0x2E as *mut u8, // PORTE
};

pub const PORTF: Port = Port {
    pin: 0x2F as *mut u8,  // PINF
    ddr: 0x30 as *mut u8,  // DDRF
    port: 0x31 as *mut u8, // PORTF
};

pub const PORTG: Port = Port {
    pin: 0x32 as *mut u8,  // PING
    ddr: 0x33 as *mut u8,  // DDRG
    port: 0x34 as *mut u8, // PORTG
};

// ATmega2560에서 H~L 포트는 IO 공간 주소가 0x100 이상으로 표시됩니다.
pub const PORTH: Port = Port {
    pin: 0x100 as *mut u8,  // PINH
    ddr: 0x101 as *mut u8,  // DDRH
    port: 0x102 as *mut u8, // PORTH
};

pub const PORTJ: Port = Port {
    pin: 0x103 as *mut u8,  // PINJ
    ddr: 0x104 as *mut u8,  // DDRJ
    port: 0x105 as *mut u8, // PORTJ
};

pub const PORTK: Port = Port {
    pin: 0x106 as *mut u8,  // PINK
    ddr: 0x107 as *mut u8,  // DDRK
    port: 0x108 as *mut u8, // PORTK
};

pub const PORTL: Port = Port {
    pin: 0x109 as *mut u8,  // PINL
    ddr: 0x10A as *mut u8,  // DDRL
    port: 0x10B as *mut u8, // PORTL
};
