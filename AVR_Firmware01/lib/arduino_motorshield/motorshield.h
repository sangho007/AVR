#ifndef MOTORSHIELD_H
#define MOTORSHIELD_H

#include <avr/io.h>
#include "util.h"

// Motor Shield 핀 정의
#define PWM_A 5    // OC3C 
#define PWM_B 5    // OC1A 
#define BRAKE_A 6  // PH6
#define BRAKE_B 5  // PH5
#define DIR_A 6    // PB6
#define DIR_B 7    // PB7

// PWM 설정
#define PWM_MAX 255
#define PWM_MIN 0

// 함수 선언
void motorshield_init(void);
void pwm_init(void);

// 모터 A 제어 함수
void set_duty_cycle_a(uint8_t duty);
uint8_t get_duty_cycle_a(void);
void motor_a_forward(uint8_t speed);
void motor_a_backward(uint8_t speed);
void motor_a_stop(void);

// 모터 B 제어 함수
void set_duty_cycle_b(uint8_t duty);
uint8_t get_duty_cycle_b(void);
void motor_b_forward(uint8_t speed);
void motor_b_backward(uint8_t speed);
void motor_b_stop(void);

#endif
