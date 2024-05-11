use crate::emulator::initialize_emulator;
use super::*;

fn noop_renderer(_: &Vec<u8>) {}

#[test]
fn should_move_from_oam_to_vram_mode() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 2;
    emulator.gpu.registers.ly = 0;
    emulator.gpu.mode_clock = 76;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator, noop_renderer);
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
    step(&mut emulator, noop_renderer);
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
    step(&mut emulator, noop_renderer);
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
    step(&mut emulator, noop_renderer);
    assert_eq!(emulator.gpu.mode, 2);
    assert_eq!(emulator.gpu.mode_clock, 0);
    assert_eq!(emulator.gpu.registers.ly, 101);
}

#[test]
fn should_move_to_vblank_mode_from_hblank_if_at_last_line() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 0;
    emulator.gpu.registers.ly = 143;
    emulator.gpu.mode_clock = 200;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator, noop_renderer);
    assert_eq!(emulator.gpu.mode, 1);
    assert_eq!(emulator.gpu.mode_clock, 0);
    assert_eq!(emulator.gpu.registers.ly, 144);
}

#[test]
fn should_fire_vblank_interrupt_when_entering_vblank_mode() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 0;
    emulator.gpu.registers.ly = 143;
    emulator.gpu.mode_clock = 200;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator, noop_renderer);
    assert_eq!(emulator.interrupts.flags, 0x1);
}

#[test]
fn should_move_back_to_oam_mode_from_vblank_at_correct_time() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 1;
    emulator.gpu.registers.ly = 153;
    emulator.gpu.mode_clock = 452;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator, noop_renderer);
    assert_eq!(emulator.gpu.mode, 2);
    assert_eq!(emulator.gpu.mode_clock, 0);
    assert_eq!(emulator.gpu.registers.ly, 0);
}

#[test]
fn should_update_stat_register_with_mode_2_status() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 1;
    emulator.gpu.registers.ly = 153;
    emulator.gpu.mode_clock = 452;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    emulator.gpu.registers.stat = 0b00000001;
    step(&mut emulator, noop_renderer);
    assert_eq!(emulator.gpu.registers.stat, 0b00000110);
}

#[test]
fn should_fire_stat_interrupt_on_switch_to_mode_2_when_enabled() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 1;
    emulator.gpu.registers.ly = 153;
    emulator.gpu.mode_clock = 452;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    emulator.gpu.registers.stat = 0b00100001;
    step(&mut emulator, noop_renderer);
    assert_eq!(emulator.interrupts.flags, 0x02);
}

#[test]
fn should_update_stat_register_with_mode_3_status() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 2;
    emulator.gpu.registers.ly = 0;
    emulator.gpu.mode_clock = 76;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    emulator.gpu.registers.stat = 0b00000010;
    step(&mut emulator, noop_renderer);
    assert_eq!(emulator.gpu.registers.stat, 0b00000011);
}

#[test]
fn should_update_stat_register_with_mode_0_status() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 3;
    emulator.gpu.registers.ly = 0;
    emulator.gpu.mode_clock = 168;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    emulator.gpu.registers.stat = 0b00000011;
    step(&mut emulator, noop_renderer);
    assert_eq!(emulator.gpu.registers.stat, 0b00000000);
}

#[test]
fn should_fire_stat_interrupt_on_switch_to_mode_0_if_enabled() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 3;
    emulator.gpu.registers.ly = 0;
    emulator.gpu.mode_clock = 168;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    emulator.gpu.registers.stat = 0b00001011;
    step(&mut emulator, noop_renderer);
    assert_eq!(emulator.interrupts.flags, 0x02);
}

#[test]
fn should_update_stat_register_with_mode_1_status() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 0;
    emulator.gpu.registers.ly = 143;
    emulator.gpu.mode_clock = 200;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    emulator.gpu.registers.stat = 0b00000000;
    step(&mut emulator, noop_renderer);
    assert_eq!(emulator.gpu.registers.stat, 0b00000001);
}

#[test]
fn should_fire_stat_interrupt_on_switch_to_mode_1_if_enabled() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 0;
    emulator.gpu.registers.ly = 143;
    emulator.gpu.mode_clock = 200;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    emulator.gpu.registers.stat = 0b00010000;
    step(&mut emulator, noop_renderer);
    assert_eq!(emulator.interrupts.flags, 0x03);
}

#[test]
fn should_fire_stat_interrupt_when_lyc_equals_ly_if_enabled() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 0;
    emulator.gpu.registers.ly = 13;
    emulator.gpu.registers.lyc = 14;
    emulator.gpu.mode_clock = 200;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    emulator.gpu.registers.stat = 0b01000000;
    step(&mut emulator, noop_renderer);
    assert_eq!(emulator.interrupts.flags, 0x02);
}

#[test]
fn should_update_stat_register_when_lyc_equals_ly() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 0;
    emulator.gpu.registers.ly = 13;
    emulator.gpu.registers.lyc = 14;
    emulator.gpu.mode_clock = 200;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    emulator.gpu.registers.stat = 0b01000000;
    step(&mut emulator, noop_renderer);
    assert_eq!(emulator.gpu.registers.stat, 0b01000110);
}

#[test]
fn should_update_stat_register_when_lyc_is_not_equal_to_ly() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 0;
    emulator.gpu.registers.ly = 14;
    emulator.gpu.registers.lyc = 14;
    emulator.gpu.mode_clock = 200;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    emulator.gpu.registers.stat = 0b01000100;
    step(&mut emulator, noop_renderer);
    assert_eq!(emulator.gpu.registers.stat, 0b01000010);
}

#[test]
fn should_not_fire_stat_interrupt_when_lyc_equals_ly_if_disabled() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 0;
    emulator.gpu.registers.ly = 13;
    emulator.gpu.registers.lyc = 14;
    emulator.gpu.mode_clock = 200;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    emulator.gpu.registers.stat = 0b00000000;
    step(&mut emulator, noop_renderer);
    assert_eq!(emulator.interrupts.flags, 0x0);
}