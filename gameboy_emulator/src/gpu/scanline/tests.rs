use crate::emulator::initialize_emulator;
use crate::gpu::sprites::{Sprite, collect_scanline_sprites};
use super::*;

const SAMPLE_TILE_A: [u8; 16] = [0x3C, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7E, 0x5E, 0x7E, 0x0A, 0x7C, 0x56, 0x38, 0x7C];
const SAMPLE_TILE_B: [u8; 16] = [0xFF, 0x0, 0x7E, 0xFF, 0x85, 0x81, 0x89, 0x83, 0x93, 0x85, 0xA5, 0x8B, 0xC9, 0x97, 0x7E, 0xFF];

fn write_sample_tile_to_memory(emulator: &mut Emulator, index: u16, tile_bytes: [u8; 16]) {
    let offset = index * 16;
    for (tile_byte_index, tile_byte) in tile_bytes.iter().enumerate() {
        emulator.memory.video_ram[(0x1000 + offset + tile_byte_index as u16) as usize] = *tile_byte;
    }
}

fn write_sample_tile_to_obj_memory(emulator: &mut Emulator, index: u16, tile_bytes: [u8; 16]) {
    let offset = index * 16;
    for (tile_byte_index, tile_byte) in tile_bytes.iter().enumerate() {
        emulator.memory.video_ram[(0x0000 + offset + tile_byte_index as u16) as usize] = *tile_byte;
    } 
}

#[test]
fn should_render_tile_line() {
    let mut emulator = initialize_emulator();
    write_sample_tile_to_memory(&mut emulator, 0, SAMPLE_TILE_A);
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
fn should_render_tile_line_with_sprite() {
    let mut emulator = initialize_emulator();

    write_sample_tile_to_memory(&mut emulator, 0, SAMPLE_TILE_A);
    write_sample_tile_to_obj_memory(&mut emulator, 1, SAMPLE_TILE_B);
    
    let sprite = Sprite {
        y_pos: 0,
        x_pos: 2,
        tile_index: 1,
        priority: false,
        y_flip: false,
        x_flip: false,
        dmg_palette: false
    };

    let mut sprites = Vec::new();
    sprites.push(sprite);

    emulator.gpu.sprite_buffer = sprites;
    emulator.gpu.registers.ly = 0;
    emulator.gpu.registers.palette = 0x1B;
    emulator.gpu.registers.obp0 = 0x1B;

    write_scanline(&mut emulator);

    assert_eq!(emulator.gpu.frame_buffer[0], 0x000000);
    assert_eq!(emulator.gpu.frame_buffer[1], 0xD3D3D3);
    assert_eq!(emulator.gpu.frame_buffer[2], 0xA9A9A9);
    assert_eq!(emulator.gpu.frame_buffer[3], 0xA9A9A9);
    assert_eq!(emulator.gpu.frame_buffer[4], 0xA9A9A9);
    assert_eq!(emulator.gpu.frame_buffer[5], 0xA9A9A9);
    assert_eq!(emulator.gpu.frame_buffer[6], 0xA9A9A9);
    assert_eq!(emulator.gpu.frame_buffer[7], 0xA9A9A9);
}

#[test]
fn should_render_tile_line_with_sprite_having_negative_y_pos() {
    let mut emulator = initialize_emulator();

    write_sample_tile_to_memory(&mut emulator, 0, SAMPLE_TILE_A);
    write_sample_tile_to_obj_memory(&mut emulator, 1, SAMPLE_TILE_B);
    
    let sprite = Sprite {
        y_pos: -2,
        x_pos: 2,
        tile_index: 1,
        priority: false,
        y_flip: false,
        x_flip: false,
        dmg_palette: false
    };

    let mut sprites = Vec::new();
    sprites.push(sprite);

    emulator.gpu.sprite_buffer = sprites;
    emulator.gpu.registers.ly = 0;
    emulator.gpu.registers.palette = 0x1B;
    emulator.gpu.registers.obp0 = 0x1B;

    write_scanline(&mut emulator);

    assert_eq!(emulator.gpu.frame_buffer[0], 0x000000);
    assert_eq!(emulator.gpu.frame_buffer[1], 0xD3D3D3);
    assert_eq!(emulator.gpu.frame_buffer[2], 0xFFFFFF);
    assert_eq!(emulator.gpu.frame_buffer[3], 0xFFFFFF);
    assert_eq!(emulator.gpu.frame_buffer[4], 0xFFFFFF);
    assert_eq!(emulator.gpu.frame_buffer[5], 0xFFFFFF);
    assert_eq!(emulator.gpu.frame_buffer[6], 0xD3D3D3);
    assert_eq!(emulator.gpu.frame_buffer[7], 0xA9A9A9); 
}

#[test]
fn should_flip_sprite_on_x_axis() {
    let mut emulator = initialize_emulator();

    write_sample_tile_to_memory(&mut emulator, 0, SAMPLE_TILE_A);
    write_sample_tile_to_obj_memory(&mut emulator, 1, SAMPLE_TILE_B);
    
    let sprite = Sprite {
        y_pos: -2,
        x_pos: 2,
        tile_index: 1,
        priority: false,
        y_flip: false,
        x_flip: true,
        dmg_palette: false
    };

    let mut sprites = Vec::new();
    sprites.push(sprite);

    emulator.gpu.sprite_buffer = sprites;
    emulator.gpu.registers.ly = 0;
    emulator.gpu.registers.palette = 0x1B;
    emulator.gpu.registers.obp0 = 0x1B;

    write_scanline(&mut emulator);

    assert_eq!(emulator.gpu.frame_buffer[0], 0x000000);
    assert_eq!(emulator.gpu.frame_buffer[1], 0xD3D3D3);
    assert_eq!(emulator.gpu.frame_buffer[2], 0xFFFFFF);
    assert_eq!(emulator.gpu.frame_buffer[3], 0xFFFFFF);
    assert_eq!(emulator.gpu.frame_buffer[4], 0xA9A9A9);
    assert_eq!(emulator.gpu.frame_buffer[5], 0xFFFFFF);
    assert_eq!(emulator.gpu.frame_buffer[6], 0xD3D3D3);
    assert_eq!(emulator.gpu.frame_buffer[7], 0x000000);  
}

#[test]
fn should_render_tile_line_in_middle_of_frame() {
    let mut emulator = initialize_emulator();
    write_sample_tile_to_memory(&mut emulator, 1, SAMPLE_TILE_A);
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
    write_sample_tile_to_memory(&mut emulator, 1, SAMPLE_TILE_A);
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
    write_sample_tile_to_memory(&mut emulator, 1, SAMPLE_TILE_A);
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
    write_sample_tile_to_memory(&mut emulator, 1, SAMPLE_TILE_A);
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

fn write_sprite(emulator: &mut Emulator, sprit_number: u8, y_pos: u8, x_pos: u8, attributes: u8) {
    let index = (sprit_number * 4) as usize;
    emulator.memory.object_attribute_memory[index] = y_pos;
    emulator.memory.object_attribute_memory[index + 1] = x_pos;
    emulator.memory.object_attribute_memory[index + 2] = 0x0;
    emulator.memory.object_attribute_memory[index + 3] = attributes;
}

#[test]
fn should_get_ten_sprites_from_oam_memory() {
    let mut emulator = initialize_emulator();
    emulator.gpu.registers.ly = 0;

    write_sprite(&mut emulator, 0, 0, 0, 0);
    write_sprite(&mut emulator, 1, 16, 0, 0);
    write_sprite(&mut emulator, 2, 44, 0, 0);
    write_sprite(&mut emulator, 3, 9, 0x1F, 0);
    write_sprite(&mut emulator, 4, 14, 0x2A, 0);
    write_sprite(&mut emulator, 5, 16, 0x60, 0);
    write_sprite(&mut emulator, 6, 0, 0xFF, 0);
    write_sprite(&mut emulator, 7, 10, 0x3F, 0);
    write_sprite(&mut emulator, 8, 16, 0x4A, 0);
    write_sprite(&mut emulator, 9, 14, 0x51, 0);
    write_sprite(&mut emulator, 10, 8, 0x22, 0);
    write_sprite(&mut emulator, 11, 11, 0x1B, 0);
    write_sprite(&mut emulator, 12, 13, 0x14, 0);
    write_sprite(&mut emulator, 13, 16, 0x55, 0);
    write_sprite(&mut emulator, 14, 14, 0x22, 0);
    write_sprite(&mut emulator, 15, 15, 0x23, 0);

    let sprites = collect_scanline_sprites(&emulator);

    assert_eq!(sprites.len(), 10);
    assert_eq!(sprites[0].y_pos, 0);
    assert_eq!(sprites[1].y_pos, -7);
    assert_eq!(sprites[2].y_pos, -2);
    assert_eq!(sprites[3].y_pos, 0);
    assert_eq!(sprites[4].y_pos, -6);
    assert_eq!(sprites[5].y_pos, 0);
    assert_eq!(sprites[6].y_pos, -2);
    assert_eq!(sprites[7].y_pos, -5);
    assert_eq!(sprites[8].y_pos, -3);
    assert_eq!(sprites[9].y_pos, 0);
}

#[test]
fn should_parse_sprite_attributes_correctly() {
    let mut emulator = initialize_emulator();
    
    write_sprite(&mut emulator, 0, 16, 0, 0b11000000);
    
    let sprites = collect_scanline_sprites(&emulator);

    assert_eq!(sprites[0].priority, true);
    assert_eq!(sprites[0].y_flip, true);
    assert_eq!(sprites[0].x_flip, false);
    assert_eq!(sprites[0].dmg_palette, false);
}
