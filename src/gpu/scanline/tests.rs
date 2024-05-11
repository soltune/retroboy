use crate::emulator::initialize_emulator;
use crate::gpu::colors::{Color, BLACK, DARK_GRAY, LIGHT_GRAY, WHITE};
use crate::gpu::sprites::{Sprite, collect_scanline_sprites};
use super::*;

const BLACK_TILE: [u8; 16] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
const SAMPLE_TILE_A: [u8; 16] = [0x3C, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7E, 0x5E, 0x7E, 0x0A, 0x7C, 0x56, 0x38, 0x7C];
const SAMPLE_TILE_B: [u8; 16] = [0xFF, 0x0, 0x7E, 0xFF, 0x85, 0x81, 0x89, 0x83, 0x93, 0x85, 0xA5, 0x8B, 0xC9, 0x97, 0x7E, 0xFF];
const WINDOW_TILE: [u8; 16] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];

fn write_tile_to_memory(emulator: &mut Emulator, base_address: u16, index: u16, tile_bytes: [u8; 16]) {
    let offset = index * 16;
    for (tile_byte_index, tile_byte) in tile_bytes.iter().enumerate() {
        emulator.memory.video_ram[(base_address + offset + tile_byte_index as u16) as usize] = *tile_byte;
    }
}

fn write_tile_to_bg_memory(emulator: &mut Emulator, index: u16, tile_bytes: [u8; 16]) {
    write_tile_to_memory(emulator, 0x1000, index, tile_bytes)
}

fn write_tile_to_obj_memory(emulator: &mut Emulator, index: u16, tile_bytes: [u8; 16]) {
    write_tile_to_memory(emulator, 0x0000, index, tile_bytes)
}

fn write_window_tile_index_to_memory(emulator: &mut Emulator, position_index: u16, tile_index: u8) {
    emulator.memory.video_ram[(0x1C00 + position_index) as usize] = tile_index;
}

fn write_sprite(emulator: &mut Emulator, sprit_number: u8, y_pos: u8, x_pos: u8, attributes: u8) {
    let index = (sprit_number * 4) as usize;
    emulator.memory.object_attribute_memory[index] = y_pos;
    emulator.memory.object_attribute_memory[index + 1] = x_pos;
    emulator.memory.object_attribute_memory[index + 2] = 0x0;
    emulator.memory.object_attribute_memory[index + 3] = attributes;
}

fn line(index: u32) -> u32 {
    index * GB_SCREEN_WIDTH
}

fn assert_pixel_color(frame_buffer: &Vec<u8>, pixel_position: u32, color: Color) {
    let pixel_index = (pixel_position * 4) as usize;
    assert_eq!(frame_buffer[pixel_index], color[0]);
    assert_eq!(frame_buffer[pixel_index + 1], color[1]);
    assert_eq!(frame_buffer[pixel_index + 2], color[2]);
    assert_eq!(frame_buffer[pixel_index + 3], color[3]);
}

#[test]
fn should_render_nothing_if_lcd_enable_flag_is_off() {
    let mut emulator = initialize_emulator();

    write_tile_to_bg_memory(&mut emulator, 0, SAMPLE_TILE_A);
    
    emulator.gpu.registers.ly = 0;
    emulator.gpu.registers.palette = 0b00011011;
    emulator.gpu.registers.lcdc = 0b00000011;
    
    write_scanline(&mut emulator);

    let frame_buffer = &emulator.gpu.frame_buffer;
    
    assert_pixel_color(frame_buffer, 0, WHITE);
    assert_pixel_color(frame_buffer, 1, WHITE);
    assert_pixel_color(frame_buffer, 2, WHITE);
    assert_pixel_color(frame_buffer, 3, WHITE);
    assert_pixel_color(frame_buffer, 4, WHITE);
    assert_pixel_color(frame_buffer, 5, WHITE);
    assert_pixel_color(frame_buffer, 6, WHITE);
    assert_pixel_color(frame_buffer, 7, WHITE);
}

#[test]
fn should_render_tile_line() {
    let mut emulator = initialize_emulator();

    write_tile_to_bg_memory(&mut emulator, 0, SAMPLE_TILE_A);
    
    emulator.gpu.registers.ly = 0;
    emulator.gpu.registers.palette = 0b00011011;
    emulator.gpu.registers.lcdc = 0b10000011;
    
    write_scanline(&mut emulator);

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_pixel_color(frame_buffer, 0, BLACK);
    assert_pixel_color(frame_buffer, 1, LIGHT_GRAY);
    assert_pixel_color(frame_buffer, 2, WHITE);
    assert_pixel_color(frame_buffer, 3, WHITE);
    assert_pixel_color(frame_buffer, 4, WHITE);
    assert_pixel_color(frame_buffer, 5, WHITE);
    assert_pixel_color(frame_buffer, 6, LIGHT_GRAY);
    assert_pixel_color(frame_buffer, 7, BLACK);
}

#[test]
fn should_render_multiple_tile_lines() {
    let mut emulator = initialize_emulator();
    
    write_tile_to_bg_memory(&mut emulator, 0, SAMPLE_TILE_A);

    emulator.gpu.registers.palette = 0b00011011;
    emulator.gpu.registers.lcdc = 0b10000011;

    for _ in 0..3 {
        write_scanline(&mut emulator);
        emulator.gpu.registers.ly += 1;
    }

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_pixel_color(frame_buffer, 0, BLACK);
    assert_pixel_color(frame_buffer, 1, LIGHT_GRAY);
    assert_pixel_color(frame_buffer, 2, WHITE);
    assert_pixel_color(frame_buffer, 3, WHITE);
    assert_pixel_color(frame_buffer, 4, WHITE);
    assert_pixel_color(frame_buffer, 5, WHITE);
    assert_pixel_color(frame_buffer, 6, LIGHT_GRAY);
    assert_pixel_color(frame_buffer, 7, BLACK);

    assert_pixel_color(frame_buffer, line(1) + 0, BLACK);
    assert_pixel_color(frame_buffer, line(1) + 1, WHITE);
    assert_pixel_color(frame_buffer, line(1) + 2, BLACK);
    assert_pixel_color(frame_buffer, line(1) + 3, BLACK);
    assert_pixel_color(frame_buffer, line(1) + 4, BLACK);
    assert_pixel_color(frame_buffer, line(1) + 5, BLACK);
    assert_pixel_color(frame_buffer, line(1) + 6, WHITE);
    assert_pixel_color(frame_buffer, line(1) + 7, BLACK);

    assert_pixel_color(frame_buffer, line(2) + 0, BLACK);
    assert_pixel_color(frame_buffer, line(2) + 1, WHITE);
    assert_pixel_color(frame_buffer, line(2) + 2, BLACK);
    assert_pixel_color(frame_buffer, line(2) + 3, BLACK);
    assert_pixel_color(frame_buffer, line(2) + 4, BLACK);
    assert_pixel_color(frame_buffer, line(2) + 5, BLACK);
    assert_pixel_color(frame_buffer, line(2) + 6, WHITE);
    assert_pixel_color(frame_buffer, line(2) + 7, BLACK);
}

#[test]
fn should_overlay_window_over_multiple_tile_lines() {
    let mut emulator = initialize_emulator();

    write_tile_to_bg_memory(&mut emulator, 0, SAMPLE_TILE_A);
    write_tile_to_bg_memory(&mut emulator, 1, WINDOW_TILE);
    write_window_tile_index_to_memory(&mut emulator, 0, 1);

    emulator.gpu.registers.wy = 1;
    emulator.gpu.registers.wx = 8;
    emulator.gpu.registers.palette = 0b00011011;
    emulator.gpu.registers.lcdc = 0b11100011;

    for _ in 0..3 {
        write_scanline(&mut emulator);
        emulator.gpu.registers.ly += 1;
    }

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_pixel_color(frame_buffer, 0, BLACK);
    assert_pixel_color(frame_buffer, 1, LIGHT_GRAY);
    assert_pixel_color(frame_buffer, 2, WHITE);
    assert_pixel_color(frame_buffer, 3, WHITE);
    assert_pixel_color(frame_buffer, 4, WHITE);
    assert_pixel_color(frame_buffer, 5, WHITE);
    assert_pixel_color(frame_buffer, 6, LIGHT_GRAY);
    assert_pixel_color(frame_buffer, 7, BLACK);

    assert_pixel_color(frame_buffer, line(1) + 0, BLACK);
    assert_pixel_color(frame_buffer, line(1) + 1, WHITE);
    assert_pixel_color(frame_buffer, line(1) + 2, WHITE);
    assert_pixel_color(frame_buffer, line(1) + 3, WHITE);
    assert_pixel_color(frame_buffer, line(1) + 4, WHITE);
    assert_pixel_color(frame_buffer, line(1) + 5, WHITE);
    assert_pixel_color(frame_buffer, line(1) + 6, WHITE);
    assert_pixel_color(frame_buffer, line(1) + 7, WHITE);

    assert_pixel_color(frame_buffer, line(2) + 0, BLACK);
    assert_pixel_color(frame_buffer, line(2) + 1, WHITE);
    assert_pixel_color(frame_buffer, line(2) + 2, WHITE);
    assert_pixel_color(frame_buffer, line(2) + 3, WHITE);
    assert_pixel_color(frame_buffer, line(2) + 4, WHITE);
    assert_pixel_color(frame_buffer, line(2) + 5, WHITE);
    assert_pixel_color(frame_buffer, line(2) + 6, WHITE);
    assert_pixel_color(frame_buffer, line(2) + 7, WHITE);
}

#[test]
fn should_render_tile_line_in_middle_of_frame() {
    let mut emulator = initialize_emulator();
    
    write_tile_to_bg_memory(&mut emulator, 1, SAMPLE_TILE_A);
    
    emulator.memory.video_ram[0x1A10] = 0x1;
    emulator.gpu.registers.ly = 3;
    emulator.gpu.registers.scy = 0x80;
    emulator.gpu.registers.scx = 0x80;
    emulator.gpu.registers.palette = 0b00011011;
    emulator.gpu.registers.lcdc = 0b10000011;
    
    write_scanline(&mut emulator);

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_pixel_color(frame_buffer, line(3) + 0, BLACK);
    assert_pixel_color(frame_buffer, line(3) + 1, WHITE);
    assert_pixel_color(frame_buffer, line(3) + 2, BLACK);
    assert_pixel_color(frame_buffer, line(3) + 3, BLACK);
    assert_pixel_color(frame_buffer, line(3) + 4, BLACK);
    assert_pixel_color(frame_buffer, line(3) + 5, BLACK);
    assert_pixel_color(frame_buffer, line(3) + 6, WHITE);
    assert_pixel_color(frame_buffer, line(3) + 7, BLACK);
}

#[test]
fn should_render_tile_line_properly_with_greater_scroll_x_value() {
    let mut emulator = initialize_emulator();
    
    write_tile_to_bg_memory(&mut emulator, 1, SAMPLE_TILE_A);
    
    emulator.memory.video_ram[0x1A10] = 0x1;
    emulator.gpu.registers.ly = 3;
    emulator.gpu.registers.scy = 0x80;
    emulator.gpu.registers.scx = 0x82;
    emulator.gpu.registers.palette = 0b00011011;
    emulator.gpu.registers.lcdc = 0b10000011;
    
    write_scanline(&mut emulator);

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_pixel_color(frame_buffer, line(3) + 0, BLACK);
    assert_pixel_color(frame_buffer, line(3) + 1, BLACK);
    assert_pixel_color(frame_buffer, line(3) + 2, BLACK);
    assert_pixel_color(frame_buffer, line(3) + 3, BLACK);
    assert_pixel_color(frame_buffer, line(3) + 4, WHITE);
    assert_pixel_color(frame_buffer, line(3) + 5, BLACK);
    assert_pixel_color(frame_buffer, line(3) + 6, BLACK);
    assert_pixel_color(frame_buffer, line(3) + 7, BLACK);
}

#[test]
fn should_wrap_around_when_rendering_past_max_tile_map_x_value() {
    let mut emulator = initialize_emulator();
    
    write_tile_to_bg_memory(&mut emulator, 1, SAMPLE_TILE_A);
    
    emulator.memory.video_ram[0x1800] = 0x1;
    emulator.gpu.registers.ly = 0;
    emulator.gpu.registers.scx = 0xFE;
    emulator.gpu.registers.palette = 0b00011011;
    emulator.gpu.registers.lcdc = 0b10000011;
    
    write_scanline(&mut emulator);

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_pixel_color(frame_buffer, 0, BLACK);
    assert_pixel_color(frame_buffer, 1, BLACK);
    assert_pixel_color(frame_buffer, 2, BLACK);
    assert_pixel_color(frame_buffer, 3, LIGHT_GRAY);
    assert_pixel_color(frame_buffer, 4, WHITE);
    assert_pixel_color(frame_buffer, 5, WHITE);
    assert_pixel_color(frame_buffer, 6, WHITE);
    assert_pixel_color(frame_buffer, 7, WHITE);
}

#[test]
fn should_wrap_around_when_rendering_past_max_tile_map_y_value() {
    let mut emulator = initialize_emulator();
    
    write_tile_to_bg_memory(&mut emulator, 1, SAMPLE_TILE_A);
    
    emulator.memory.video_ram[0x1800] = 0x1;
    emulator.gpu.registers.ly = 2;
    emulator.gpu.registers.scy = 0xFE;
    emulator.gpu.registers.palette = 0b00011011;
    emulator.gpu.registers.lcdc = 0b10000011;
    
    write_scanline(&mut emulator);

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_pixel_color(frame_buffer, line(2) + 0, BLACK);
    assert_pixel_color(frame_buffer, line(2) + 1, LIGHT_GRAY);
    assert_pixel_color(frame_buffer, line(2) + 2, WHITE);
    assert_pixel_color(frame_buffer, line(2) + 3, WHITE);
    assert_pixel_color(frame_buffer, line(2) + 4, WHITE);
    assert_pixel_color(frame_buffer, line(2) + 5, WHITE);
    assert_pixel_color(frame_buffer, line(2) + 6, LIGHT_GRAY);
    assert_pixel_color(frame_buffer, line(2) + 7, BLACK);
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

#[test]
fn should_render_tile_line_with_sprite() {
    let mut emulator = initialize_emulator();

    write_tile_to_bg_memory(&mut emulator, 0, SAMPLE_TILE_A);
    write_tile_to_obj_memory(&mut emulator, 1, SAMPLE_TILE_B);
    
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
    emulator.gpu.registers.palette = 0b00011011;
    emulator.gpu.registers.obp0 = 0b00011011;
    emulator.gpu.registers.lcdc = 0b10000011;

    write_scanline(&mut emulator);

    let frame_buffer: &Vec<u8> = &emulator.gpu.frame_buffer;

    assert_pixel_color(frame_buffer, 0, BLACK);
    assert_pixel_color(frame_buffer, 1, LIGHT_GRAY);
    assert_pixel_color(frame_buffer, 2, DARK_GRAY);
    assert_pixel_color(frame_buffer, 3, DARK_GRAY);
    assert_pixel_color(frame_buffer, 4, DARK_GRAY);
    assert_pixel_color(frame_buffer, 5, DARK_GRAY);
    assert_pixel_color(frame_buffer, 6, DARK_GRAY);
    assert_pixel_color(frame_buffer, 7, DARK_GRAY);
}

#[test]
fn should_render_sprite_with_white_background_if_background_and_window_enabled_is_cleared() {
    let mut emulator = initialize_emulator();

    write_tile_to_bg_memory(&mut emulator, 0, SAMPLE_TILE_A);
    write_tile_to_obj_memory(&mut emulator, 1, SAMPLE_TILE_B);
    
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
    emulator.gpu.registers.palette = 0b00011011;
    emulator.gpu.registers.obp0 = 0b00011011;
    emulator.gpu.registers.lcdc = 0b10000010;

    write_scanline(&mut emulator);

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_pixel_color(frame_buffer, 0, WHITE);
    assert_pixel_color(frame_buffer, 1, WHITE);
    assert_pixel_color(frame_buffer, 2, DARK_GRAY);
    assert_pixel_color(frame_buffer, 3, DARK_GRAY);
    assert_pixel_color(frame_buffer, 4, DARK_GRAY);
    assert_pixel_color(frame_buffer, 5, DARK_GRAY);
    assert_pixel_color(frame_buffer, 6, DARK_GRAY);
    assert_pixel_color(frame_buffer, 7, DARK_GRAY);
}

#[test]
fn should_render_tile_line_with_sprite_having_negative_y_pos() {
    let mut emulator = initialize_emulator();

    write_tile_to_bg_memory(&mut emulator, 0, SAMPLE_TILE_A);
    write_tile_to_obj_memory(&mut emulator, 1, SAMPLE_TILE_B);
    
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
    emulator.gpu.registers.palette = 0b00011011;
    emulator.gpu.registers.obp0 = 0b00011011;
    emulator.gpu.registers.lcdc = 0b10000011;

    write_scanline(&mut emulator);

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_pixel_color(frame_buffer, 0, BLACK);
    assert_pixel_color(frame_buffer, 1, LIGHT_GRAY);
    assert_pixel_color(frame_buffer, 2, WHITE);
    assert_pixel_color(frame_buffer, 3, WHITE);
    assert_pixel_color(frame_buffer, 4, WHITE);
    assert_pixel_color(frame_buffer, 5, WHITE);
    assert_pixel_color(frame_buffer, 6, LIGHT_GRAY);
    assert_pixel_color(frame_buffer, 7, DARK_GRAY);
}

#[test]
fn should_flip_sprite_on_x_axis() {
    let mut emulator = initialize_emulator();

    write_tile_to_bg_memory(&mut emulator, 0, SAMPLE_TILE_A);
    write_tile_to_obj_memory(&mut emulator, 1, SAMPLE_TILE_B);
    
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
    emulator.gpu.registers.palette = 0b00011011;
    emulator.gpu.registers.obp0 = 0b00011011;
    emulator.gpu.registers.lcdc = 0b10000011;

    write_scanline(&mut emulator);

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_pixel_color(frame_buffer, 0, BLACK);
    assert_pixel_color(frame_buffer, 1, LIGHT_GRAY);
    assert_pixel_color(frame_buffer, 2, WHITE);
    assert_pixel_color(frame_buffer, 3, WHITE);
    assert_pixel_color(frame_buffer, 4, DARK_GRAY);
    assert_pixel_color(frame_buffer, 5, WHITE);
    assert_pixel_color(frame_buffer, 6, LIGHT_GRAY);
    assert_pixel_color(frame_buffer, 7, BLACK); 
}

#[test]
fn should_flip_sprite_on_y_axis() {
    let mut emulator = initialize_emulator();

    write_tile_to_bg_memory(&mut emulator, 0, SAMPLE_TILE_A);
    write_tile_to_obj_memory(&mut emulator, 1, SAMPLE_TILE_B);
    
    let sprite = Sprite {
        y_pos: -2,
        x_pos: 2,
        tile_index: 1,
        priority: false,
        y_flip: true,
        x_flip: false,
        dmg_palette: false
    };

    let mut sprites = Vec::new();
    sprites.push(sprite);

    emulator.gpu.sprite_buffer = sprites;
    emulator.gpu.registers.ly = 0;
    emulator.gpu.registers.palette = 0b00011011;
    emulator.gpu.registers.obp0 = 0b00011011;
    emulator.gpu.registers.lcdc = 0b10000011;

    write_scanline(&mut emulator);

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_pixel_color(frame_buffer, 0, BLACK);
    assert_pixel_color(frame_buffer, 1, LIGHT_GRAY);
    assert_pixel_color(frame_buffer, 2, WHITE);
    assert_pixel_color(frame_buffer, 3, WHITE);
    assert_pixel_color(frame_buffer, 4, DARK_GRAY);
    assert_pixel_color(frame_buffer, 5, WHITE);
    assert_pixel_color(frame_buffer, 6, LIGHT_GRAY);
    assert_pixel_color(frame_buffer, 7, DARK_GRAY);
}

#[test]
fn should_render_eight_by_sixteen_sprite() {
    let mut emulator = initialize_emulator();

    write_tile_to_bg_memory(&mut emulator, 0, BLACK_TILE);
    write_tile_to_obj_memory(&mut emulator, 2, SAMPLE_TILE_A);
    write_tile_to_obj_memory(&mut emulator, 3, SAMPLE_TILE_B);
    
    let sprite = Sprite {
        y_pos: 0,
        x_pos: 2,
        tile_index: 3,
        priority: false,
        y_flip: false,
        x_flip: false,
        dmg_palette: false
    };

    let mut sprites = Vec::new();
    sprites.push(sprite);

    emulator.gpu.sprite_buffer = sprites;
    emulator.gpu.registers.ly = 0;
    emulator.gpu.registers.palette = 0b00011011;
    emulator.gpu.registers.obp0 = 0b00011011;
    emulator.gpu.registers.lcdc = 0b10000111;

    for _ in 0..9 {
        write_scanline(&mut emulator);
        emulator.gpu.registers.ly += 1;
    }

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_pixel_color(frame_buffer, 0, BLACK);
    assert_pixel_color(frame_buffer, 1, BLACK);
    assert_pixel_color(frame_buffer, 2, BLACK);
    assert_pixel_color(frame_buffer, 3, LIGHT_GRAY);
    assert_pixel_color(frame_buffer, 4, WHITE);
    assert_pixel_color(frame_buffer, 5, WHITE);
    assert_pixel_color(frame_buffer, 6, WHITE);
    assert_pixel_color(frame_buffer, 7, WHITE);

    assert_pixel_color(frame_buffer, line(8) + 0, BLACK);
    assert_pixel_color(frame_buffer, line(8) + 1, BLACK);
    assert_pixel_color(frame_buffer, line(8) + 2, DARK_GRAY);
    assert_pixel_color(frame_buffer, line(8) + 3, DARK_GRAY);
    assert_pixel_color(frame_buffer, line(8) + 4, DARK_GRAY);
    assert_pixel_color(frame_buffer, line(8) + 5, DARK_GRAY);
    assert_pixel_color(frame_buffer, line(8) + 6, DARK_GRAY);
    assert_pixel_color(frame_buffer, line(8) + 7, DARK_GRAY);
}

#[test]
fn should_prioritize_non_white_background_colors_when_sprite_priority_flag_set_to_true() {
    let mut emulator = initialize_emulator();

    write_tile_to_bg_memory(&mut emulator, 0, SAMPLE_TILE_A);
    write_tile_to_obj_memory(&mut emulator, 1, SAMPLE_TILE_B);
    
    let sprite = Sprite {
        y_pos: 0,
        x_pos: 2,
        tile_index: 1,
        priority: true,
        y_flip: false,
        x_flip: false,
        dmg_palette: false
    };

    let mut sprites = Vec::new();
    sprites.push(sprite);

    emulator.gpu.sprite_buffer = sprites;
    emulator.gpu.registers.ly = 0;
    emulator.gpu.registers.palette = 0b00011011;
    emulator.gpu.registers.obp0 = 0b00011011;
    emulator.gpu.registers.lcdc = 0b10000011;

    write_scanline(&mut emulator);

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_pixel_color(frame_buffer, 0, BLACK);
    assert_pixel_color(frame_buffer, 1, LIGHT_GRAY);
    assert_pixel_color(frame_buffer, 2, DARK_GRAY);
    assert_pixel_color(frame_buffer, 3, DARK_GRAY);
    assert_pixel_color(frame_buffer, 4, DARK_GRAY);
    assert_pixel_color(frame_buffer, 5, DARK_GRAY);
    assert_pixel_color(frame_buffer, 6, LIGHT_GRAY);
    assert_pixel_color(frame_buffer, 7, BLACK); 
}

#[test]
fn should_prioritize_background_colors_when_lcdc_bit_1_is_off() {
    let mut emulator = initialize_emulator();

    write_tile_to_bg_memory(&mut emulator, 0, SAMPLE_TILE_A);
    write_tile_to_obj_memory(&mut emulator, 1, SAMPLE_TILE_B);
    
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
    emulator.gpu.registers.palette = 0b00011011;
    emulator.gpu.registers.obp0 = 0b00011011;
    emulator.gpu.registers.lcdc = 0b10000001;

    write_scanline(&mut emulator);

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_pixel_color(frame_buffer, 0, BLACK);
    assert_pixel_color(frame_buffer, 1, LIGHT_GRAY);
    assert_pixel_color(frame_buffer, 2, WHITE);
    assert_pixel_color(frame_buffer, 3, WHITE);
    assert_pixel_color(frame_buffer, 4, WHITE);
    assert_pixel_color(frame_buffer, 5, WHITE);
    assert_pixel_color(frame_buffer, 6, LIGHT_GRAY);
    assert_pixel_color(frame_buffer, 7, BLACK);
}
