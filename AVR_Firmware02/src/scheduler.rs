//
// 필요한 크레이트
//
// avr_device::atmega2560: ATmega2560 MCU의 SVD(Single Vendor Device) 파일과
// 관련 주변 장치에 접근하기 위해 사용됩니다.
// avr_device::interrupt: AVR 마이크로컨트롤러에서 인터럽트 관련 기능을 사용하기 위해 필요합니다.
//
use avr_device::atmega2560;
use avr_device::interrupt;

//
// RefCell, Mutex를 사용하기 위한 준비
//
// - RefCell: 런타임 시점에 불변/가변 빌림을 강제하기 위한 셀.
//   일반적인 Rust에서는 컴파일 시점에 빌림 규칙이 엄격히 적용되지만, 임베디드 환경에서
//   글로벌 데이터를 인터럽트에서도 안전하게 접근하기 위해 RefCell을 사용합니다.
//
// - Mutex: 여기서 사용되는 Mutex는 멀티쓰레드용이 아니라, 인터럽트 우선순위를 고려한
//   '임계영역(critical section)' 진입/이탈을 위해 사용되는 것입니다.
//   avr_device::interrupt::free(|cs| { ... })와 함께 사용되어, 인터럽트를 일시적으로
//   비활성화하여 원자성을 보장합니다.
//
use core::cell::RefCell;
use avr_device::interrupt::Mutex;

//
// Task 구조체
//   - task: 실제로 실행할 함수 포인터
//   - period: 주기(ms 단위). 0이면 등록 시점부터 계속 실행되도록 함
//   - next_run: 다음 실행 시점(시스템 시간 기준으로 언제 실행할지 결정)
//   - ready: 실행 준비 플래그. 스케줄러가 이 값을 확인하여 task를 실행할지 결정
//
#[derive(Clone, Copy)]
pub struct Task {
    pub task: fn(),
    pub period: u16,
    pub next_run: u16,
    pub ready: bool,
}

//
// Task를 생성하는 정적 함수.
// period가 0이면 등록 시점부터 ready = true가 되어 계속 실행됩니다.
//
impl Task {
    pub const fn new(task_fn: fn(), period: u16) -> Self {
        Self {
            task: task_fn,
            period,
            next_run: period,
            ready: period == 0,
        }
    }
}

//
//-------------------------------------------------------------------------
// 전역 데이터: 시스템 시간, 태스크 리스트
//-------------------------------------------------------------------------
//
// TASKS:
//   [Option<Task>; 10] 크기의 배열로 최대 10개의 태스크를 저장할 수 있습니다.
//   None이면 비어있는 슬롯, Some(Task)이면 태스크가 등록되어 있는 슬롯입니다.
//   RefCell로 감싸서 런타임 시점에 가변 빌림을 허용하고, Mutex로 다시 감싸서
//   인터럽트 중에도 안전하게 접근할 수 있도록 합니다.
//
// SYSTEM_TIME:
//   시스템 시간을 1ms 단위로 증가시키는 용도입니다. (u16 범위)
//   동일하게 RefCell + Mutex 조합으로 인터럽트와의 원자성을 보장합니다.
//
static TASKS: Mutex<RefCell<[Option<Task>; 10]>> = Mutex::new(RefCell::new([None; 10]));
static SYSTEM_TIME: Mutex<RefCell<u16>> = Mutex::new(RefCell::new(0));

//
//-------------------------------------------------------------------------
// 타이머 초기화 (CTC 모드, 1ms 인터럽트)
//-------------------------------------------------------------------------
//

/// 전역 인터럽트를 활성화하는 안전 래퍼 함수
///
/// Rust에서 전역 인터럽트 활성화는 unsafe 블록을 통해서 가능합니다.
/// 하지만 임베디드 환경에서 인터럽트 사용은 일반적인 동작이므로, 해당 동작을
/// enable_interrupts()라는 안전 함수로 래핑하여 노출합니다.
/// 호출자가 이 함수를 사용할 때는, 인터럽트가 활성화됨으로 인해 발생할 수 있는
/// 동시성 문제에 주의해야 합니다.
pub fn enable_interrupts() {
    unsafe {
        interrupt::enable();
    }
}

/// 8비트 타이머/카운터0를 CTC 모드로 설정하고, 약 1ms마다 인터럽트를 발생시키도록 초기화.
///
/// - Timer0을 사용 (atmega2560의 SVD 상에서 "TC0"으로 명명됨)
/// - 분주비 64 설정 (16MHz MCU 클럭에서 16,000,000/64=250,000Hz)
/// - OCR0A = 249로 설정 → 250,000/(249+1)=1,000 → 1kHz = 1ms 주기
/// - 출력비교일치 인터럽트(OCIE0A) 활성화
/// - 최종적으로 글로벌 인터럽트도 활성화
pub fn timer_init() {
    // ATmega2560 주변장치 구조체 접근
    let dp = atmega2560::Peripherals::take().unwrap();

    // Timer0 주변장치 핸들 가져오기
    let tc0 = dp.TC0;

    // TCCR0A: CTC 모드 설정
    // WGM0 필드는 2비트 길이이며, CTC 모드는 2(0b10)를 의미합니다.
    // 즉, WGM01=1, WGM00=0이 되는 조합.
    tc0.tccr0a.write(|w| w.wgm0().bits(2));

    // TCCR0B: 분주비 64 설정 (CS0=0b011)
    //
    // CS0(Clock Select)은 타이머 클럭 소스를 설정하기 위한 비트.
    // 0b011 → 클럭/64 분주.
    tc0.tccr0b.write(|w| w.cs0().bits(0b011));

    // OCR0A: 249 설정
    // 분주된 주파수가 250kHz이므로, 250kHz / (249+1) = 1,000Hz = 1ms 주기
    tc0.ocr0a.write(|w| unsafe { w.bits(249) });

    // TIMSK0: 출력 비교 A 매치 인터럽트 활성화
    // OCIE0A 비트를 1로 설정.
    tc0.timsk0.write(|w| w.ocie0a().set_bit());

    // 글로벌 인터럽트 활성화
    enable_interrupts();
}

//
//-------------------------------------------------------------------------
// 타이머 인터럽트 서비스 루틴 (ISR)
// 1ms마다 호출되어 시스템 시간 증가 및 태스크 스케줄링 점검
//-------------------------------------------------------------------------
//
// (주의) #[avr_device::interrupt(atmega2560)] 속성으로 인해, 컴파일러가
//        이 함수를 실제 인터럽트 벡터로 매핑합니다. 이름은 TIMER0_COMPA.
//        인터럽트 핸들러에서는 반드시 빠르게 처리하고, 오래 걸리는 작업은
//        하지 않는 것이 일반적인 권장사항입니다.
//
#[avr_device::interrupt(atmega2560)]
fn TIMER0_COMPA() {
    //
    // interrupt::free(|cs| { ... }) 내부에서는 인터럽트를 일시적으로 중단한 채
    // 임계 구역(critical section)에 진입할 수 있습니다. 하지만 이 함수 자체가
    // 이미 인터럽트 컨텍스트에서 실행 중이므로 추가로 인터럽트가 중첩되지 않도록
    // 보호할 수 있습니다. (중첩 인터럽트가 허용되는 다른 MCU라면 의미가 달라집니다)
    // AVR은 기본적으로 인터럽트 우선순위가 없고, 중첩 인터럽트를 사용하지 않음.
    //
    interrupt::free(|cs| {
        // 1) 시스템 시간 업데이트
        let mut system_time_ref = SYSTEM_TIME.borrow(cs).borrow_mut();
        *system_time_ref = system_time_ref.wrapping_add(1);
        let now = *system_time_ref; // 업데이트된 시간 보관

        // 2) 등록된 태스크 확인 및 ready 플래그 설정
        let mut tasks_ref = TASKS.borrow(cs).borrow_mut();
        for slot in tasks_ref.iter_mut() {
            if let Some(task) = slot.as_mut() {
                // period가 0보다 크고, 현재 시간이 next_run 이상이 되면
                // 실행 준비 상태(ready=true)로 만들어주고, next_run도 갱신
                if task.period > 0 && now >= task.next_run {
                    task.ready = true;
                    // 다음 실행 시점: now + period
                    task.next_run = now.wrapping_add(task.period);
                }
            }
        }
    });
}

//
//-------------------------------------------------------------------------
// 태스크 등록 함수
// period == 0 → 등록 시점부터 계속 실행(ready = true)
//-------------------------------------------------------------------------
//
// 주어진 task_fn을 TASKS 배열의 빈 슬롯(None)에 등록합니다.
//
pub fn task_add(task_fn: fn(), period: u16) {
    interrupt::free(|cs| {
        let mut tasks = TASKS.borrow(cs).borrow_mut();
        for slot in tasks.iter_mut() {
            if slot.is_none() {
                // 빈 슬롯을 찾아서 태스크 등록
                *slot = Some(Task::new(task_fn, period));
                return;
            }
        }
        // 만약 여기까지 온다면 빈 슬롯이 없습니다. (현재 코드에서는 무시)
        // 추후에는 "등록 실패" 처리 등을 할 수도 있습니다.
    });
}

//
//-------------------------------------------------------------------------
// 시스템 시간 읽기 (ms 단위)
//-------------------------------------------------------------------------
//
// 인터럽트로 인해 1ms마다 증가되는 SYSTEM_TIME 값을 리턴합니다.
// u16 범위를 벗어날 경우 wrapping_add로 인해 오버플로가 발생하나, 그대로 순환됩니다.
//
pub fn get_system_time() -> u16 {
    interrupt::free(|cs| {
        let time = *SYSTEM_TIME.borrow(cs).borrow();
        time
    })
}

//
//-------------------------------------------------------------------------
// 스케줄러 실행 (메인 루프 내에서 계속 호출)
//-------------------------------------------------------------------------
//
// 1) 현재 ready 상태인 태스크를 임시로 ready_tasks에 빼두고,
// 2) 임계 구역을 빠져나온 뒤(즉, 인터럽트 허용 상태에서) 실제로 실행.
//
// 이렇게 하는 이유:
//   - 긴 시간 동안 실행되는 함수(태스크)를 실행할 때, 임계구역(critical section) 내에서
//     실행하면 전체 시스템 응답성이 떨어집니다.
//   - 따라서, "어떤 태스크를 실행해야 하는지"만 결정하고 플래그를 세우는 작업은 짧게 수행한 뒤,
//     실제 함수 호출은 임계 구역 밖에서 이루어져, 인터럽트를 다시 활성화하여
//     시스템 전체 응답성을 높입니다.
//
pub fn scheduler_run() {
    // 준비된 태스크 함수를 임시로 담아둘 배열
    let mut ready_tasks: [Option<fn()>; 10] = [None; 10];
    let mut count = 0;

    // 1) 임계 구역 내에서, ready 상태인 태스크만 골라서 ready_tasks에 복사
    interrupt::free(|cs| {
        let mut tasks = TASKS.borrow(cs).borrow_mut();
        for slot in tasks.iter_mut() {
            if let Some(task) = slot.as_mut() {
                if task.ready {
                    ready_tasks[count] = Some(task.task);
                    count += 1;

                    // period > 0인 태스크만 다시 ready=false로 만든다.
                    // period == 0인 태스크는 계속 ready 상태로 두어서
                    // 매번 스케줄러가 실행할 때마다 실행됨.
                    if task.period > 0 {
                        task.ready = false;
                    }
                }
            }
        }
    });

    // 2) 임계 구역 밖에서 태스크 실제 실행
    //    이 시점에서는 인터럽트가 다시 허용되므로, 시스템 시간 등은 계속 갱신될 수 있음.
    for i in 0..count {
        if let Some(task_fn) = ready_tasks[i] {
            // 실제 태스크 함수 실행
            task_fn();
        }
    }
}

//
//-------------------------------------------------------------------------
// 블로킹 delay 함수 (ms 단위)
//-------------------------------------------------------------------------
//
// start_time = 현재 시스템 시간
// 현재 시스템 시간에서 start_time을 뺀 값이 ms보다 작으면 계속 반복.
//
// 이 함수는 블로킹 방식이므로, 일정 시간 동안 멈춰있습니다.
// 다만, 그동안에도 다른 태스크를 실행하고 싶다면 loop 내부에서 scheduler_run()을
// 호출하거나, 저전력 모드 진입 등의 처리가 가능합니다.
// 여기서는 단순한 예시를 위해 아무 동작 없이 기다리기만 합니다.
//
pub fn delay(ms: u16) {
    let start_time = get_system_time();
    while get_system_time().wrapping_sub(start_time) < ms {
        // 다른 태스크를 돌리려면 여기서 scheduler_run() 호출 가능
        // scheduler_run();
    }
}
