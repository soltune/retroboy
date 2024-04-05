use crate::emulator::initialize_emulator;
use super::*;

#[test]
fn should_move_from_oam_to_vram_mode() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 2;
    emulator.gpu.registers.ly = 0;
    emulator.gpu.mode_clock = 76;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.gpu.mode, 3);
    assert_eq!(emulator.gpu.mode_clock, 0);
}

#[test]
fn should_move_from_vram_to_hblank_mode() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 3;
    emulator.gpu.registers.ly = 0;
    emulator.gpu.mode_clock = 168;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.gpu.mode, 0);
    assert_eq!(emulator.gpu.mode_clock, 0);
}

#[test]
fn should_not_move_from_oam_to_vram_mode_too_early() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 2;
    emulator.gpu.registers.ly = 0;
    emulator.gpu.mode_clock = 40;
    emulator.cpu.clock.instruction_clock_cycles = 4; 
    step(&mut emulator);
    assert_eq!(emulator.gpu.mode, 2);
    assert_eq!(emulator.gpu.mode_clock, 44);
}

#[test]
fn should_move_back_to_oam_mode_from_hblank_if_not_at_last_line() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 0;
    emulator.gpu.registers.ly = 100;
    emulator.gpu.mode_clock = 200;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.gpu.mode, 2);
    assert_eq!(emulator.gpu.mode_clock, 0);
    assert_eq!(emulator.gpu.registers.ly, 101);
}

#[test]
fn should_move_to_vblank_mode_from_hblank_if_at_last_line() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 0;
    emulator.gpu.registers.ly = 142;
    emulator.gpu.mode_clock = 200;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.gpu.mode, 1);
    assert_eq!(emulator.gpu.mode_clock, 0);
    assert_eq!(emulator.gpu.registers.ly, 143);
}

#[test]
fn should_fire_vblank_interrupt_when_entering_vblank_mode() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 0;
    emulator.gpu.registers.ly = 142;
    emulator.gpu.mode_clock = 200;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.interrupts.flags, 0x1);
}

#[test]
fn should_move_back_to_oam_mode_from_vblank_at_correct_time() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 1;
    emulator.gpu.registers.ly = 152;
    emulator.gpu.mode_clock = 452;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.gpu.mode, 2);
    assert_eq!(emulator.gpu.mode_clock, 0);
    assert_eq!(emulator.gpu.registers.ly, 0);
}
