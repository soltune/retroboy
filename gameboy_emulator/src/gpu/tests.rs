use crate::mmu::initialize_memory;

use super::*;

#[test]
fn should_move_from_oam_to_vram_mode() {
    let mut memory = initialize_memory();
    let mut gpu_state = GpuState { mode: 2, line: 0, mode_clock: 76 };
    step(&mut gpu_state, &mut memory, 4);
    assert_eq!(gpu_state.mode, 3);
    assert_eq!(gpu_state.mode_clock, 0);
}

#[test]
fn should_move_from_vram_to_hblank_mode() {
    let mut memory = initialize_memory();
    let mut gpu_state = GpuState { mode: 3, line: 0, mode_clock: 168 };
    step(&mut gpu_state, &mut memory, 4);
    assert_eq!(gpu_state.mode, 0);
    assert_eq!(gpu_state.mode_clock, 0);
}

#[test]
fn should_not_move_from_oam_to_vram_mode_too_early() {
    let mut memory = initialize_memory();
    let mut gpu_state = GpuState { mode: 2, line: 0, mode_clock: 40 };
    step(&mut gpu_state, &mut memory, 4);
    assert_eq!(gpu_state.mode, 2);
    assert_eq!(gpu_state.mode_clock, 44);
}

#[test]
fn should_move_back_to_oam_mode_from_hblank_if_not_at_last_line() {
    let mut memory = initialize_memory();
    let mut gpu_state = GpuState { mode: 0, line: 100, mode_clock: 200 };
    step(&mut gpu_state, &mut memory, 4);
    assert_eq!(gpu_state.mode, 2);
    assert_eq!(gpu_state.mode_clock, 0);
    assert_eq!(gpu_state.line, 101);
}

#[test]
fn should_move_to_vblank_mode_from_hblank_if_at_last_line() {
    let mut memory = initialize_memory();
    let mut gpu_state = GpuState { mode: 0, line: 142, mode_clock: 200 };
    step(&mut gpu_state, &mut memory, 4);
    assert_eq!(gpu_state.mode, 1);
    assert_eq!(gpu_state.mode_clock, 0);
    assert_eq!(gpu_state.line, 143);
}

#[test]
fn should_move_back_to_oam_mode_from_vblank_at_correct_time() {
    let mut memory = initialize_memory();
    let mut gpu_state = GpuState { mode: 1, line: 142, mode_clock: 4556 };
    step(&mut gpu_state, &mut memory, 4);
    assert_eq!(gpu_state.mode, 2);
    assert_eq!(gpu_state.mode_clock, 0);
    assert_eq!(gpu_state.line, 0);
}