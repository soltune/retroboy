use crate::emulator::initialize_emulator;
use super::*;

fn init_emulator_with_gpu_state(gpu_state: GpuState) -> Emulator {
    let emulator = initialize_emulator();
    Emulator { gpu: gpu_state, ..emulator }
}


#[test]
fn should_move_from_oam_to_vram_mode() {
    let gpu_state = GpuState { mode: 2, line: 0, mode_clock: 76 };
    let mut emulator = init_emulator_with_gpu_state(gpu_state);
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.gpu.mode, 3);
    assert_eq!(emulator.gpu.mode_clock, 0);
}

#[test]
fn should_move_from_vram_to_hblank_mode() {
    let gpu_state = GpuState { mode: 3, line: 0, mode_clock: 168 };
    let mut emulator = init_emulator_with_gpu_state(gpu_state);
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.gpu.mode, 0);
    assert_eq!(emulator.gpu.mode_clock, 0);
}

#[test]
fn should_not_move_from_oam_to_vram_mode_too_early() {
    let gpu_state = GpuState { mode: 2, line: 0, mode_clock: 40 };
    let mut emulator = init_emulator_with_gpu_state(gpu_state);
    emulator.cpu.clock.instruction_clock_cycles = 4; 
    step(&mut emulator);
    assert_eq!(emulator.gpu.mode, 2);
    assert_eq!(emulator.gpu.mode_clock, 44);
}

#[test]
fn should_move_back_to_oam_mode_from_hblank_if_not_at_last_line() {
    let gpu_state = GpuState { mode: 0, line: 100, mode_clock: 200 };
    let mut emulator = init_emulator_with_gpu_state(gpu_state);
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.gpu.mode, 2);
    assert_eq!(emulator.gpu.mode_clock, 0);
    assert_eq!(emulator.gpu.line, 101);
}

#[test]
fn should_move_to_vblank_mode_from_hblank_if_at_last_line() {
    let gpu_state = GpuState { mode: 0, line: 142, mode_clock: 200 };
    let mut emulator = init_emulator_with_gpu_state(gpu_state);
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.gpu.mode, 1);
    assert_eq!(emulator.gpu.mode_clock, 0);
    assert_eq!(emulator.gpu.line, 143);
}

#[test]
fn should_move_back_to_oam_mode_from_vblank_at_correct_time() {
    let gpu_state = GpuState { mode: 1, line: 152, mode_clock: 452 };
    let mut emulator = init_emulator_with_gpu_state(gpu_state);
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.gpu.mode, 2);
    assert_eq!(emulator.gpu.mode_clock, 0);
    assert_eq!(emulator.gpu.line, 0);
}