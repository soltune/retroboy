use crate::emulator::initialize_emulator;
use super::*;

#[test]
fn should_return_obj_size_mode() {
   let mut emulator = initialize_emulator();
   emulator.gpu.registers.lcdc = 0x04;
   let result = get_obj_size_mode(emulator.gpu.registers.lcdc);
   assert_eq!(result, true); 
}

#[test]
fn should_return_bg_tile_map_mode() {
    let mut emulator = initialize_emulator();
    emulator.gpu.registers.lcdc = 0x08;
    let result = get_bg_tile_map_mode(emulator.gpu.registers.lcdc);
    assert_eq!(result, true);
}

#[test]
fn should_return_tile_data_mode() {
    let mut emulator = initialize_emulator();
    emulator.gpu.registers.lcdc = 0x10;
    let result = get_tile_data_addressing_mode(emulator.gpu.registers.lcdc);
    assert_eq!(result, true);
}

#[test]
fn should_return_window_enabled_mode() {
    let mut emulator = initialize_emulator();
    emulator.gpu.registers.lcdc = 0x20;
    let result = get_window_enabled_mode(emulator.gpu.registers.lcdc);
    assert_eq!(result, true);
}

#[test]
fn should_return_window_tile_map_mode() {
    let mut emulator = initialize_emulator();
    emulator.gpu.registers.lcdc = 0x40;
    let result = get_window_tile_map_mode(emulator.gpu.registers.lcdc);
    assert_eq!(result, true);
}

#[test]
fn should_return_lcdc_enabled_mode() {
    let mut emulator = initialize_emulator();
    emulator.gpu.registers.lcdc = 0x80;
    let result = get_lcd_enabled_mode(emulator.gpu.registers.lcdc);
    assert_eq!(result, true);
}