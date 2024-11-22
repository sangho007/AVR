#include "motorshield.h"
#include <util/delay.h>

// 전역 변수로 현재 속도 저장
static uint8_t motor_a_speed = 0;
static uint8_t motor_b_speed = 0;

void motorshield_init(void) {
    // 방향 핀 설정 (PORTB)
    DDRB |= (1 << DIR_A) | (1 << DIR_B);      // DIR_A(PB6), DIR_B(PB7) 출력으로 설정
    
    // 브레이크 핀 설정 (PORTH)
    DDRH |= (1 << BRAKE_A) | (1 << BRAKE_B);  // BRAKE_A(PH6), BRAKE_B(PH5) 출력으로 설정
    
    // PWM 핀 설정
    DDRE |= (1 << PWM_A);  // PWM_A(PE5, OC3C) 출력으로 설정
    DDRB |= (1 << PWM_B);  // PWM_B(PB5, OC1A) 출력으로 설정
    
    // PWM 초기화 호출
    pwm_init();
}

void pwm_init(void) {
    // Timer3 설정 (모터 A용 - OC3C)
    TCCR3A |= (1 << COM3C1) | (1 << WGM31);   // 고속 PWM 모드, 비반전
    TCCR3B |= (1 << WGM33) | (1 << WGM32) | (1 << CS31);  // 8분주
    ICR3 = PWM_MAX;  // TOP 값 설정
    OCR3C = 0;       // 초기 듀티비 0
    
    // Timer1 설정 (모터 B용 - OC1A)
    TCCR1A |= (1 << COM1A1) | (1 << WGM11);   // 고속 PWM 모드, 비반전
    TCCR1B |= (1 << WGM13) | (1 << WGM12) | (1 << CS11);  // 8분주
    ICR1 = PWM_MAX;  // TOP 값 설정
    OCR1A = 0;       // 초기 듀티비 0
}

void set_duty_cycle_a(uint8_t duty) {
    if (duty > PWM_MAX) {
        duty = PWM_MAX;
    }
    OCR3C = duty;  // 모터 A의 듀티비 설정 (Timer3)
    motor_a_speed = duty;
}

void set_duty_cycle_b(uint8_t duty) {
    if (duty > PWM_MAX) {
        duty = PWM_MAX;
    }
    OCR1A = duty;  // 모터 B의 듀티비 설정 (Timer1)
    motor_b_speed = duty;
}

uint8_t get_duty_cycle_a(void) {
    return motor_a_speed;
}

uint8_t get_duty_cycle_b(void) {
    return motor_b_speed;
}

void motor_a_forward(uint8_t speed) {
    PORTH &= ~(1 << BRAKE_A);  // 브레이크 A 해제
    PORTB |= (1 << DIR_A);     // 방향 A 설정
    set_duty_cycle_a(speed);
}

void motor_a_backward(uint8_t speed) {
    PORTH &= ~(1 << BRAKE_A);  // 브레이크 A 해제
    PORTB &= ~(1 << DIR_A);    // 방향 A 설정
    set_duty_cycle_a(speed);
}

void motor_a_stop(void) {
    PORTH |= (1 << BRAKE_A);   // 브레이크 A 설정
    set_duty_cycle_a(0);
}

void motor_b_forward(uint8_t speed) {
    PORTH &= ~(1 << BRAKE_B);  // 브레이크 B 해제
    PORTB |= (1 << DIR_B);     // 방향 B 설정
    set_duty_cycle_b(speed);
}

void motor_b_backward(uint8_t speed) {
    PORTH &= ~(1 << BRAKE_B);  // 브레이크 B 해제
    PORTB &= ~(1 << DIR_B);    // 방향 B 설정
    set_duty_cycle_b(speed);
}

void motor_b_stop(void) {
    PORTH |= (1 << BRAKE_B);   // 브레이크 B 설정
    set_duty_cycle_b(0);
}
