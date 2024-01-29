use crate::emulator::initialize_emulator;

use super::*;

#[test]
fn increments_base_speed_by_number_of_m_cycles() {
    let mut emulator = initialize_emulator();
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.timers.m_cycles_clock, 1);
}

#[test]
fn resets_base_speed_after_four_m_cycles() {
    let mut emulator = initialize_emulator();
    emulator.cpu.clock.instruction_clock_cycles = 4;
    emulator.timers.m_cycles_clock = 3;
    step(&mut emulator);
    assert_eq!(emulator.timers.m_cycles_clock, 0);
}

#[test]
fn increments_divider_clock_after_four_m_cycles() {
    let mut emulator = initialize_emulator();
    emulator.cpu.clock.instruction_clock_cycles = 4;
    emulator.timers.m_cycles_clock = 3;
    step(&mut emulator);
    assert_eq!(emulator.timers.divider_clock, 1);
}

#[test]
fn increments_divider_register_after_sixteen_divider_clock_increments() {
    let mut emulator = initialize_emulator();
    emulator.cpu.clock.instruction_clock_cycles = 4;
    emulator.timers.m_cycles_clock = 3;
    emulator.timers.divider_clock = 15;
    step(&mut emulator);
    assert_eq!(emulator.timers.divider, 1);
    assert_eq!(emulator.timers.divider_clock, 0);
}

#[test]
fn wraps_when_divider_register_overflows() {
    let mut emulator = initialize_emulator();
    emulator.cpu.clock.instruction_clock_cycles = 4;
    emulator.timers.m_cycles_clock = 3;
    emulator.timers.divider_clock = 15;
    emulator.timers.divider = 0xFF;
    step(&mut emulator);
    assert_eq!(emulator.timers.divider, 0);
    assert_eq!(emulator.timers.divider_clock, 0);
}

#[test]
fn increments_counter_register_at_a_fourth_the_rate_of_base_speed_when_configured() {
    let mut emulator = initialize_emulator();
    emulator.cpu.clock.instruction_clock_cycles = 4;
    emulator.timers.m_cycles_clock = 3;
    emulator.timers.base_clock = 3;
    emulator.timers.control = 0x06;
    step(&mut emulator);
    assert_eq!(emulator.timers.counter, 1);
    assert_eq!(emulator.timers.base_clock, 0);
}

#[test]
fn increments_counter_register_at_a_sixteenth_of_the_rate_of_base_speed_when_configured() {
    let mut emulator = initialize_emulator();
    emulator.cpu.clock.instruction_clock_cycles = 4;
    emulator.timers.m_cycles_clock = 3;
    emulator.timers.base_clock = 15;
    emulator.timers.control = 0x07;
    step(&mut emulator);
    assert_eq!(emulator.timers.counter, 1);
    assert_eq!(emulator.timers.base_clock, 0);
}

#[test]
fn increments_counter_register_at_a_sixty_fourth_of_the_rate_of_base_speed_when_configured() {
    let mut emulator = initialize_emulator();
    emulator.cpu.clock.instruction_clock_cycles = 4;
    emulator.timers.m_cycles_clock = 3;
    emulator.timers.base_clock = 63;
    emulator.timers.control = 0x04;
    step(&mut emulator);
    assert_eq!(emulator.timers.counter, 1);
    assert_eq!(emulator.timers.base_clock, 0);
}

#[test]
fn increments_counter_register_at_same_rate_of_base_speed_when_configured() {
    let mut emulator = initialize_emulator();
    emulator.cpu.clock.instruction_clock_cycles = 4;
    emulator.timers.m_cycles_clock = 3;
    emulator.timers.base_clock = 0;
    emulator.timers.control = 0x05;
    step(&mut emulator);
    assert_eq!(emulator.timers.counter, 1);
    assert_eq!(emulator.timers.base_clock, 0);
}

#[test]
fn should_not_increment_counter_register_at_wrong_time() {
    let mut emulator = initialize_emulator();
    emulator.cpu.clock.instruction_clock_cycles = 4;
    emulator.timers.m_cycles_clock = 3;
    emulator.timers.base_clock = 14;
    emulator.timers.control = 0x07;
    step(&mut emulator);
    assert_eq!(emulator.timers.counter, 0);
    assert_eq!(emulator.timers.base_clock, 15);
}

#[test]
fn should_not_increment_counter_register_if_timer_is_off() {
    let mut emulator = initialize_emulator();
    emulator.cpu.clock.instruction_clock_cycles = 4;
    emulator.timers.m_cycles_clock = 3;
    emulator.timers.base_clock = 15;
    emulator.timers.control = 0;
    step(&mut emulator);
    assert_eq!(emulator.timers.counter, 0);
    assert_eq!(emulator.timers.base_clock, 15);
}

#[test]
fn should_fire_interrupt_on_counter_register_overflow() {
    let mut emulator = initialize_emulator();
    emulator.cpu.clock.instruction_clock_cycles = 4;
    emulator.timers.m_cycles_clock = 3;
    emulator.timers.base_clock = 0x15;
    emulator.timers.control = 0x07;
    emulator.timers.counter = 0xFF;
    step(&mut emulator);
    assert_eq!(emulator.timers.counter, 0);
    assert_eq!(emulator.timers.base_clock, 0);
    assert_eq!(emulator.interrupts.flags, 0x04);
}

#[test]
fn should_reset_counter_register_to_modulo_on_overflow() {
    let mut emulator = initialize_emulator();
    emulator.cpu.clock.instruction_clock_cycles = 4;
    emulator.timers.m_cycles_clock = 3;
    emulator.timers.base_clock = 0x15;
    emulator.timers.control = 0x07;
    emulator.timers.counter = 0xFF;
    emulator.timers.modulo = 0x04;
    step(&mut emulator);
    assert_eq!(emulator.timers.counter, 0x04);
    assert_eq!(emulator.timers.base_clock, 0);
    assert_eq!(emulator.interrupts.flags, 0x04);
}