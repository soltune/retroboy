use crate::emulator::initialize_emulator;
use super::*;

#[test]
fn should_decrement_period_divider() {
    let mut emulator = initialize_emulator();
    emulator.apu.ch1_period_divider = 742;
    emulator.apu.ch1_period_low = 26;
    emulator.apu.ch1_period_high = 197;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.apu.ch1_period_divider, 741);
}

#[test]
fn should_decrement_period_divider_twice_with_eight_instruction_cycles() {
    let mut emulator = initialize_emulator();
    emulator.apu.ch1_period_divider = 742;
    emulator.apu.ch1_period_low = 26;
    emulator.apu.ch1_period_high = 197;
    emulator.cpu.clock.instruction_clock_cycles = 8;
    step(&mut emulator);
    assert_eq!(emulator.apu.ch1_period_divider, 740); 
}

#[test]
fn should_reload_period_divider_once_it_reaches_zero() {
   let mut emulator = initialize_emulator();
   emulator.apu.ch1_period_divider = 1;
   emulator.apu.ch1_period_low = 26;
   emulator.apu.ch1_period_high = 197;
   emulator.cpu.clock.instruction_clock_cycles = 4;
   step(&mut emulator);
   assert_eq!(emulator.apu.ch1_period_divider, 742);
}

#[test]
fn should_properly_wrap_period_divider_value_with_eight_instruction_cycles() {
    let mut emulator = initialize_emulator();
    emulator.apu.ch1_period_divider = 1;
    emulator.apu.ch1_period_low = 26;
    emulator.apu.ch1_period_high = 197;
    emulator.cpu.clock.instruction_clock_cycles = 8;
    step(&mut emulator);
    assert_eq!(emulator.apu.ch1_period_divider, 741);  
}

#[test]
fn should_increment_wave_duty_position_when_period_divider_reaches_zero() {
    let mut emulator = initialize_emulator();
    emulator.apu.ch1_period_divider = 1;
    emulator.apu.ch1_period_low = 26;
    emulator.apu.ch1_period_high = 197;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.apu.ch1_wave_duty_position, 1);
}

#[test]
fn should_reset_wave_duty_position_to_zero_when_increased_above_seven() {
    let mut emulator = initialize_emulator();
    emulator.apu.ch1_period_divider = 1;
    emulator.apu.ch1_period_low = 26;
    emulator.apu.ch1_period_high = 197;
    emulator.apu.ch1_wave_duty_position = 7;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.apu.ch1_wave_duty_position, 0);
}