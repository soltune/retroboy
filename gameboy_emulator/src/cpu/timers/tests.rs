use crate::mmu::initialize_memory;

use super::*;

#[test]
fn incrememnts_base_speed_by_number_of_m_cycles() {
    let mut memory = initialize_memory();
    increment_timer(&mut memory, 1);
    assert_eq!(memory.timer_registers.m_cycles_clock, 1);
}

#[test]
fn resets_base_speed_after_four_m_cycles() {
    let mut memory = initialize_memory();
    memory.timer_registers.m_cycles_clock = 3;
    increment_timer(&mut memory, 1);
    assert_eq!(memory.timer_registers.m_cycles_clock, 0);
}

#[test]
fn increments_divider_clock_after_four_m_cycles() {
    let mut memory = initialize_memory();
    memory.timer_registers.m_cycles_clock = 3;
    increment_timer(&mut memory, 1);
    assert_eq!(memory.timer_registers.divider_clock, 1);
}

#[test]
fn increments_divider_register_after_sixteen_divider_clock_increments() {
    let mut memory = initialize_memory();
    memory.timer_registers.m_cycles_clock = 3;
    memory.timer_registers.divider_clock = 15;
    increment_timer(&mut memory, 1);
    assert_eq!(memory.timer_registers.divider, 1);
    assert_eq!(memory.timer_registers.divider_clock, 0);
}

#[test]
fn wraps_when_divider_register_overflows() {
    let mut memory = initialize_memory();
    memory.timer_registers.m_cycles_clock = 3;
    memory.timer_registers.divider_clock = 15;
    memory.timer_registers.divider = 0xFF;
    increment_timer(&mut memory, 1);
    assert_eq!(memory.timer_registers.divider, 0);
    assert_eq!(memory.timer_registers.divider_clock, 0);
}

#[test]
fn increments_counter_register_at_a_fourth_the_rate_of_base_speed_when_configured() {
    let mut memory = initialize_memory();
    memory.timer_registers.m_cycles_clock = 3;
    memory.timer_registers.base_clock = 3;
    memory.timer_registers.control = 0x06;
    increment_timer(&mut memory, 1);
    assert_eq!(memory.timer_registers.counter, 1);
    assert_eq!(memory.timer_registers.base_clock, 0);
}

#[test]
fn increments_counter_register_at_a_sixteenth_of_the_rate_of_base_speed_when_configured() {
    let mut memory = initialize_memory();
    memory.timer_registers.m_cycles_clock = 3;
    memory.timer_registers.base_clock = 15;
    memory.timer_registers.control = 0x07;
    increment_timer(&mut memory, 1);
    assert_eq!(memory.timer_registers.counter, 1);
    assert_eq!(memory.timer_registers.base_clock, 0);
}

#[test]
fn increments_counter_register_at_a_sixty_fourth_of_the_rate_of_base_speed_when_configured() {
    let mut memory = initialize_memory();
    memory.timer_registers.m_cycles_clock = 3;
    memory.timer_registers.base_clock = 63;
    memory.timer_registers.control = 0x04;
    increment_timer(&mut memory, 1);
    assert_eq!(memory.timer_registers.counter, 1);
    assert_eq!(memory.timer_registers.base_clock, 0);
}

#[test]
fn increments_counter_register_at_same_rate_of_base_speed_when_configured() {
    let mut memory = initialize_memory();
    memory.timer_registers.m_cycles_clock = 3;
    memory.timer_registers.base_clock = 0;
    memory.timer_registers.control = 0x05;
    increment_timer(&mut memory, 1);
    assert_eq!(memory.timer_registers.counter, 1);
    assert_eq!(memory.timer_registers.base_clock, 0);
}

#[test]
fn should_not_increment_counter_register_at_wrong_time() {
    let mut memory = initialize_memory();
    memory.timer_registers.m_cycles_clock = 3;
    memory.timer_registers.base_clock = 14;
    memory.timer_registers.control = 0x07;
    increment_timer(&mut memory, 1);
    assert_eq!(memory.timer_registers.counter, 0);
    assert_eq!(memory.timer_registers.base_clock, 15);
}

#[test]
fn should_not_increment_counter_register_if_timer_is_off() {
    let mut memory = initialize_memory();
    memory.timer_registers.m_cycles_clock = 3;
    memory.timer_registers.base_clock = 15;
    memory.timer_registers.control = 0;
    increment_timer(&mut memory, 1);
    assert_eq!(memory.timer_registers.counter, 0);
    assert_eq!(memory.timer_registers.base_clock, 15);
}

#[test]
fn should_fire_interrupt_on_counter_register_overflow() {
    let mut memory = initialize_memory();
    memory.timer_registers.m_cycles_clock = 3;
    memory.timer_registers.base_clock = 0x15;
    memory.timer_registers.control = 0x07;
    memory.timer_registers.counter = 0xFF;
    increment_timer(&mut memory, 1);
    assert_eq!(memory.timer_registers.counter, 0);
    assert_eq!(memory.timer_registers.base_clock, 0);
    assert_eq!(memory.interrupt_registers.flags, 0x04);
}

#[test]
fn should_reset_counter_register_to_modulo_on_overflow() {
    let mut memory = initialize_memory();
    memory.timer_registers.m_cycles_clock = 3;
    memory.timer_registers.base_clock = 0x15;
    memory.timer_registers.control = 0x07;
    memory.timer_registers.counter = 0xFF;
    memory.timer_registers.modulo = 0x04;
    increment_timer(&mut memory, 1);
    assert_eq!(memory.timer_registers.counter, 0x04);
    assert_eq!(memory.timer_registers.base_clock, 0);
    assert_eq!(memory.interrupt_registers.flags, 0x04);
}