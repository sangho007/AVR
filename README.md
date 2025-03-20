# AVR_Firmware02

이 프로젝트는 **Arduino Mega 2560(ATmega2560 MCU)** 보드에서 동작하는 간단한 **소프트웨어 스케줄러** 예제 코드입니다. Rust로 작성되었으며, `no_std` 환경에서 동작합니다.

## 개요

- **메인 MCU**: ATmega2560 (Arduino Mega 2560)
- **언어/환경**: Rust + `no_std`
- **기능**:
  - Timer0 인터럽트를 이용해 시스템 tick(약 1ms) 관리
  - 간단한 스케줄링 기법 적용
  - 일정 주기(예: 2ms, 10ms, 100ms 등)로 태스크 실행
  - 아두이노메가 LED 토글, UART 송수신(Interrupt 기반)을 예제로 포함

## 폴더 구조

```bash
AVR_Firmware02
├── .cargo
    ├── config.toml
├── Cargo.lock
├── Cargo.toml
├── avr-atmega2560.json
└── src
    ├── main.rs        # 엔트리 포인트, setup 및 메인 루프
    ├── scheduler.rs   # 스케줄러 로직 (타이머 인터럽트, task 등록/실행)
    ├── port.rs        # Port 구조체 (핀 입출력)
    └── serial.rs      # UART 초기화, 송신/수신 핸들러
```

### 주요 파일 설명

- **`main.rs`**
  - 실제 `#[entry]` 함수를 통해 MCU가 부팅된 후 가장 먼저 실행
  - `timer_init()`, `serial::init()` 등 주변장치 초기화
  - 예제 태스크(`user_task_1`, `user_task_2`, `user_task_3`, `user_task_4`)를 등록하고 `scheduler::scheduler_run()`을 계속 호출
- **`scheduler.rs`**
  - `timer_init()`으로 Timer0을 **CTC 모드(1ms 주기)**로 설정
  - `TIMER0_COMPA` 인터럽트 핸들러에서 시스템 시간 증가 및 태스크 준비(ready) 상태 업데이트
  - `task_add()`로 태스크를 주기별로 등록
  - `scheduler_run()`에서 ready 상태인 태스크들을 실제로 실행
- **`port.rs`**
  - `Port` 구조체를 통해 핀 입출력, 토글 등 간단한 GPIO 제어
  - 예: `port::PORTB.set_pin_output(7);`로 B포트 7번 핀을 출력으로 설정
- **`serial.rs`**
  - UART(USART0) 초기화와 송/수신(인터럽트 기반) 로직
  - 송신 링버퍼를 이용하여 논블로킹 방식 구현
  - `serial_echo()` 함수 예제: RX 수신 데이터를 그대로 TX로 에코

## 빌드 및 업로드

Arduino Mega 2560(ATmega2560)에서 Rust 펌웨어를 빌드하기 위해서는 몇 가지 준비가 필요합니다.

### 1. Rust (nightly) 및 avr-device 툴체인 설치
- [`rustup`](https://rustup.rs/)을 사용하여 **nightly** 컴파일러를 설치합니다.  
- AVR 크로스 컴파일을 위해 [`avr-gcc`, `avr-libc` 등](https://www.microchip.com/en-us/development-tool/avr-and-sam-downloads-archive)을 설치해야 합니다(운영체제 별로 설치 방법 상이).
- (선택) 아래와 같이 AVR용 사용자 정의 타겟 JSON(`avr-atmega2560.json`)을 사용하거나, [`avr-unknown-gnu-atmega2560`](https://github.com/avr-rust/rust/tree/avr-support)를 사용합니다.

### 2. 레포지토리 클론
```bash
git clone https://github.com/your_username/AVR_Firmware02.git
cd AVR_Firmware02
```

### 3. 빌드 & 업로드
```bash
cargo run
```
- 빌드가 끝나면, `target/avr-atmega2560/debug/AVR_Firmware02.elf` 파일이 생성됩니다.
- ravrdude를 사용하며 업로드 됩니다.

## 동작 확인

- **LED 토글 예제**  
  `user_task_1`은 `PORTB`의 7번 핀을 토글합니다. 보드에 연결된 내장 LED가  점멸하는지 확인하세요.

- **시리얼**  
  `user_task_2`, `user_task_3` 등에서는 주기적으로 `"10ms_Task!\r\n"`, `"2ms_Task!\r\n"` 등의 문자열을 전송합니다.  
  또한 `user_task_4`(=`serial_echo`) 함수가 RX 데이터를 그대로 TX로 에코합니다. 시리얼 모니터(115200bps)에 입력하면, 동일한 문자가 돌아오는지 확인 가능합니다.

## 주의사항
  
- UART는 **Double Speed(U2X0)** 모드 사용, BAUD 계산식은 [코드](./src/serial.rs) 내 확인
- 각종 레지스터 주소(특히 I/O 공간 0x100 이상)나 인터럽트 벡터는 ATmega2560 기준입니다. 타 AVR MCU에서는 맞지 않을 수 있습니다.

## 참고

- [Rust Embedded Book](https://docs.rust-embedded.org/book/)
- [avr-device crate (atmega2560)](https://crates.io/crates/avr-device)
- [avr-rust 프로젝트 문서](https://github.com/avr-rust/rust)  

-----

이상으로 간단한 **Arduino Mega 2560 + Rust no_std + 간단 소프트웨어 스케줄러** 예제에 대한 소개를 마칩니다.  
질문이나 개선사항이 있다면 **이슈** 통해 알려주세요! 
