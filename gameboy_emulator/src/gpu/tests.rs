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

#[test]
fn should_return_bg_and_window_enabled_mode() {
    let mut emulator = initialize_emulator();
    emulator.gpu.registers.lcdc = 0x01;
    let result = get_bg_and_window_enabled_mode(emulator.gpu.registers.lcdc);
    assert_eq!(result, true);
}

#[test]
fn should_return_obj_enabled_mode() {
    let mut emulator = initialize_emulator();
    emulator.gpu.registers.lcdc = 0x02;
    let result = get_obj_enabled_mode(emulator.gpu.registers.lcdc);
    assert_eq!(result, true);
}

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

fn write_sample_tile_to_memory(emulator: &mut Emulator, index: u16) {
    let offset = index * 16;
    let tile_bytes: [u8; 16] = [
        0x3C, 
        0x7E,
        0x42,
        0x42,
        0x42,
        0x42,
        0x42,
        0x42,
        0x7E,
        0x5E,
        0x7E,
        0x0A,
        0x7C,
        0x56,
        0x38,
        0x7C
    ];
    for (tile_byte_index, tile_byte) in tile_bytes.iter().enumerate() {
        emulator.memory.video_ram[(0x1000 + offset + tile_byte_index as u16) as usize] = *tile_byte;
    }
}

#[test]
fn should_render_tile_line() {
    let mut emulator = initialize_emulator();
    write_sample_tile_to_memory(&mut emulator, 0);
    emulator.gpu.registers.ly = 0;
    emulator.gpu.registers.palette = 0x1B;
    write_scanline(&mut emulator);
    assert_eq!(emulator.gpu.frame_buffer[0], 0x000000);
    assert_eq!(emulator.gpu.frame_buffer[1], 0xD3D3D3);
    assert_eq!(emulator.gpu.frame_buffer[2], 0xFFFFFF);
    assert_eq!(emulator.gpu.frame_buffer[3], 0xFFFFFF);
    assert_eq!(emulator.gpu.frame_buffer[4], 0xFFFFFF);
    assert_eq!(emulator.gpu.frame_buffer[5], 0xFFFFFF);
    assert_eq!(emulator.gpu.frame_buffer[6], 0xD3D3D3);
    assert_eq!(emulator.gpu.frame_buffer[7], 0x000000);
}

#[test]
fn should_render_tile_line_in_middle_of_frame() {
    let mut emulator = initialize_emulator();
    write_sample_tile_to_memory(&mut emulator, 1);
    emulator.memory.video_ram[0x1A10] = 0x1;
    emulator.gpu.registers.ly = 3;
    emulator.gpu.registers.scy = 0x80;
    emulator.gpu.registers.scx = 0x80;
    emulator.gpu.registers.palette = 0x1B;
    write_scanline(&mut emulator);
    assert_eq!(emulator.gpu.frame_buffer[0x1E0], 0x000000);
    assert_eq!(emulator.gpu.frame_buffer[0x1E1], 0xFFFFFF);
    assert_eq!(emulator.gpu.frame_buffer[0x1E2], 0x000000);
    assert_eq!(emulator.gpu.frame_buffer[0x1E3], 0x000000);
    assert_eq!(emulator.gpu.frame_buffer[0x1E4], 0x000000);
    assert_eq!(emulator.gpu.frame_buffer[0x1E5], 0x000000);
    assert_eq!(emulator.gpu.frame_buffer[0x1E6], 0xFFFFFF);
    assert_eq!(emulator.gpu.frame_buffer[0x1E7], 0x000000);
}

#[test]
fn should_render_tile_line_properly_with_greater_scroll_x_value() {
    let mut emulator = initialize_emulator();
    write_sample_tile_to_memory(&mut emulator, 1);
    emulator.memory.video_ram[0x1A10] = 0x1;
    emulator.gpu.registers.ly = 3;
    emulator.gpu.registers.scy = 0x80;
    emulator.gpu.registers.scx = 0x82;
    emulator.gpu.registers.palette = 0x1B;
    write_scanline(&mut emulator);
    assert_eq!(emulator.gpu.frame_buffer[0x1E0], 0x000000);
    assert_eq!(emulator.gpu.frame_buffer[0x1E1], 0x000000);
    assert_eq!(emulator.gpu.frame_buffer[0x1E2], 0x000000);
    assert_eq!(emulator.gpu.frame_buffer[0x1E3], 0x000000);
    assert_eq!(emulator.gpu.frame_buffer[0x1E4], 0xFFFFFF);
    assert_eq!(emulator.gpu.frame_buffer[0x1E5], 0x000000);
    assert_eq!(emulator.gpu.frame_buffer[0x1E6], 0x000000);
    assert_eq!(emulator.gpu.frame_buffer[0x1E7], 0x000000);
}

#[test]
fn should_wrap_around_when_rendering_past_max_tile_map_x_value() {
    let mut emulator = initialize_emulator();
    write_sample_tile_to_memory(&mut emulator, 1);
    emulator.memory.video_ram[0x1800] = 0x1;
    emulator.gpu.registers.ly = 0;
    emulator.gpu.registers.scx = 0xFE;
    emulator.gpu.registers.palette = 0x1B;
    write_scanline(&mut emulator);
    assert_eq!(emulator.gpu.frame_buffer[0], 0x000000);
    assert_eq!(emulator.gpu.frame_buffer[1], 0x000000);
    assert_eq!(emulator.gpu.frame_buffer[2], 0x000000);
    assert_eq!(emulator.gpu.frame_buffer[3], 0xD3D3D3);
    assert_eq!(emulator.gpu.frame_buffer[4], 0xFFFFFF);
    assert_eq!(emulator.gpu.frame_buffer[5], 0xFFFFFF);
    assert_eq!(emulator.gpu.frame_buffer[6], 0xFFFFFF);
    assert_eq!(emulator.gpu.frame_buffer[7], 0xFFFFFF);
}

#[test]
fn should_wrap_around_when_rendering_past_max_tile_map_y_value() {
    let mut emulator = initialize_emulator();
    write_sample_tile_to_memory(&mut emulator, 1);
    emulator.memory.video_ram[0x1800] = 0x1;
    emulator.gpu.registers.ly = 2;
    emulator.gpu.registers.scy = 0xFE;
    emulator.gpu.registers.palette = 0x1B;
    write_scanline(&mut emulator);
    assert_eq!(emulator.gpu.frame_buffer[0x140], 0x000000);
    assert_eq!(emulator.gpu.frame_buffer[0x141], 0xD3D3D3);
    assert_eq!(emulator.gpu.frame_buffer[0x142], 0xFFFFFF);
    assert_eq!(emulator.gpu.frame_buffer[0x143], 0xFFFFFF);
    assert_eq!(emulator.gpu.frame_buffer[0x144], 0xFFFFFF);
    assert_eq!(emulator.gpu.frame_buffer[0x145], 0xFFFFFF);
    assert_eq!(emulator.gpu.frame_buffer[0x146], 0xD3D3D3);
    assert_eq!(emulator.gpu.frame_buffer[0x147], 0x000000);
}

fn write_sprite(emulator: &mut Emulator, sprit_number: u8, y_pos: u8, x_pos: u8) {
    let index = (sprit_number * 4) as usize;
    emulator.memory.object_attribute_memory[index] = y_pos;
    emulator.memory.object_attribute_memory[index + 1] = x_pos;
    emulator.memory.object_attribute_memory[index + 2] = 0x0;
    emulator.memory.object_attribute_memory[index + 3] = 0x0;
}

#[test]
fn should_get_ten_sprites_from_oam_memory() {
    let mut emulator = initialize_emulator();
    emulator.gpu.registers.ly = 0;

    write_sprite(&mut emulator, 0, 0, 0);
    write_sprite(&mut emulator, 1, 16, 0);
    write_sprite(&mut emulator, 2, 44, 0);
    write_sprite(&mut emulator, 3, 9, 0x1F);
    write_sprite(&mut emulator, 4, 14, 0x2A);
    write_sprite(&mut emulator, 5, 16, 0x60);
    write_sprite(&mut emulator, 6, 0, 0xFF);
    write_sprite(&mut emulator, 7, 10, 0x3F);
    write_sprite(&mut emulator, 8, 16, 0x4A);
    write_sprite(&mut emulator, 9, 14, 0x51);
    write_sprite(&mut emulator, 10, 8, 0x22);
    write_sprite(&mut emulator, 11, 11, 0x1B);
    write_sprite(&mut emulator, 12, 13, 0x14);
    write_sprite(&mut emulator, 13, 16, 0x55);
    write_sprite(&mut emulator, 14, 14, 0x22);
    write_sprite(&mut emulator, 15, 15, 0x23);

    let sprites = collect_scanline_sprites(&mut emulator);

    assert_eq!(sprites.len(), 10);
    assert_eq!(sprites[0].y_pos, 16);
    assert_eq!(sprites[1].y_pos, 9);
    assert_eq!(sprites[2].y_pos, 14);
    assert_eq!(sprites[3].y_pos, 16);
    assert_eq!(sprites[4].y_pos, 10);
    assert_eq!(sprites[5].y_pos, 16);
    assert_eq!(sprites[6].y_pos, 14);
    assert_eq!(sprites[7].y_pos, 8);
    assert_eq!(sprites[8].y_pos, 11);
    assert_eq!(sprites[9].y_pos, 13);
}