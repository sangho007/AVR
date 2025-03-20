//
// 필요한 크레이트
//
use avr_device::atmega2560;
use avr_device::interrupt;

//
// RefCell, Mutex를 사용하기 위한 준비
//
use core::cell::RefCell;
use avr_device::interrupt::Mutex;

//
// Task 구조체
//
#[derive(Clone, Copy)]
pub struct Task {
    pub task: fn(),
    pub period: u16,
    pub next_run: u16,
    pub ready: bool,
}

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
// 전역 데이터
//
static TASKS: Mutex<RefCell<[Option<Task>; 10]>> = Mutex::new(RefCell::new([None; 10]));
static SYSTEM_TIME: Mutex<RefCell<u16>> = Mutex::new(RefCell::new(0));

/// 전역 인터럽트 활성화 함수
pub fn enable_interrupts() {
    unsafe {
        interrupt::enable();
    }
}

/// 타이머 초기화 (CTC 모드, 약 1ms 인터럽트)
///
/// - **변경점**: 이제 `timer_init()`이 직접 `Peripherals::take()`를 쓰지 않고
///   호출 시점에 `tc0: atmega2560::TC0`을 인자로 받습니다.
pub fn timer_init(tc0: atmega2560::TC0) {
    // TCCR0A: CTC 모드 설정 (WGM0 = 2 → WGM01=1, WGM00=0)
    tc0.tccr0a.write(|w| w.wgm0().bits(2));

    // TCCR0B: 분주비 64 설정 (CS0 = 0b011)
    tc0.tccr0b.write(|w| w.cs0().bits(0b011));

    // OCR0A: 249 설정 → 1ms 주기
    tc0.ocr0a.write(|w| unsafe { w.bits(249) });

    // TIMSK0: 출력 비교 A 매치 인터럽트 활성화 (OCIE0A=1)
    tc0.timsk0.write(|w| w.ocie0a().set_bit());

    // 글로벌 인터럽트 활성화
    enable_interrupts();
}

#[avr_device::interrupt(atmega2560)]
fn TIMER0_COMPA() {
    interrupt::free(|cs| {
        // 1) 시스템 시간 1ms 증가
        let mut system_time_ref = SYSTEM_TIME.borrow(cs).borrow_mut();
        *system_time_ref = system_time_ref.wrapping_add(1);
        let now = *system_time_ref;

        // 2) 등록된 태스크 확인 후 ready 플래그 세팅
        let mut tasks_ref = TASKS.borrow(cs).borrow_mut();
        for slot in tasks_ref.iter_mut() {
            if let Some(task) = slot.as_mut() {
                // period > 0 && now >= next_run → ready = true
                if task.period > 0 && now >= task.next_run {
                    task.ready = true;
                    task.next_run = now.wrapping_add(task.period);
                }
            }
        }
    });
}

pub fn task_add(task_fn: fn(), period: u16) {
    interrupt::free(|cs| {
        let mut tasks = TASKS.borrow(cs).borrow_mut();
        for slot in tasks.iter_mut() {
            if slot.is_none() {
                *slot = Some(Task::new(task_fn, period));
                return;
            }
        }
        // 빈 슬롯 없으면 등록 실패 처리 (여기서는 무시)
    });
}

pub fn get_system_time() -> u16 {
    interrupt::free(|cs| {
        let time = *SYSTEM_TIME.borrow(cs).borrow();
        time
    })
}

/// 스케줄러 실행 (ready 태스크를 찾아서 실제로 실행)
pub fn scheduler_run() {
    let mut ready_tasks: [Option<fn()>; 10] = [None; 10];
    let mut count = 0;

    // 1) 임계구역 내에서 ready 태스크만 복사해둠
    interrupt::free(|cs| {
        let mut tasks = TASKS.borrow(cs).borrow_mut();
        for slot in tasks.iter_mut() {
            if let Some(task) = slot.as_mut() {
                if task.ready {
                    ready_tasks[count] = Some(task.task);
                    count += 1;

                    // period > 0이면 한 번 실행 후 ready false
                    // period=0이면 매번 실행(ready 유지)
                    if task.period > 0 {
                        task.ready = false;
                    }
                }
            }
        }
    });

    // 2) 임계구역 밖에서 태스크 실제 실행
    for i in 0..count {
        if let Some(task_fn) = ready_tasks[i] {
            task_fn();
        }
    }
}

/// 블로킹 delay (ms 단위)
pub fn delay(ms: u16) {
    let start_time = get_system_time();
    while get_system_time().wrapping_sub(start_time) < ms {
        // 필요시 여기서 scheduler_run() 실행 가능
        // scheduler_run();
    }
}
