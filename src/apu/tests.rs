use crate::emulator::initialize_emulator;
use super::*;

#[test]
fn should_decrement_period_divider() {
    let mut emulator = initialize_emulator();
    emulator.apu.ch1_period_divider = 0b010100011010;
    emulator.apu.ch1_period_low = 0b00011010;
    emulator.apu.ch1_period_high = 0b11000101;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.apu.ch1_period_divider, 0b010100011001);
}

#[test]
fn should_decrement_period_divider_twice_with_eight_instruction_cycles() {
    let mut emulator = initialize_emulator();
    emulator.apu.ch1_period_divider = 0b010100011010;
    emulator.apu.ch1_period_low = 0b00011010;
    emulator.apu.ch1_period_high = 0b11000101;
    emulator.cpu.clock.instruction_clock_cycles = 8;
    step(&mut emulator);
    assert_eq!(emulator.apu.ch1_period_divider, 0b010100011000); 
}

#[test]
fn should_reload_period_divider_once_it_reaches_zero() {
   let mut emulator = initialize_emulator();
   emulator.apu.ch1_period_divider = 0b1;
   emulator.apu.ch1_period_low = 0b00011010;
   emulator.apu.ch1_period_high = 0b11000101;
   emulator.cpu.clock.instruction_clock_cycles = 4;
   step(&mut emulator);
   assert_eq!(emulator.apu.ch1_period_divider, 0b10100011010);
}
